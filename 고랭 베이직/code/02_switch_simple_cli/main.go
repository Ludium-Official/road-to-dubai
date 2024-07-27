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

		switch input {
		case "exit":
			fmt.Println("Exiting...")
			return
		case "hello":
			fmt.Println("Hello, world!")
		case "even":
			fmt.Println("Even numbers from 0 to 10:")
			for i := 0; i <= 10; i++ {
				if i%2 == 0 {
					fmt.Println(i)
				}
			}
		case "odd":
			fmt.Println("Odd numbers from 1 to 10:")
			for i := 1; i <= 10; i++ {
				if i%2 != 0 {
					fmt.Println(i)
				}
			}
		case "help":
			fmt.Println("Available commands:")
			fmt.Println("exit - Exit the program")
			fmt.Println("hello - Print 'Hello, world!'")
			fmt.Println("even - Print even numbers from 0 to 10")
			fmt.Println("odd - Print odd numbers from 1 to 10")
			fmt.Println("help - Show this help message")
		default:
			fmt.Println("Unknown command")
		}
	}
}
