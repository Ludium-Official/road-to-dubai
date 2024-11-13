# 05m04. sync 패키지 사용해보기 - WaitGroup

## 목차
0. sync 패키지 사용해보기 - WaitGroup
1. 기본 설정하기
2. 코드 작성하기
3. WaitGroup 실행화면 제출 예시

## 0. sync 패키지 사용해보기 - WaitGroup
Go의 sync 패키지는 동시성 프로그래밍을 위한 여러 도구를 제공한다. 여기에는 WaitGroup, Mutex 등이 포함된다. WaitGroup은 여러 고루틴의 완료를 기다릴 때 사용한다. 이번 실습에서는 Go 언어의 `sync` 패키지를 사용하여 `WaitGroup`을 사용해보자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# wait_group 디렉토리 생성
$ mkdir wait_group && cd wait_group

# wait_group go module 생성 
$ go mod init wait_group
```

## 2. 코드 작성하기
전체 코드는 다음과 같다: 
```go
package main

import (
	"fmt"
	"sync"
	"time"
)

func worker(id int, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Printf("Worker %d starting\n", id)
	time.Sleep(time.Second)
	fmt.Printf("Worker %d done\n", id)
}

func main() {
	var wg sync.WaitGroup

	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go worker(i, &wg)
	}

	wg.Wait()
}
```
> 실습 코드 확인하기: [05_wait_group](../code/05_wait_group/)

## 3. WaitGroup 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_wait_group_result_example.png" alt="05_concurrency_wait_group_result_example" width="600"/>
</div>

