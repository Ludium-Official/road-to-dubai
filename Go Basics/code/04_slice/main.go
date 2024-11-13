
package main

import (
	"fmt"
)

func main() {
	// 슬라이스 선언
	var s1 []int
	fmt.Println(s1) // []

	// 슬라이스 초기화
	s2 := []int{1, 2, 3, 4, 5}
	fmt.Println(s2) // [1 2 3 4 5]

	// make 함수로 슬라이스 생성
	s3 := make([]int, 5)
	fmt.Println(s3) // [0 0 0 0 0]

	// 슬라이스의 요소 접근 및 수정
	fmt.Println(s2[0]) // 1
	s2[0] = 10
	fmt.Println(s2[0]) // 10

	// 슬라이스 부분 선택
	subSlice := s2[1:3]
	fmt.Println(subSlice) // [2 3]

	// 슬라이스의 길이와 용량
	fmt.Println(len(s2)) // 5
	fmt.Println(cap(s2)) // 5

	// append 함수로 요소 추가
	s2 = append(s2, 6)
	fmt.Println(s2) // [10 2 3 4 5 6]
}
