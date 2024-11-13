# 01-09. Try Pointer

## 0. Try Pointer
Let's practice how to use a pointer.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create pointer directory
$ mkdir pointer && cd pointer

# Create pointer go module
$ go mod init pointer
```

## 2. Creating code
### 1. Declare and initialize pointer variables
Declare the int-type variable a and the pointer variable ptr, and store the address value of a in the ptr pointer variable.
```go
package main

import (
	"fmt"
)

func main() {
	// Declare int type value a 
    var a int = 10
	// Declare point variable ptr 
    var ptr *int

    ptr = &a // Pointer variable ptr has the adddress of a
}
```
### 2. Look up and change values using pointers
Use a pointer to look up the value stored in address a, access the address, and change the value of the stored variable a.
```go
// Search variable a
fmt.Println(a) // 10

// Search ptr variable (= Address of variable a)
fmt.Println("Address of a:", ptr) // Address of a: 0xc000012028

// Search value of a using pointer  
fmt.Println("Value at the address stored in ptr:", *ptr) // Value at the address stored in ptr: 10

// Change the value in the address using pointer
*ptr = 20
fmt.Println("New value of a:", a) // New value of a: 20
```

The entire code is as follows:
```go
package main

import (
	"fmt"
)

func main() {
    var a int = 10
    var ptr *int

    ptr = &a // Pointer vlaue ptr has address a

    fmt.Println(a) // 10
    fmt.Println("Address of a:", ptr) // Address of a: 0xc000012028
    fmt.Println("Value at the address stored in ptr:", *ptr) // Value at the address stored in ptr: 10

    // Change the value in the address using pointer
    *ptr = 20
    fmt.Println("New value of a:", a) // New value of a: 20
}
```
> Check the practice code: [01_pointer](../code/01_pointer/)

## 3. Example of submission of Pointer program execution screen
The results printed by running the program are as follows:

<div style="text-align: center;">
   <img src="../assets/01_basic_pointer_result_example.png" alt="01_basic_pointer_result_example" width="600"/>
</div>
