package main

import (
	"fmt"
)

func main() {
	var b1 bool = true
	var b2 bool = false

	fmt.Println(b1) // true
	fmt.Println(b2) // false

	// bool 타입의 기본값 
	var b3 bool
	fmt.Println(b3) // false

	// 비교 연산자 
	a := 10
	b := 20
	fmt.Println(a == b) // false
	fmt.Println(a != b) // true
	fmt.Println(a < b)  // true
	fmt.Println(a > b)  // false
	fmt.Println(a <= b) // true
	fmt.Println(a >= b) // false

	// 논리 연산자 
	c := true
	d := false
	fmt.Println(c && d) // false
	fmt.Println(c || d) // true
	fmt.Println(!c)     // false
}