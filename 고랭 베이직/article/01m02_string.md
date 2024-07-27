
# 01m02. String 사용해보기

## 목차
0. String 사용해보기
1. 기본 설정하기
2. main 함수 작성하기 
3. String 프로그램 실행화면 제출 예시

## 0. String 사용해보기
실습으로 String 타입을 사용해보도록 하자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# string 디렉토리 생성
$ mkdir string && cd string

# string go module 생성 
$ go mod init string
```

## 2. main 함수 작성하기
이제 String 타입을 사용할 main 함수 코드를 작성하도록 하자. main 함수에서 실행할 코드는 다음과 같다:
```go
package main

import (
	"fmt"
)

func main() {
	var str string = "Hello, Cosmos"
	fmt.Println(str) // Hello, Cosmos

	// len 함수는 문자 수가 아닌 문자열의 바이트 수를 반환한다. 
	fmt.Println(len(str)) // 13
	
    fmt.Println(str[0]) // 72 (ASCII value of 'H')

    // range로 문자열을 반복하여 유니코드 코드 포인트를 가져와서 출력한다.
	for index, runeValue := range str { 
        // index: 문자열에서 rune의 시작 바이트 위치이다.
        // runeValue: 해당 위치에 있는 문자의 유니코드 코드 포인트이다.
		fmt.Printf("%d: %c\n", index, runeValue)
	}
	// 0: H
	// 1: e
	// 2: l
	// 3: l
	// 4: o
	// 5: ,
	// 6:  
	// 7: C
    // 8: o
    // 9: s
    // 10: m
    // 11: o
    // 12: s

	// 문자열 concatenation
	str1 := "Hello, "
	str2 := "World!"
	combined := str1 + str2
	fmt.Println(combined) // Hello, World!

	// Substring (slicing 사용)
	substr := str[7:13]
	fmt.Println(substr) // Cosmos

	// 문자열을 byte 배열로 변환하기 
	byteSlice := []byte(str)
	byteSlice[0] = 'h'
	newStr := string(byteSlice)
	fmt.Println(newStr) // hello, Cosmos
}
```
> 실습 코드 확인하기: [01_string](../code/01_string/)

## 3. string 프로그램 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/01_basic_string_result_example.png" alt="string_result_example" width="600"/>
</div>

