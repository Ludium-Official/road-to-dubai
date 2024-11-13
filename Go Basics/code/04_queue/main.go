
package main

import (
	"fmt"
)

type Queue []int

// 큐에 요소 추가
func (q *Queue) Enqueue(val int) {
	*q = append(*q, val)
}

// 큐에서 요소 제거
func (q *Queue) Dequeue() int {
	if len(*q) == 0 {
		return -1 // 큐가 비었을 경우
	}
	val := (*q)[0]
	*q = (*q)[1:]
	return val
}

func main() {
	q := Queue{}
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)

	fmt.Println(q.Dequeue()) // 1
	fmt.Println(q.Dequeue()) // 2
	fmt.Println(q.Dequeue()) // 3
    fmt.Println(q.Dequeue()) // -1
}
