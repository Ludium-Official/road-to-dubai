package main

import (
	"fmt"
)

func main() {
	// 맵 선언
	var m1 map[string]int
	fmt.Println(m1) // map[]

	// 맵 초기화
	m2 := map[string]int{"a": 1, "b": 2}
	fmt.Println(m2) // map[a:1 b:2]

	// make 함수로 맵 생성
	m3 := make(map[string]int)
	fmt.Println(m3) // map[]

	// 맵에 값 추가 및 수정
	m3["a"] = 1
	fmt.Println(m3) // map[a:1]
	m3["a"] = 10
	fmt.Println(m3) // map[a:10]

	// 값 접근
	val := m3["a"]
	fmt.Println(val) // 10

	// 존재하지 않는 키 접근
	val2 := m3["b"]
	fmt.Println(val2) // 0

	// 값 삭제
	delete(m3, "a")
	fmt.Println(m3) // map[]
}
