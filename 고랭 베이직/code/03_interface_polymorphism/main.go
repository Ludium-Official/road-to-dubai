package main

import (
	"fmt"
)

// 1. Speaker 인터페이스 정의
type Speaker interface {
	Speak() string
}

// 2. Person 타입 정의 및 Speaker 인터페이스 구현
type Person struct {
	name string
}

func (p Person) Speak() string {
	return "Hello, my name is " + p.name
}

// 3. Dog 타입 정의 및 Speaker 인터페이스 구현
type Dog struct {
	name string
}

func (d Dog) Speak() string {
	return "Woof! My name is " + d.name
}

// 4. Greet 함수 정의
func Greet(s Speaker) {
	fmt.Println(s.Speak())
}

// 5. main 함수
func main() {
	// Person 타입의 인스턴스를 생성하고 Greet 함수에 전달
	p := Person{name: "Alice"}
	Greet(p) // Hello, my name is Alice

	// Dog 타입의 인스턴스를 생성하고 Greet 함수에 전달
	d := Dog{name: "Buddy"}
	Greet(d) // Woof! My name is Buddy
}
