# 04m00. Array 사용해보기

## 목차 
0. Array 사용해보기
1. 기본 설정하기
2. 코드 작성하기
3. Array 실행화면 제출하기

## 0. Array 사용해보기
이번 실습에서는 Array를 직접 선언하고 초기화해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# array 디렉토리 생성
$ mkdir array && cd array

# array go module 생성 
$ go mod init array
```

## 2. 코드 작성하기
전체 코드는 다음과 같다: 
```go
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
```
> 실습 코드 확인하기: [04_array](../code/04_array/)


## 3. Array 실행화면 제출하기
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/04_data_structure_array_result_example.png" alt="04_data_structure_array_result_example" width="600"/>
</div>

