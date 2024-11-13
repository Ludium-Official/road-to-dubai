
package main

import (
	"fmt"
)

func main() {
	// 배열 선언
	var arr1 [5]int
	fmt.Println(arr1) // [0 0 0 0 0]

	// 배열 초기화
	arr2 := [5]int{1, 2, 3, 4, 5}
	fmt.Println(arr2) // [1 2 3 4 5]

	// 부분 초기화
	arr3 := [5]int{1, 2}
	fmt.Println(arr3) // [1 2 0 0 0]

	// 배열의 요소 접근
	fmt.Println(arr2[0]) // 1
	arr2[0] = 10
	fmt.Println(arr2[0]) // 10
}
