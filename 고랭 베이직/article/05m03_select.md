# 05m03. select문 사용해보기

## 목차
0. select문 사용해보기
1. 기본 설정하기
2. 코드 작성하기
3. select문 실행화면 제출 예시

## 0. select문 사용해보기
select 문은 여러 채널 작업을 기다리고, 그 중 하나가 준비되면 해당 작업을 실행한다. 이는 다중 채널 동작을 제어하는 데 유용하다. select 문에서 default 케이스를 사용하면 모든 채널이 준비되지 않은 경우에도 즉시 실행된다. 실습을 통해 select 문을 사용하는 방법을 익혀보자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# select 디렉토리 생성
$ mkdir select && cd select

# select go module 생성 
$ go mod init select
```

## 2. 코드 작성하기
전체 코드는 다음과 같다:
```go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int, 1)
	ch <- 1
	select {
	case val := <-ch:
		fmt.Println("received", val)
	default:
		fmt.Println("no value received")
	}
}
```
> 실습 코드 확인하기: [05_select](../code/05_select/)

## 3. select문 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_select_result_example.png" alt="05_concurrency_select_result_example" width="600"/>
</div>

