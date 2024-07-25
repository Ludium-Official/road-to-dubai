package main

import (
	"fmt"
)

func main() {
	// 버퍼 크기 2인 채널 생성
	ch := make(chan int, 2)

	// 채널에 값 보내기
	ch <- 1
	ch <- 2

	// 채널에서 값 받기
	fmt.Println(<-ch) // 1
	fmt.Println(<-ch) // 2
}
