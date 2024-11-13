# 01-03. Try String Formatting

## 0. Try String Formatting
Let's use the String Formatting function as a practice.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create string_formatting directory
$ mkdir string_formatting && cd string_formatting

# Create string_formatting go module 
$ go mod init string_formatting
```

## 2. Creating a main function
Now let's write the main function code that will use the String Formatting function. The code to execute in the main function is as follows:
```go
package main

import (
	"fmt"
)

func main() {
	intVar := 123
	floatVar := 123.456
	strVar := "Hello, World!"
	boolVar := true
	pointerVar := &intVar

	// %v
	fmt.Printf("Default format: %v\n", intVar)         // 123
	fmt.Printf("Default format: %v\n", floatVar)       // 123.456
	fmt.Printf("Default format: %v\n", strVar)         // Hello, World!
	fmt.Printf("Default format: %v\n", boolVar)        // true
	fmt.Printf("Default format: %v\n", pointerVar)     // 0xc0000a6010

	// %T
	fmt.Printf("Type: %T\n", intVar)                   // int
	fmt.Printf("Type: %T\n", floatVar)                 // float64
	fmt.Printf("Type: %T\n", strVar)                   // string
	fmt.Printf("Type: %T\n", boolVar)                  // bool
	fmt.Printf("Type: %T\n", pointerVar)               // *int

	// %x
	fmt.Printf("Hexadecimal: %x\n", intVar)            // 7b
	fmt.Printf("Hexadecimal: %x\n", strVar)            // 48656c6c6f2c20576f726c6421

	// %d 
	fmt.Printf("Integer: %d\n", intVar)                // 123

	// %f, %e, %E 
	fmt.Printf("Float: %f\n", floatVar)                // 123.456000
	fmt.Printf("Scientific (lowercase): %e\n", floatVar) // 1.234560e+02
	fmt.Printf("Scientific (uppercase): %E\n", floatVar) // 1.234560E+02

	// %s 
	fmt.Printf("String: %s\n", strVar)                 // Hello, World!

	// %p
	fmt.Printf("Pointer address: %p\n", pointerVar)    // 0xc0000a6010
}
```
> Check the practice code: [01_string_formattting](../code/01_string_formattting/)

## 3. Example of submitting a string formatting program execution screen
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/01_basic_string_formatting_result_example.png" alt="string_formatting_result_example" width="600"/>
</div>
