package main

import (
	"fmt"
)

type Person struct {
    name string
    age  int
}

func main() {
    p := Person{name: "Alice", age: 30}
    fmt.Println(p.name) // Alice
    fmt.Println(p.age)  // 30

    // 필드 접근 및 수정하기
    p.age = 31
    fmt.Println(p.age)  // 31
}