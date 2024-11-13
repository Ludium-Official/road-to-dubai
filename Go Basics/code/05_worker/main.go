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
