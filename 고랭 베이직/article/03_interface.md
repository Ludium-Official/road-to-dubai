# 003. Go Interface
> 이 아티클에서는 인터페이스의 개념과 이를 통한 다형성의 구현 방법을 설명한다. 인터페이스를 사용하여 다양한 타입의 객체를 동일한 방법으로 처리하는 방법을 학습하고, 이를 통해 코드의 유연성과 확장성을 높이는 방법을 실습한다.

## 목차
0. Interface란 무엇인가?
1. Interface와 polymorphism(다형성)
   1. (실습) Interface를 통해 다형성 구현하기
2. Empty Interface


## 0. Interface란 무엇인가?
Interface(이하 인터페이스)는 메서드 시그니처의 집합을 정의하며, 어떤 타입이 해당 인터페이스를 구현하는지 여부는 해당 타입이 인터페이스의 모든 메서드를 구현했는지에 따라 결정된다. 인터페이스 특징은 다음과 같다:
- 암시적 구현: type은 메서드를 구현함으로써 인터페이스를 자동으로 충족한다.
- 다형성: 인터페이스는 함수가 필요한 메서드를 구현하기만 하면 다른 타입을 받아들이고 반환할 수 있게 해준다.


이러한 유연성과 단순성 덕분에 Go의 인터페이스 시스템은 강력하고 사용하기 쉬우며, 다형성과 분리를 통해 깔끔하고 유지 관리가 용이한 코드를 만들 수 있다.

## 1. Interface와 polymorphism(다형성)
다형성(Polymorphism)은 객체 지향 프로그래밍 및 함수형 프로그래밍에서 중요한 개념으로, 여러 다른 데이터 타입들을 동일한 방식으로 다룰 수 있게 하는 기능을 의미한다. 다형성은 코드를 더 유연하고 재사용 가능하게 만들어 준다. 이러한 기능은 인터페이스를 활용하여 구현할 수 있다.

### 1. (실습) Interface를 통해 다형성 구현하기
이번 실습에서는 `Speaker` 인터페이스를 활용하여 다양한 타입의 객체가 동일한 메서드를 구현하도록 하고, 이를 통해 다형성을 구현해보자. 

#### 1. 기본 설정하기
기본 설정은 다음과 같다:
```sh
# interface_polymorphism 디렉토리 생성
$ mkdir interface_polymorphism && cd interface_polymorphism

# interface_polymorphism go module 생성 
$ go mod init interface_polymorphism
```

#### 2. 코드 작성하기
다음은 실습을 위한 boilerpalte 코드이다.
```go
package main

type Speaker interface {
	Speak() string
}

func main() {
	
}
```

프로그램 구현 요구 사항은 다음과 같다:
1. Person 타입을 정의하고, 이 타입이 Speaker 인터페이스를 구현하도록 한다.
2. Dog 타입을 정의하고, 이 타입이 Speaker 인터페이스를 구현하도록 한다.
3. Greet 함수를 정의한다. 이 함수는 Speaker 인터페이스를 인수로 받아서 Speak 메서드를 호출하고 결과를 출력한다.
4. main 함수에서 Person과 Dog 타입의 인스턴스를 생성하고, Greet 함수에 전달하여 각각의 인사를 출력한다.
> 구현된 실습 코드 확인하기: [03_interface_polymorphism](../code/03_interface_polymorphism/)


#### 3. Interface를 통해 다형성을 구현한 프로그램 실행화면 제출하기
프로그램을 실행하여 출력된 예시 결과는 다음과 같다:
<div style="text-align: center;">
   <img src="../assets/03_interface_result_example.png" alt="03_interface_result_example" width="600"/>
</div>


## 2. Empty Interface
empty interface(빈 인터페이스)는 어떠한 타입도 담을 수 있는 컨테이너이다. 이를 사용하면 어떠한 타입도 변수에 할당할 수 있으므로, 일반적으로 제네릭, 임의의 데이터 저장, 다양한 타입을 처리해야 할 때 유용하다. 빈 인터페이스는 매우 유연하지만, 사용하는 곳에서는 해당 값의 타입을 알 수 없기 때문에, 값을 사용할 때 type assertion이나 type switch를 통해 원래의 타입을 확인해야 한다. 

아래 예제는 빈 인터페이스를 사용하여 다양한 타입의 값을 저장하고, type switch를 사용하여 해당 값의 타입을 식별하는 방법을 보여준다:
```go
package main

import (
	"fmt"
)

// 정의된 타입 구조체
type Person struct {
	Name string
	Age  int
}

type Animal struct {
	Species string
	Age     int
}

// 입력된 값을 구조체와 비교하는 함수
func compareStructs(i interface{}) {
	switch v := i.(type) {
	case Person:
		fmt.Printf("Value is a Person: %+v\n", v)
	case Animal:
		fmt.Printf("Value is an Animal: %+v\n", v)
	default:
		fmt.Printf("Unknown type: %T\n", v)
	}
}

func main() {
	// 다양한 타입의 값을 담을 수 있는 빈 인터페이스 슬라이스
	var i interface{}

	// Person 구조체와 비교
	i = Person{Name: "Alice", Age: 30}
	compareStructs(i) // Value is a Person: {Name: Alice Age: 30}

	// Animal 구조체와 비교
	i = Animal{Species: "Dog", Age: 5}
	compareStructs(i) // Value is an Animal: {Species: Dog Age: 5}

	// 기타 타입과 비교
	i = "Hello"
	compareStructs(i) // Unknown type: string
}
```


## Resources 
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec
