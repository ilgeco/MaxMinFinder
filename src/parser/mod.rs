use nom::{error::ErrorKind, number::complete, sequence::preceded, IResult};

#[derive(Debug)]
pub struct FloatIter<'a> {
    input: &'a str,
}

impl<'a> FloatIter<'a> {
    pub fn new(input: &'a str) -> Self {
        return Self { input };
    }
}

impl<'a> Iterator for FloatIter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = parse(self.input);
        match res {
            Ok(ok_res) => {
                self.input = ok_res.0;
                return Some(ok_res.1);
            }
            Err(_) => None,
        }
    }
}

fn parse(mut input: &str) -> IResult<&str, f64> {
    loop {
        match preceded::<_, _, _, nom::error::Error<&str>, _, _>(
            nom::combinator::opt(nom::bytes::complete::is_not("-0123456789.")),
            complete::double,
        )(input)
        {
            ret @ Ok(_) => return ret,
            Err(err_code) => match err_code {
                ret @ nom::Err::Incomplete(_) => return Err(ret),
                nom::Err::Error(err_internal) => {
                    if err_internal.code == ErrorKind::Float && err_internal.input != "" {
                        input = nom::bytes::complete::is_a::<_, _, nom::error::Error<&str>>(
                            "-0123456789.",
                        )(err_internal.input)
                        .unwrap()
                        .0;
                    } else {
                        return Err(nom::Err::Error(err_internal));
                    }
                }
                ret @ nom::Err::Failure(_) => return Err(ret),
            },
        };
    }
}

#[cfg(test)]
mod parser_test {

    use super::*;

    #[test]
    fn parser_simple_test() {
        let input = "3.34";
        assert_eq!(parse(input), Ok(("", 3.34_f64)));

        let input = "3.34 4.568";
        assert_eq!(parse(input), Ok((" 4.568", 3.34_f64)));

        let input = "casa 3.34";
        assert_eq!(parse(input), Ok(("", 3.34_f64)));

        let input = "ca\nsa .34";
        assert_eq!(parse(input), Ok(("", 0.34_f64)));

        let input = "casa 3.34 wasa";
        assert_eq!(parse(input), Ok((" wasa", 3.34_f64)));

        let input = " 3.34";
        assert_eq!(parse(input), Ok(("", 3.34_f64)));

        let input = "-11 coco 33";
        assert_eq!(parse(input), Ok((" coco 33", -11_f64)));

        let input = "3.34.6689";
        assert_eq!(parse(input), Ok((".6689", 3.34_f64)));

        let input = "not numbers here";
        let e_code = Err(nom::Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Float,
        )));
        assert_eq!(parse(input), e_code);

        let input = "not numbers here.\n but here yes 3.15 and 8.9";
        assert_eq!(parse(input), Ok((" and 8.9", 3.15_f64)));
    }

    #[test]
    fn multiple_number_parsing() {
        let input = "not numbers here.\n but here yes 3.15 -and- 8.9";
        let res = parse(input);
        assert_eq!(res, Ok((" -and- 8.9", 3.15_f64)));
        let input = res.unwrap().0;
        let res = parse(input);
        assert_eq!(res, Ok(("", 8.9_f64)));
    }


    #[test]
    fn iterator_test() {
        let input = "8 9 -10.33 not numbers here.\n but here yes 3.15 -and- 8.9 ......23 .45.36 ora no ";
        let mut iter = FloatIter::new(input);
        assert_eq!(iter.next(), Some(8_f64));
        assert_eq!(iter.next(), Some(9_f64));
        assert_eq!(iter.next(), Some(-10.33_f64));
        assert_eq!(iter.next(), Some(3.15_f64));
        assert_eq!(iter.next(), Some(8.9_f64));
        assert_eq!(iter.next(), Some(0.45_f64));
        assert_eq!(iter.next(), Some(0.36_f64));
        assert_eq!(iter.next(), None);
    }



}
