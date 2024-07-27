# 05m02. channel을 이용한 동시 작업 패턴 사용해보기

## 목차
0. channel을 이용한 동시 작업 패턴 사용해보기
1. 기본 설정하기
2. 코드 작성하기
3. channel을 이용한 동시 작업 패턴 실행화면 제출 예시

## 0. channel을 이용한 동시 작업 패턴 사용해보기
채널을 사용하여 여러 고루틴에 작업을 분배할 수 있다. 작업자 패턴은 채널을 사용하여 여러 고루틴에 작업을 분배하는 방법이다. 여러 작업자 고루틴이 작업을 분배받아 동시에 처리할 수 있도록 한다. 실습을 통해 작업자 패턴을 사용하는 방법을 익혀보자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# worker 디렉토리 생성
$ mkdir worker && cd worker

# worker go module 생성 
$ go mod init worker
```

## 2. 코드 작성하기
다음과 같이 작업자 패턴을 사용하는 코드를 작성해보자:
```go
package main

import (
	"fmt"
	"time"
)

// 작업자 함수
func worker(id int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Printf("worker %d started job %d\n", id, j)
		time.Sleep(time.Second)
		fmt.Printf("worker %d finished job %d\n", id, j)
		results <- j * 2
	}
}

func main() {
	const numJobs = 5
	jobs := make(chan int, numJobs)
	results := make(chan int, numJobs)

	// 3개의 작업자 고루틴 생성
	for w := 1; w <= 3; w++ {
		go worker(w, jobs, results)
	}

	// 작업 채널에 작업 보내기
	for j := 1; j <= numJobs; j++ {
		jobs <- j
	}
	close(jobs)

	// 결과 받기
	for a := 1; a <= numJobs; a++ {
		fmt.Printf("result: %d\n", <-results)
	}
}
```
> 실습 코드 확인하기: [05_worker](../code/05_worker/)

## 3. channel을 이용한 동시 작업 패턴 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_worker_result_example.png" alt="05_concurrency_worker_result_example" width="600"/>
</div>

