 **Provide an easy way to find the minimum/maximum number from File/ClipBoard/Stdin**

### Compilation

>```cargo build --relase```


 ### Usages:

 * **File**          
      >``` [min | max] -F <file_name> ```

 * **Stdin**        
     >``` echo "3 4 5" | [min | max] ```

 * **ClipBoard**        
     >``` [min | max] ```

### Output

* **max**

    Display the maximum number finded

* **min**

    Display the minimum number finded

 ### **Return codes**

 * &nbsp;0 -&nbsp;&nbsp; *SUCCESS*

 * -1 -&nbsp;&nbsp; *ERROR*