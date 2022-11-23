![GitHub](https://img.shields.io/github/license/ilgeco/MaxMinFinder?style=plastic)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ilgeco/MaxMinFinder/Rust?style=plastic)
![GitHub top language](https://img.shields.io/github/languages/top/ilgeco/MaxMinFinder?style=plastic)

**Provide an easy way to find the minimum/maximum number from File/ClipBoard/Stdin**
[Wiki](https://ilgeco.github.io/MaxMinFinder/max/)

### Compilation

  

```bash
cargo build --relase
```

  
  

### Usages:

  

*  **File**

	``` [min | max] -F <file_name> ```

  

*  **Stdin**

	``` echo "3 4 5" | [min | max] ```

  

*  **ClipBoard**

	``` [min | max] ```

  

### Output

  

*  **max**

  

Display the maximum number finded

  

*  **min**

  

Display the minimum number finded

  

### **Return codes**

  

*  &nbsp;0 -&nbsp;&nbsp;  *SUCCESS*

  

* -1 -&nbsp;&nbsp;  *ERROR*
