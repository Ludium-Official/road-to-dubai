package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("Enter command: ")
		scanner.Scan()
		input := strings.TrimSpace(scanner.Text())

		if input == "exit" {
			fmt.Println("Exiting...")
			break
		} else if input == "hello" {
			fmt.Println("Hello, world!")
		} else if input == "even" {
			fmt.Println("Even numbers from 0 to 10:")
			for i := 0; i <= 10; i++ {
				if i%2 == 0 {
					fmt.Println(i)
				}
			}
		} else if input == "odd" {
			fmt.Println("Odd numbers from 1 to 10:")
			for i := 1; i <= 10; i++ {
				if i%2 != 0 {
					fmt.Println(i)
				}
			}
		} else {
			fmt.Println("Unknown command")
		}
	}
}
