# 01m05. Fucntion 사용해보기

## 목차
0. Fucntion 사용해보기
1. 기본 설정하기
2. main 함수 작성하기 
3. Fucntion 프로그램 실행화면 제출 예시


## 0. Function 사용해보기 
실습으로 Function을 직접 정의하여 사용해보도록 하자. 

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# function 디렉토리 생성
$ mkdir function && cd function

# function go module 생성 
$ go mod init function
```

## 2. main 함수 작성하기
이제 function을 직접 정의하고 사용할 코드를 작성하도록 하자. 매개변수와 반환값이 없는 함수 `greet()` 함수와 두 개의 정수를 매개변수로 받아, 그 합을 반환하는 `add(a int, b int)` 함수를 구현해보자.

전체 코드는 다음과 같다:
```go
package main

import (
	"fmt"
)

// 매개변수와 반환값이 없는 함수
func greet() {
	fmt.Println("Hello, World!")
}

// 매개변수와 반환값이 있는 함수 
func add(a int, b int) int {
	return a + b
}


func main() {
	// 함수 호출 
	greet()

	result := add(3, 4)
	fmt.Println("Sum:", result) // 7
}
```
> 실습 코드 확인하기: [01_function](../code/01_function/)

## 3. Function 프로그램 실행화면 제출하기 
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/01_basic_function_result_example.png" alt="01_basic_function_result_example" width="600"/>
</div>

