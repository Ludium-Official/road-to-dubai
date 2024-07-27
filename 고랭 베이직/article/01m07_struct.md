# 01m07. Struct 사용해보기

## 목차
0. Struct 사용해보기
1. 기본 설정하기
2. 코드 작성하기
   1. Person 구조체 정의하기
   2. Person 구조체의 인스턴스 생성하기 
   3. 생성한 Person 인스턴스 필드 값 조회 및 수정하기
3. Struct 프로그램 실행화면 제출 예시

## 0. Struct 사용해보기
실습으로 구조체를 직접 정의하여 사용해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# struct 디렉토리 생성
$ mkdir struct && cd struct

# struct go module 생성 
$ go mod init struct
```

## 2. 코드 작성하기
이제 구조체를 직접 정의하고 사용할 코드를 다음 순서에 맞춰서 작성해보도록 하자.

### 1. Person 구조체 정의하기
- name: 문자열 타입의 필드
- age: 정수 타입의 필드
```go
type Person struct {
    name string
    age  int
}
```

### 2. Person 구조체의 인스턴스 생성하기 
```go
func main() {
    p := Person{name: "Alice", age: 30}
}
```

### 3. (2번에서) 생성한 Person 인스턴스 필드 값 조회 및 수정하기
인스턴스로 생성된 객체의 name, age에 접근해보고 age 값을 직접 수정해보도록 하자.
```go
func main() {
    p := Person{name: "Alice", age: 30}
	// 필드 값 조회하기
    fmt.Println(p.name) // Alice
    fmt.Println(p.age)  // 30

    // 필드 수정하기
    p.age = 31
    fmt.Println(p.age)  // 31
}
```

전체 코드는 다음과 같다:
```go
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
	// 필드 값 조회하기
    fmt.Println(p.name) // Alice
    fmt.Println(p.age)  // 30

    // 필드 수정하기
    p.age = 31
    fmt.Println(p.age)  // 31
}
```
> 실습 코드 확인하기: [01_struct](../code/01_struct/)

## 3. Struct 프로그램 실행화면 제출 예시 
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/01_basic_struct_result_example.png" alt="01_basic_struct_result_example" width="600"/>
</div>


 