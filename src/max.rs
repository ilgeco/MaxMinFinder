//! **Provide an easy way to find the maximum number from File/ClipBoard/Stdin**
//!
//! ## Usages:
//!
//! * **File**          
//!      ``` max -F <file_name> ```
//!
//! * **Stdin**        
//!     ``` echo "3 4 5" | max ```
//!
//! * **ClipBoard**        
//!     ``` max ```
//!
//! ## **Return codes**
//!
//! * &nbsp;0 -&nbsp;&nbsp; *SUCCESS*
//!
//! * -1 -&nbsp;&nbsp; *ERROR*
//!    

#![deny(missing_docs)]
#![deny(warnings)]

use std::{
    cmp::Ordering,
    fs::{self},
    io::{self, Read, ErrorKind},
    path::Path,
    process::exit,
};

use clap::Parser;
use regex::Regex;
use terminal_clipboard::get_string;

/// Provide an easy way to find the minimum number from File/ClipBoard/Stdin
/// See doc for full guide
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File name
    #[clap(short, long)]
    file: Option<String>,
}

/// Retrive the content of a file in a particular Path
///
/// ## Return Value
///     Content of the file
///
/// ## Exception
///
///     If file cannot be opened exit with -1
///
fn retrive_file(file_name: impl AsRef<Path>) -> String {
    match fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "{}",
                format!("File {} not found", file_name.as_ref().to_str().unwrap())
            );
            exit(-1)
        }
    }
}

/// Retrive the content from the ClipBoard
///
/// ## Return Value
///     Content of the file
///
/// ## Exception
///
///     If file cannot be opened exit with -1
///
fn retrive_clip() -> String {
    match get_string() {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Clipboard fail");
            exit(-1)
        }
    }
}

/// Retrive a String from stdin
///
/// ## Return Value
///
///     * Ok(String) if stdin != ""
///     * Error otherwise
fn retrive_stdin() -> io::Result<String> {
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().lock().read_to_string(&mut buffer)?;

        Ok(buffer)
    } else {
        Err(io::Error::new(ErrorKind::Other, "oh no!"))
    }
}

fn main() {
    let number_re = Regex::new(r"(?:-)?[0-9]+([.,][0-9]+)?").unwrap();
    let args = Args::parse();
    let retrived = match args.file {
        Some(x) => retrive_file(x),
        None => match retrive_stdin() {
            Ok(x) => x,
            Err(_) => retrive_clip(),
        },
    };

    let max = number_re
        .captures_iter(&retrived)
        .map(|x| x[0].parse::<f64>().unwrap())
        .max_by(|&x, &y| {
            if x > y {
                return Ordering::Greater;
            } else if y > x {
                return Ordering::Less;
            }
            return Ordering::Equal;
        });
    match max {
        Some(x) => {
            println!("{}", x);
        }
        None => {}
    }
}