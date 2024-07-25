package main

import (
	"fmt"
)

func main() {
	var input int
	fmt.Print("Enter a number: ")
	fmt.Scanf("%d", &input)

	if input < 0 {
		fmt.Println("The number is negative")
	} else if input == 0 {
		fmt.Println("The number is zero")
	} else if input > 0 && input <= 10 {
		fmt.Println("The number is between 1 and 10")
	} else {
		fmt.Println("The number is greater than 10")
	}
}
