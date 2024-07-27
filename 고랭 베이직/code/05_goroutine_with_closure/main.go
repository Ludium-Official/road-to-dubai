package main

import (
	"fmt"
)

func main() {
	done := make(chan bool)

	for i := 0; i < 5; i++ {
		go func(num int) {
			fmt.Printf("%d ", num) // 예상한 대로 출력됨
			done <- true
		}(i)
	}

	for i := 0; i < 5; i++ {
		<-done
	}
}
