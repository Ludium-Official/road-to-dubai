package main

import (
	"fmt"
)

func main() {
	// 채널 생성
	ch := make(chan int)

	// 고루틴에서 채널에 값 보내기
	go func() {
		ch <- 42
	}()

	// 채널에서 값 받기
	val := <-ch
	fmt.Println(val) // 42
}
