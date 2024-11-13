package main

import (
	"fmt"
)

// 매개변수와 반환값이 없는 함수
func greet() {
	fmt.Println("Hello, World!")
}

// 매개변수와 반환값이 있는 함수 
func add(a int, b int) int {
	return a + b
}


func main() {
	// 함수 호출 
	greet()

	result := add(3, 4)
	fmt.Println("Sum:", result) // 7
}