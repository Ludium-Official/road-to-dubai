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
