package main

import (
	"fmt"
)

func main() {
	done := make(chan bool)

	for i := 0; i < 5; i++ {
		go func() {
            fmt.Printf("%d ", i) // 예상과 다르게 출력될 수 있음
			done <- true
		}()
	}

	for i := 0; i < 5; i++ {
		<-done
	}
}
