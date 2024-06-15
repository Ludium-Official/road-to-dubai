# 002. Go Control Structure
> 이 아티클에서는 Go 언어의 제어 구조에 대해 다룬다. 조건문, 반복문, 그리고 switch 문을 통해 프로그램의 흐름을 제어하는 방법을 설명한다.


## 목차 
1. 조건문: if, else if, else
2. 반복문: for, break, continue
3. switch 문


## 1. 조건문: if, else if, else
Go에서 조건문은 if, else if, else 키워드를 사용하여 정의한다. 조건문은 특정 조건에 따라 코드 블록을 실행하거나 건너뛸 수 있게 한다.

### 기본 if 문
조건이 참이면 코드 블록을 실행한다.
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

### if-else 문
조건이 참이면 첫 번째 코드 블록을, 거짓이면 두 번째 코드 블록을 실행한다.
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

### if-else if-else 문
여러 조건을 순차적으로 검사한다. 첫 번째 참인 조건의 코드 블록을 실행하고, 모든 조건이 거짓이면 마지막 else 블록을 실행한다.
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

### 조건문 내 변수 선언
if 문 내에서 변수 선언이 가능하다. 이 변수는 if 문 블록 내에서만 유효하다.
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

### 기본 for 문
초기화, 조건, 후처리를 포함한 기본 형태이다.
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

### 조건식만 있는 for 문
이는 초기화와 후처리가 없이 조건식만 있는 형태도 가능하다. 
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

### 무한 루프
조건 없이 계속 반복한다. 

```go
package main

import (
	"fmt"
)

func main() {
	for {
		fmt.Println("Infinite loop")
		break // Infinite loop를 방지하기 위해 추가
	}
}
```

### range를 사용한 for 문
배열, 슬라이스, 맵, 채널 등을 순회할 때 사용한다.

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

### break와 continue
break는 반복문을 종료하고, continue는 다음 반복으로 건너뛴다.
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
switch 문은 하나 이상의 조건을 순서대로 평가하여, 일치하는 조건의 코드 블록을 실행한다. case 절에 해당하지 않으면 default 절을 실행한다.

### 기본 switch 문
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

### 여러 값을 검사하는 switch 문
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

### 조건식을 사용하는 switch 문
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

### fallthrough
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

fallthrough는 오직 다음 case 절로만 실행을 강제로 넘길 수 있다. 즉, 현재 case 절의 끝에만 사용될 수 있으며, 다중 fallthrough는 허용되지 않는다. 이를 사용할 경우, 다음 case 절의 조건은 검사되지 않으므로 논리적 오류를 방지하기 위해 신중하게 사용해야 한다.


# Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
