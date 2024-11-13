# 05m00. 고루틴(goroutine) 사용해보기

## 목차
0. 고루틴(goroutine) 사용해보기
1. 기본 설정하기
2. 코드 작성하기 
3. 고루틴(goroutine) 실행화면 제출 예시

## 0. 고루틴(goroutine) 사용해보기
실습을 통해 고루틴을 사용해보자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# goroutine 디렉토리 생성
$ mkdir goroutine && cd goroutine

# goroutine go module 생성 
$ go mod init goroutine
```

## 2. 코드 작성하기 
고루틴은 `go` 키워드를 사용하여 생성하며, 함수를 호출할 때 사용한다. 다음과 같이 say 함수를 고루틴으로 실행하여 "hello"와 "world"를 동시에 출력하는 코드를 작성해보자:
```go
package main

import (
	"fmt"
	"time"
)

func say(s string) {
	for i := 0; i < 5; i++ {
		fmt.Println(s)
		time.Sleep(100 * time.Millisecond)
	}
}

func main() {
	go say("world")
	say("hello")
}
```
> 실습 코드 확인하기: [05_goroutine](../code/05_goroutine/)


## 3. 고루틴(goroutine) 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_goroutine_result_example.png" alt="05_concurrency_goroutine_result_example" width="600"/>
</div>


