# 01-08. Try the Method

## 0. Try Method
Let's try to define and use the method directly as a practice.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create method directory
$ mkdir method && cd method

# Cretae method go module 
$ go mod init method
```

## 2. Creating code
Now let's define the structure directly and use the methods used with it.

### 1. Define Rectangle structures
- width: field of float64 type
- height: field of floor64 type
```go
type Rectangle struct {
    width, height float64
}
```

### 2. Add the area method to the Rectangle structure
The area method calculates the area of the Rectangle and returns it.
```go
func (r Rectangle) area() float64 {
    return r.width * r.height
}
```

### 3. Add scale method to Rectangle structure
The scale method enlarges or reduces the width and height of the Rectangle to a given factor.
```go
func (r *Rectangle) scale(factor float64) {
    r.width *= factor
    r.height *= factor
}
```

### 4. Creating a main function
Create an instance of the Rectangle structure from the main function, and call the area and scale methods.
```go
func main() {
    rect := Rectangle{width: 3, height: 4}
    fmt.Println("Area:", rect.area()) // Area: 12

    rect.scale(2)
    fmt.Println("New dimensions:", rect.width, rect.height) // New dimensions: 6 8
    fmt.Println("New area:", rect.area()) // New area: 48
}
```

The entire code is as follows:
```go
package main

import (
	"fmt"
)

type Rectangle struct {
    width, height float64
}

func (r Rectangle) area() float64 {
    return r.width * r.height
}

func (r *Rectangle) scale(factor float64) {
    r.width *= factor
    r.height *= factor
}

func main() {
    rect := Rectangle{width: 3, height: 4}
    fmt.Println("Area:", rect.area()) // Area: 12

    rect.scale(2)
    fmt.Println("New dimensions:", rect.width, rect.height) // New dimensions: 6 8
    fmt.Println("New area:", rect.area()) // New area: 48
}
```
> Check the practice code: [01_method](../code/01_method/)

## 3. Example of submitting a method program execution screen
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/01_basic_method_result_example.png" alt="01_basic_method_result_example" width="600"/>
</div>



