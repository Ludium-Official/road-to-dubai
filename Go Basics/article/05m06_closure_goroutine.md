# 05m06. 클로저(closure)를 활용하여 고루틴 사용해보기

## 목차
0. 클로저(closure)를 활용하여 고루틴 사용해보기
1. 기본 설정
2. 클로저 기능을 사용하여 고루틴 프로그램 작성하기
3. 클로저 고루틴 실행화면 제출 예시

## 0. 클로저(closure)를 활용하여 고루틴 사용해보기
[01_basic의 closure 파트](./01_basic.md#8-closure)에서 알아본 클로저는 고루틴과 채널을 사용할 때 매우 중요하다. 클로저는 함수 내에서 외부 변수에 접근할 수 있도록 하여 고루틴이 상태를 유지하거나 공유 상태를 안전하게 변경할 수 있게 한다.

고루틴 내에서 클로저를 사용할 때, 클로저가 외부 변수의 참조를 캡처한다는 점을 주의해야 한다. 이는 의도치 않은 동작을 초래할 수 있다:
```go
package main

import (
	"fmt"
)

func main() {
	done := make(chan bool)

	for i := 0; i < 5; i++ {
		go func() {
            fmt.Printf("%d ", i) // 예상과 다르게 출력될 수 있음
			done <- true
		}()
	}

	for i := 0; i < 5; i++ {
		<-done
	}
}
```
> 예제 코드 확인하기: [05_goroutine_no_closure](../code/05_goroutine_no_closure/)

예제에서는 i 변수가 고루틴 내에서 클로저로 캡처된다. 정상적이라면 0~4의 숫자가 임의로 출력되어야 한다. 그러나 실제 출력은 다음과 같다: 
```sh
5 5 5 5 5
```

다음은 이를 실제로 여러 번 실행에 해본 결과이다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_goroutine_no_closure_result_example.png" alt="05_concurrency_goroutine_no_closure_result_example" width="600"/>
</div>

이는 고루틴이 실제로 실행되는 시점이 문제를 일으키게 된다.
1. 메인 스레드에 있는 for 루프가 빠르게 실행된다.
2. 고루틴은 생성되자마자 실행되는 것이 아니라, 약간의 지연이 발생할 수 있다.
3. 고루틴은 클로저를 통해 i의 현재 값을 참조하지만, 이 값은 고루틴이 실행될 때의 i 값이다.

그래서 위의 예시에서는 i가 5가 되어 for 루프가 종료될 때까지 실제로 고루틴이  실행되지 않은 것이다. 이러한 버그는 공유되는 변수에 따라 예측할 수 없는 이상한 영향을 미칠 수 있다. 특히나 대부분 결정론적이어야 하는 블록체인 시스템 환경에서는 더 중요하게 다뤄져야 한다. 이러한 문제를 피하기 위해, 반복문 내에서 고루틴을 생성할 때 변수의 값을 명시적으로 캡처해야 한다. 실습을 통해 클로저를 활용한 고루틴을 작성해보도록 하자.

## 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# goroutine_with_closure 디렉토리 생성
$ mkdir goroutine_with_closure && cd goroutine_with_closure

# goroutine_with_closure go module 생성 
$ go mod init goroutine_with_closure
```

## 2. 클로저 기능을 사용하여 고루틴 프로그램 작성하기
다음과 같이 고루틴 함수에 `num int`을 인자로 받아서 사용하도록 수정하자:
```go
package main

import (
	"fmt"
)

func main() {
	done := make(chan bool)

	for i := 0; i < 5; i++ {
		go func(num int) {
			fmt.Printf("%d ", num) // 예상한 대로 출력됨
			done <- true
		}(i)
	}

	for i := 0; i < 5; i++ {
		<-done
	}
}
```
> 실습 코드 확인하기: [05_goroutine_with_closure](../code/05_goroutine_with_closure/)

이러면 예상한대로 0~4의 숫자가 임의로 출력되는 것을 확인할 수 있다. 

## 3. 클로저 고루틴 실행화면 제출 예시
프로그램을 실행하여 출력된 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/05_concurrency_goroutine_with_closure_result_example.png" alt="05_concurrency_goroutine_with_closure_result_example" width="600"/>
</div>

