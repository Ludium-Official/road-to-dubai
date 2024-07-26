package main

import (
	"fmt"
)

func main() {
    var a int = 10
    var ptr *int

    ptr = &a // ptr 포인터 변수는 a의 주소값을 가지고 있다

    fmt.Println(a) // 10
    fmt.Println("Address of a:", ptr) // Address of a: 0xc000012028
    fmt.Println("Value at the address stored in ptr:", *ptr) // Value at the address stored in ptr: 10

    // pointer를 사용하여 해당 주소에 들어있는 값을 변경한다
    *ptr = 20
    fmt.Println("New value of a:", a) // New value of a: 20
}
