# 005. Go Concurrency 
> 이 아티클에서는 Go 언어의 동시성 프로그래밍을 다룬다. 고루틴(goroutine)과 채널(channel), select 문, sync 패키지 등을 사용하여 동시성을 제어하는 방법을 설명한다.

## 목차
1. 고루틴(goroutine)과 채널(channel)
2. 채널을 이용한 동시성 제어
3. select 문
4. sync 패키지 사용

## 1. 고루틴(goroutine)과 채널(channel)
### 고루틴(goroutine)
고루틴은 Go에서 경량 스레드를 생성하는 방법이다. go 키워드를 사용하여 함수 호출을 고루틴으로 실행할 수 있다. 고루틴은 매우 가볍고, 수천 개의 고루틴을 생성해도 성능에 큰 영향을 주지 않는다.

### Goroutine Example
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

### 채널(channel)
채널은 고루틴 간의 통신을 위한 도구이다. 채널을 사용하면 고루틴 간에 데이터를 주고받을 수 있다. 채널은 타입을 지정하여 선언하며, make 함수를 사용하여 생성한다.

### Channel Example
```go
package main

import (
	"fmt"
)

func main() {
	// 채널 생성
	ch := make(chan int)

	// 고루틴에서 채널에 값 보내기
	go func() {
		ch <- 42
	}()

	// 채널에서 값 받기
	val := <-ch
	fmt.Println(val) // 42
}
```

### 채널의 주요 특성
- 동기화: 채널을 통해 값을 주고받을 때 고루틴 간에 동기화가 이루어집니다.
- 방향성: 채널은 양방향이거나 송신 전용, 수신 전용으로 선언할 수 있습니다.

### 송신 전용 및 수신 전용 채널
```go
package main

import (
	"fmt"
)

func send(ch chan<- int, val int) {
	ch <- val
}

func receive(ch <-chan int) int {
	return <-ch
}

func main() {
	ch := make(chan int)

	go send(ch, 42)
	val := receive(ch)
	fmt.Println(val) // 42
}
```

## 2. 채널을 이용한 동시성 제어
### 버퍼링된 채널
버퍼링된 채널은 정해진 크기의 버퍼를 가지며, 버퍼가 가득 차지 않은 한 송신은 블록되지 않습니다. 버퍼 크기를 지정하여 생성할 수 있다.

### 버퍼링된 채널 Example
```go
package main

import (
	"fmt"
)

func main() {
	// 버퍼 크기 2인 채널 생성
	ch := make(chan int, 2)

	// 채널에 값 보내기
	ch <- 1
	ch <- 2

	// 채널에서 값 받기
	fmt.Println(<-ch) // 1
	fmt.Println(<-ch) // 2
}
```

### 채널을 이용한 작업 분배
채널을 사용하여 여러 고루틴에 작업을 분배할 수 있습니다. 예를 들어, 작업자 패턴(worker pattern)을 구현할 수 있다.

### 작업자 패턴 Example 
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

## 3. select 문
select 문은 여러 채널 작업을 기다리고, 그 중 하나가 준비되면 해당 작업을 실행한다. 이는 다중 채널 동작을 제어하는 데 유용하다.

### select 문 Example 
```go
package main

import (
	"fmt"
	"time"
)

func main() {
	ch1 := make(chan string)
	ch2 := make(chan string)

	go func() {
		time.Sleep(1 * time.Second)
		ch1 <- "one"
	}()

	go func() {
		time.Sleep(2 * time.Second)
		ch2 <- "two"
	}()

	for i := 0; i < 2; i++ {
		select {
		case msg1 := <-ch1:
			fmt.Println("received", msg1)
		case msg2 := <-ch2:
			fmt.Println("received", msg2)
		}
	}
}
```

### default 케이스
select 문에서 default 케이스를 사용하면 모든 채널이 준비되지 않은 경우에도 즉시 실행된다.

### default 케이스 Example
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

## 4. sync 패키지 사용
Go의 sync 패키지는 동시성 프로그래밍을 위한 여러 도구를 제공한다. 여기에는 WaitGroup, Mutex, Once 등이 포함된다.

### WaitGroup
WaitGroup은 여러 고루틴의 완료를 기다릴 때 사용한다.

### WaitGroup Example 
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

### Mutex
Mutex는 임계 구역을 보호하여 동시 접근을 제어한다.

### Mutex Example 
```go
package main

import (
	"fmt"
	"sync"
)

type SafeCounter struct {
	mu sync.Mutex
	v  map[string]int
}

func (c *SafeCounter) Inc(key string) {
	c.mu.Lock()
	c.v[key]++
	c.mu.Unlock()
}

func (c *SafeCounter) Value(key string) int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.v[key]
}

func main() {
	c := SafeCounter{v: make(map[string]int)}
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c.Inc("somekey")
		}()
	}

	wg.Wait()
	fmt.Println(c.Value("somekey")) // 1000
}
```

### Once
Once는 한 번만 실행되는 코드를 보장한다.

### Once Example
```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	var once sync.Once
	onceBody := func() {
		fmt.Println("Once executed")
	}

	for i := 0; i < 3; i++ {
		go once.Do(onceBody)
	}

	// Wait for goroutines to finish
	var wg sync.WaitGroup
	wg.Add(3)
	wg.Wait()
}
```


# Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
