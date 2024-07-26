package main

import (
	"fmt"
)

type Stack []int

// 스택에 요소 추가
func (s *Stack) Push(val int) {
	*s = append(*s, val)
}

// 스택에서 요소 제거
func (s *Stack) Pop() int {
	if len(*s) == 0 {
		return -1 // 스택이 비었을 경우
	}
	val := (*s)[len(*s)-1]
	*s = (*s)[:len(*s)-1]
	return val
}

func main() {
	s := Stack{}
	s.Push(1)
	s.Push(2)
	s.Push(3)

	fmt.Println(s.Pop()) // 3
	fmt.Println(s.Pop()) // 2
	fmt.Println(s.Pop()) // 1
    fmt.Println(s.Pop()) // -1
}
