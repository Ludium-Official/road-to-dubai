# 002. Go Control Structure
> 이 아티클에서는 Go 언어의 제어 구조에 대해 다룬다. 조건문(if, else), 반복문(for), 그리고 switch문과 같은 제어 구조를 통해 프로그램의 흐름을 제어하는 방법을 설명한다. 각 제어 구조의 문법과 사용 예시를 제공하고, 이를 활용한 실습 과제를 통해 실력을 향상을 목표로 한다.


## 목차 
1. 조건문: if, else if, else
   1. 기본 if 문
   2. if-else 문
   3. if-else if-else 문
   4. 조건문 내 변수 선언
2. 반복문: for, break, continue
   1. 기본 for 문
   2. 조건식만 있는 for 문
   3. 무한 루프: 정지 판별 문제
   4. range를 사용한 for 문
   5. break와 continue
3. switch 문
   1. 기본 switch 문
   2. 여러 값을 검사하는 switch 문
   3. 조건식을 사용하는 switch 문
   4. fallthrough

## 1. 조건문: if, else if, else
Go에서 조건문은 if, else if, else 키워드를 사용하여 정의한다. 조건문은 특정 조건에 따라 코드 블록을 실행하거나 건너뛸 수 있게 한다.

### 1. 기본 if 문
조건이 참이면 코드 블록을 실행한다. 다음은 x가 5보다 큰지 검사하는 방법을 보여준다:
```go
package main

import (
	"fmt"
)

func main() {
	x := 10
	if x > 5 {
		fmt.Println("x is greater than 5")
	}
}
```

### 2. if-else 문
조건이 참이면 첫 번째 코드 블록을, 거짓이면 두 번째 코드 블록을 실행한다. 다음은 x가 5보다 큰지 검사하고 마냑 크지 않다면 else 블록을 실행하는 방법을 보여준다: 
```go
package main

import (
	"fmt"
)

func main() {
	x := 3
	if x > 5 {
		fmt.Println("x is greater than 5")
	} else {
		fmt.Println("x is not greater than 5")
	}
}
```

### 3. if-else if-else 문
여러 조건을 순차적으로 검사한다. 첫 번째 참인 조건의 코드 블록을 실행하고, 모든 조건이 거짓이면 마지막 else 블록을 실행한다. 다음은 else if문을 추가하여 조건 분기를 더 세밀하게 할 수 있는 방법을 보여준다:
```go
package main

import (
	"fmt"
)

func main() {
	x := 8
	if x < 3 {
		fmt.Println("x is less than 3")
	} else if x < 7 {
		fmt.Println("x is less than 7 but greater than or equal to 3")
	} else {
		fmt.Println("x is greater than or equal to 7")
	}
}
```

### 4. 조건문 내 변수 선언
if 문 내에서 변수 선언이 가능하다. 이 변수는 if 문 블록 내에서만 유효하다. 다음은 조건문 내에서 x를 10으로 초기화하고, x > 5 조건을 검사하는 방법을 보여준다:
```go
package main

import (
	"fmt"
)

func main() {
	if x := 10; x > 5 {
        // x 존재 
		fmt.Println("x is greater than 5")
	}
    // x 존재하지 않음
}
```

## 2. 반복문: for, break, continue
Go에서는 유일한 반복문으로 for 문을 사용한다. 다양한 방식으로 반복문을 사용할 수 있다.

### 1. 기본 for 문
초기화, 조건, 후처리를 포함한 기본 형태이다. 다음은 i를 0으로 초기화하고, i가 5보다 작은 동안 반복하며, 각 반복 후 i를 증가시켜 i의 값을 출력하는 예시 코드이다: 
```go
package main

import (
	"fmt"
)

func main() {
	for i := 0; i < 5; i++ {
		fmt.Println(i)
	}
}
```

### 2. 조건식만 있는 for 문
이는 초기화와 후처리가 없이 조건식만 있는 형태도 가능하다. 다음은 위와 같은 기능을 하지만 조건식만 있는 형태로 구현한 예시 코드이다:
```go
package main

import (
	"fmt"
)

func main() {
	i := 0
	for i < 5 {
		fmt.Println(i)
		i++
	}
}
```

### 3. 무한 루프
반복문에 조건을 명시하지 않으면 계속해서 반복하여 무한히 실행되는 함수를 구현할 수 있다. 무한 루프 개념은 블록체인에서는 자주 등장한다. 왜냐하면 이러한 함수가 악의적인 사용자에 의해 블록체인 네트워크에서 실행된다면 Liveness에 큰 영향을 끼치게 되기 때문이다. 이는 네트워크의 다른 트랜잭션을 방해하고 전체 시스템의 성능을 저하시킬 수 있다. 이 문제를 방지하기 위해 프로그램 가능하게 설계한 EVM(Ethereum Virtual Machine)과 같은 블록체인 시스템에서는 gas의 개념을 도입하였다. 만약 무한 루프인지 아닌지 판별 할 수 있는 머신이 존재했다면 이더리움 gas의 개념이 존재하지 않았을 것이라고 가볍게 생각해볼 수 있다. 

#### 튜링 정지 문제 
이는 유명한 튜링 정지 문제에서 비롯된다. 다음은 튜링의 정지 문제에 대한 간단한 예제를 Go 언어로 작성한 것이다. 이 예제는 정지 여부를 판별하려는 시도를 보여준다. 실제로는 어떠한 함수가 무한으로 동작하는지에 대해 정확히 판별할 수 있는 코드가 존재하지 않으니 가볍게 참고만 하도록 하자: 
```go
// 이 함수는 주어진 프로그램이 특정 입력에 대해 정지하는지 여부를 판별하는 가상의 함수이다.
// 실제로는 이와 같은 함수가 모든 경우에 대해 정확히 작동할 수 없음을 나타내기 위해 항상 false를 반환하도록 구현되어 있다.
func halts(program func(), input int) bool {
	// 이러한 함수는 존재하지 않는다.
	return false 
}

func problematicFunction() {
	for {
		// Infinite loop
		fmt.Println("This function never halts.")
	}
}

func main() {
	// problematicFunction(); // 무한 실행
	input := 42 // Example input
	if halts(problematicFunction, input) {
		fmt.Println("The program halts.")
	} else {
		fmt.Println("The program does not halt.")
	}
}
```
- 왜 정지 문제를 판별할 수 없을까? 이에 대한 자세한 설명으로는 [SNUON_컴퓨터과학이 여는 세계_2.4 튜링 기계의 급소: 튜링기계 하나는 자연수 하나_이광근](https://www.youtube.com/watch?si=5aWY7S7tfIaplFF7&t=453&v=RINdVaoXV5c&feature=youtu.be) 영상을 참고하면 좋다. 
> 예제 코드 확인하기: [02_halting_problem](../code/02_halting_problem/)

### 4. range를 사용한 for 문
배열, 슬라이스, 맵, 채널 등을 순회할 때 사용한다. 다음은 `nums` 슬라이스의 각 요소에 대해 인덱스 i와 값 num을 반복하는 예시 코드이다:
```go
package main

import (
	"fmt"
)

func main() {
	nums := []int{2, 3, 4}
	for i, num := range nums {
		fmt.Printf("Index: %d, Value: %d\n", i, num)
	}

	m := map[string]string{"a": "apple", "b": "banana"}
	for k, v := range m {
		fmt.Printf("Key: %s, Value: %s\n", k, v)
	}
}
```

### 5. break와 continue
break는 반복문을 종료하고, continue는 다음 반복으로 건너뛴다. 다음은 이 두 기능을 활용한 예시 코드이다:
```go
package main

import (
	"fmt"
)

func main() {
	for i := 0; i < 10; i++ {
		if i == 5 {
			break
		}
		if i%2 == 0 {
			continue
		}
		fmt.Println(i)
	}
}
```


## 3. switch 문
Go에서 switch 문은 하나 이상의 조건을 순서대로 평가하여, 일치하는 조건의 코드 블록을 실행하는 구조이다. 각 case 절에 해당하지 않으면 default 절을 실행한다. switch 문을 잘 활용하면 코드 가독성을 높이고 여러 조건을 깔끔하게 처리할 수 있다.

### 1. 기본 switch 문
기본 switch 문은 하나의 변수 값을 여러 case 절과 비교하여 일치하는 경우에 해당 코드 블록을 실행한다. 예시 코드는 다음과 같다:
```go
package main

import (
	"fmt"
)

func main() {
	x := 2
	switch x {
	case 1:
		fmt.Println("One")
	case 2:
		fmt.Println("Two")
	case 3:
		fmt.Println("Three")
	default:
		fmt.Println("Other")
	}
}
```
- `switch x`: x의 값을 평가한다.
- `case 1`, `case 2`, `case 3`: x의 값이 각각 1, 2, 3과 일치할 때 실행할 코드 블록을 정의한다.
- `default`: 어떤 case 절에도 해당하지 않을 때 실행할 코드 블록을 정의한다.

### 2. 여러 값을 검사하는 switch 문
여러 값을 검사하는 switch 문에서는 각 case 절에 여러 값을 포함할 수 있다. 이는 동일한 코드 블록을 여러 값에 대해 실행할 때 유용하다. 다음은 switch 문을 통해 1이상 6이하의 홀수와 짝수를 구분하는 프로그램 예시를 보여준다:
```go
package main

import (
	"fmt"
)

func main() {
	x := 4
	switch x {
	case 1, 3, 5:
		fmt.Println("Odd")
	case 2, 4, 6:
		fmt.Println("Even")
	default:
		fmt.Println("Other")
	}
}
```
- `case 1, 3, 5`: x의 값이 1, 3, 5 중 하나일 때 "Odd"를 출력한다.
- `case 2, 4, 6`: x의 값이 2, 4, 6 중 하나일 때 "Even"을 출력한다.
- `default`: x의 값이 나열된 값에 해당하지 않을 때 "Other"를 출력한다.

### 3. 조건식을 사용하는 switch 문
조건식을 사용하는 switch 문에서는 각 case 절에 조건식을 사용하여 조건을 평가할 수 있다. 이 방식은 특정 값뿐만 아니라 보다 복잡한 조건을 처리할 때 유용하다. 
```go
package main

import (
	"fmt"
)

func main() {
	x := 10
	switch {
	case x < 5:
		fmt.Println("x is less than 5")
	case x < 10:
		fmt.Println("x is less than 10 but greater than or equal to 5")
	default:
		fmt.Println("x is 10 or more")
	}
}
```
- `switch`: switch 키워드 뒤에 아무것도 명시하지 않아도 됩니다. 각 case 절에서 직접 조건을 평가한다.
- `case x < 5`: x가 5보다 작으면 "x is less than 5"를 출력한다.
- `case x < 10`: x가 10보다 작고 5 이상이면 "x is less than 10 but greater than or equal to 5"를 출력한다.
- `default`: x가 10 이상이면 "x is 10 or more"를 출력한다.

### 4. fallthrough
fallthrough 키워드는 Go의 switch 문에서 사용되는 특별한 키워드로, 현재 case 절이 실행된 후에 다음 case 절을 강제로 실행하도록 한다. 즉, fallthrough 키워드를 사용하면 조건을 검사하지 않고 다음 case 절의 코드를 실행하게 된다.
```go
package main

import (
	"fmt"
)

func main() {
	x := 1
	switch x {
	case 1:
		fmt.Println("One")
		fallthrough
	case 2:
		fmt.Println("Two")
		fallthrough
	case 3:
		fmt.Println("Three")
	default:
		fmt.Println("Other")
	}
	// One 
	// Two
	// Three
}
```
- `case 1`: x가 1인 경우 "One"을 출력한 후 fallthrough 키워드에 의해 다음 case 절(즉, case 2)을 실행한다.
- `case 2`: 조건을 검사하지 않고 "Two"를 출력한 후 다시 fallthrough 키워드에 의해 다음 case 절(즉, case 3)을 실행한다.
- `case 3`: 조건을 검사하지 않고 "Three"를 출력한다.
- `default`: fallthrough에 의해 다음 case 절이 없을 때 실행된다. 위 코드에서는 실행되지 않는다.

fallthrough는 오직 다음 case 절로만 실행을 강제로 넘길 수 있다. 즉, 현재 case 절의 끝에만 사용될 수 있으며, 다중 fallthrough는 허용되지 않는다. 이를 사용할 경우, 다음 case 절의 조건은 검사되지 않으므로 논리적 오류를 방지하기 위해 신중하게 사용해야 한다.


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
