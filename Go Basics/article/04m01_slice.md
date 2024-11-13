# 04-01. Try Slice

## 0. Try Slice
In this practice, let's declare Slice ourselves and reset it.

## 1. Setting Preferences
The default settings are as follows:
```sh
# Create slice directory
$ mkdir slice && cd slice

# Create slice go module
$ go mod init slice
```

## 2. Creating code
The full code is as follows:
```go
package main

import (
	"fmt"
)

func main() {
	// Declare slice
	var s1 []int
	fmt.Println(s1) // []

	// Initialize slice
	s2 := []int{1, 2, 3, 4, 5}
	fmt.Println(s2) // [1 2 3 4 5]

	// Create slice with make function
	s3 := make([]int, 5)
	fmt.Println(s3) // [0 0 0 0 0]

	// Partial access to slice and edit
	fmt.Println(s2[0]) // 1
	s2[0] = 10
	fmt.Println(s2[0]) // 10

	// Partially select the slice
	subSlice := s2[1:3]
	fmt.Println(subSlice) // [2 3]

	// Slice length and capacity
	fmt.Println(len(s2)) // 5
	fmt.Println(cap(s2)) // 5

	// Add an element with append function 
	s2 = append(s2, 6)
	fmt.Println(s2) // [10 2 3 4 5 6]
}
```
> Check the practice code: [04_slice](../code/04_slice/)

## 3. Example of submitting Slice execution screen
The results printed by running the program are as follows:
<div style="text-align: center;">
   <img src="../assets/04_data_structure_slice_result_example.png" alt="04_data_structure_slice_result_example" width="600"/>
</div>
