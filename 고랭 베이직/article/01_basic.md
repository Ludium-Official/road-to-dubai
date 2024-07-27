
# 001. Go Basic 
> 이 아티클에서는 Golang의 기본적인 데이터 타입(Numerics, String, Booleans)과 변수 선언 및 초기화 방법을 설명한다. 그리고나서 실질적인 프로그래밍에 필요한 함수 정의 및 호출, 구조체와 메서드, 포인터의 활용, 그리고 클로저에 대해 학습한다. 각 개념마다 실습 과제를 통해 직접 코드를 작성해보며 이해를 돕는다.

## 목차
0. Numeircs
   1. Integer
   2. Float & Complex 
1. String
2. String Formatting 
3. Booleans
4. 변수 선언 및 초기화하기  
   1. 초기화 없이 여러 변수 선언하기
   2. 상수(Constant) 정의하기
5. Function
6. Struct
   1. 객체 지향 프로그래밍의 Class와 Go의 Struct 비교
7. Method
   1. 객체 지향 프로그래밍의 Class와 Go의 Method 비교
8. Pointer 
9. Closure

## 0. Numerics
integer, floating-point, complex, rune 상수를 통칭하여 numeric 상수라고 한다.

### 1. Integer
go는 여러 Interger 타입을 제공한다.

#### general types
- `int`: OS(32비트 또는 64비트)에 따라 크기가 달라진다. 양수와 음수를 모두 나타낼 수 있다.
- `uint`: 부호가 없는 정수 유형으로 음수가 아닌 양의 정수만 나타낼 수 있다.


#### specific types
specific 타입은 비트 길이를 명시적으로 지정할 수 있다. 
- `int8`: -128 ~ 127
- `int16`: -32768 ~ 32767
- `int32`(= `rune`): -2147483648 ~ 2147483647
- `int64`: -9223372036854775808 ~ 9223372036854775807

다음은 음수가 아닌 양의 정수만 나타내는 명시적 타입들이다. 
- `uint8`(= `byte`): 0 ~ 255
- `uint16`: 0 ~ 65535
- `uint32`: 0 ~ 4294967295
- `uint64`: 0 ~ 18446744073709551615

#### special types
- `byte`: 일반적으로 원시 바이너리 데이터를 나타내는 데 사용되는 uint8의 별칭이다. 
- `rune`: 유니코드 코드 포인트를 나타내는 데 사용되는 int32의 별칭이다.
- `uintptr`: "모든 포인터의 비트 패턴을 보유하는" 정수이다. 시스템 콜을 통해 OS와 직접 소통하는 경우에 가끔 사용되는데, 거의 사용할 일이 없다. ([참고](https://stackoverflow.com/questions/59042646/whats-the-difference-between-uint-and-uintptr-in-golang))


### 2. Float & Complex 
go는 부동소수점과 복소수를 표현하기 위한 여러 타입을 제공한다. 이러한 타입은 수식 계산 및 공간 데이터 처리를 비롯한 다양한 프로그램 동작에 사용된다.

#### float types
Go는 두 가지 부동 소수점 유형을 제공한다:
- `float32`: 32비트 부동 소수점 숫자를 나타낸다.
- `float64`: 64비트 부동 소수점 숫자를 나타낸다.

정밀도가 유한하기 때문에 부동 소수점 유형은 실수의 대략적인 표현만 제공한다.

#### complex types
- `complex64`: 두 개의 float32 값(실수 부분과 허수 부분)으로 구성된다.
- `complex128`: 두 개의 float64 값(실수 부분과 허수 부분)으로 구성된다.

complex 사용법은 go에서 제공하는 내장 함수를 사용하여 다룰 수 있다:
- complex(r, i): 실수 부분 r과 허수 부분 i로 복소수를 생성한다.
- real(c): 복소수 c의 실수 부분을 반환한다.
- imag(c): 복소수 c의 허수 부분을 반환한다.


부동 소수점 타입은 float32 및 float64으로 나타낸다. 이는 정밀도가 유한하기 때문에 실수에 대한 근사치일 뿐이다. complex64 및 complex128은 복소수를 나타낸다. 이들은 지리공간 좌표계와 과학 응용 분야 등에서 유용하다. 복소수에는 항상 부동 소수점인 '실수' 부분과 '허수' 부분이 있다. 
- 실수 부분과 허수 부분이 float32인 경우 복소수는 complex64가 된다. 
- 마찬가지로 실수 부분과 허수 부분이 float64이면 복소수는 complex128이 된다.


## 1. String
Go에서 `string`은 기본적으로 UTF8로 인코딩되는 읽기 전용 바이트 시퀀스이며 불변(immutable)이다. 즉, 다양한 방법으로 문자열을 읽고 조작할 수는 있지만 문자열은 불변이므로 문자열을 구성하는 바이트는 직접 수정할 수 없다:
- UTF-8 인코딩: 문자열은 UTF-8로 인코딩되므로 모든 유효한 유니코드 문자를 나타낼 수 있다.
- 불변성: 일단 생성된 문자열의 내용은 변경할 수 없습니다. 문자열을 수정하는 모든 작업은 새 문자열을 생성한다.
- 길이 및 인덱싱: len 함수는 문자 수가 아닌 문자열의 바이트 수를 반환합니다. 문자열을 색인화하면 문자가 아닌 해당 위치의 바이트가 반환된다. 


## 2. String Formatting 
fmt 패키지는 다양한 방식으로 문자열의 포맷을 지정할 수 있는 `fmt.Printf`와 같은 함수를 제공한다. 

| verb  |    description                 | 
|-------|--------------------------------|
| %v    | value (default format)         |
| %T    | 해당 value의 type                |
| %x    | Hexadecimal encoding           |
| %d    | Integer (base 10)              |
| %f    | Floating-point number          |
| %e    | Scientific notation (lowercase)|
| %E    | Scientific notation (uppercase)|
| %p    | Pointer 주소값                   |
| %s    | String                         |
| %c    | Unicode 코드 포인트로 표시되는 문자   |


## 3. Booleans
Go에서 `bool` 타입은 말 그대로 boolean 값을 나타낸다. 이는 `true` 혹은 `false`로 표현이 가능하다. 

Go에서 bool 타입의 주요 특성은 다음과 같다:
- 기본값: bool 타입 변수의 기본값은 false이다. 
- 비교 연산자: bool 타입과 함께 ==, !=, <, >, <=, >=와 같은 비교 연산자의 결과로 표현된다.
- 논리 연산자: bool 타입과 함께 &&(AND), ||(OR), ! (NOT) 등의 논리 연산으로 표현된다.


## 4. 변수 선언 및 초기화하기  
go에서는 여러 가지 방법으로 변수를 정의하고 초기화할 수 있다. 다음은 변수를 정의하는 다양한 방법이다:
```go
var s string = "initial"

// 또는 := 연산자와 함께 shorthand 표기법을 사용할 수도 있다
s := "initial"
```

### 1. 초기화 없이 여러 변수 선언하기
var 키워드를 사용하여 초기화하지 않고 여러 변수를 선언할 수도 있다:
```go
var (
    a, b int
    s string
    c complex64
)

// 이는 각 변수를 개별적으로 선언하는 것과 같다. 
// 초기화하지 않으면 변수는 타입에 따라 0이라는 값을 갖는다.
var a, b int
var s string
var c complex64
```

### 2. 상수(Constant) 정의하기
상수를 정의하려면 const 키워드를 사용해야 한다:
```go
const hello = "Hello, World!"
```

## 5. Function
Function(이하 함수)는 Go에서 핵심적인 부분이다. 
- 함수를 사용하면 코드를 재사용 가능한 블록으로 캡슐화할 수 있다. 
- 함수는 0개 이상의 매개변수와 0개 이상의 반환값을 받을 수 있다. 

함수를 정의하고 사용하는 방법은 다음과 같다:
```go
func functionName(parameterName parameterType) returnType {
    // function body
    return value
}
```


## 5. Struct
Struct(이하 구조체)는 하나의 이름으로 변수를 그룹화하는 복합 데이터 타입이다. 이러한 변수를 필드라고 한다. 구조체는 객체 지향 프로그래밍의 클래스와 유사하지만 상속을 지원하지 않는다.

구조체를 정의하고 사용하는 방법은 다음과 같다:
```go
type StructName struct {
    field1 fieldType1
    field2 fieldType2
    // more fields...
}
```
 
### 2. 객체 지향 프로그래밍의 Class와 Go의 Struct 비교
Class는 기존 객체 지향 프로그래밍 언어에 필수적으로 들어가는 기능이다. 하지만 다이아몬드 문제와 같은 상속 문제가 골치 아프기 떄문이다. 그래서 Go는 상속의 기능을 가지지 않은 구조체를 만들었다. 클래스 비교 관점에서 바라본 구조체의 특징을 보면 다음과 같다:
- 상속 없음: 클래스와 달리 Go의 구조체는 상속을 지원하지 않는다. 대신 Go는 컴포지션을 사용하여 코드를 재사용한다.
- 캡슐화: Go 구조체는 캡슐화를 제공하지만 클래스에 있는 private 또는 protected와 같은 접근 지시자 같은 기능이 없다. 대신 Go는 필드 이름을 대문자로 표기하는 규칙을 사용하여 public 필드임을 나타낸다.
- 구조체 정의 내에 메서드 없음: 메서드는 구조체 자체 내에 정의되지 않고 외부에서 구조체 타입과 연관된다.

#### 상속과 컴포지션 관련 참고 글 
- [Why is there no type inheritance?](https://go.dev/doc/faq#Is_Go_an_object-oriented_language:~:text=always%20resolved%20statically.-,Why%20is%20there%20no%20type%20inheritance%3F,-Object%2Doriented%20programming) 
- [Why no "class" keyword in Go?](https://groups.google.com/g/golang-nuts/c/aJ6JiiIusqg/m/TJM09vOkv9YJ)
- [Why there are no classes in the GoLang programming language?](https://www.quora.com/Why-there-are-no-classes-in-the-GoLang-programming-language)

## 6. Method
Method(메서드)는 Function과 유사하지만 특정 타입(일반적으로 구조체)과 연관되어 있다. 메서드는 타입의 동작을 정의하고 구조체의 필드에 액세스하고 동작을 정의할 수 있다. 

그래서 흔히 구조체와 함께 자주 사용된다:
```go
type TypeName struct {
    // fields
}

func (receiver TypeName) methodName(parameters) returnType {
    // method body
}
```

### 2. 객체 지향 프로그래밍의 Class와 Go의 Method 비교
- Receiver: Go의 메서드는 클래스에서 메서드가 객체와 연관되는 방식과 유사하게 receiver argument를 사용하여 유형과 연관된다.
- `this` 키워드 없음: Go 메서드는 많은 객체 지향 언어에서 볼 수 있는 암시적 `this` 키워드 대신 명시적으로 receiver 이름을 사용한다.
- Pointer Receiver: Go 메서드는 클래스에서 객체의 상태를 수정하는 것과 유사하게 포인터 수신자를 사용하여 수신자의 값을 수정할 수 있다.

## 7. Pointer 
Pointer(이하 포인터)는 다른 변수의 메모리 주소를 보관하는 변수이다. 포인터를 사용하면 실제 메모리 위치를 직접 참조하고 조작할 수 있다. 주요 특성은 다음과 같다:
- 주소 연산자(`&`): 변수의 주소를 가져온다.
- 역참조 연산자(`*`): 포인터가 가리키는 주소의 값을 가져온다.
- 값의 변경: 포인터를 통해 값을 변경하면, 해당 주소에 저장된 실제 값이 변경된다.

간단한 예시 샘플 코드는 다음과 같다: 
```go
var ptr *int
ptr = &variable
```
- `*int`: 포인터 타입으로, int 타입의 변수를 가리키는 포인터임을 의미한다.
- `&variable`: 변수의 메모리 주소를 반환한다.


## 8. Closure
Closure(클로저)는 프로그래밍에서 중요한 개념이다. 이는 코드의 재사용성을 높이고, 변수를 캡슐화하며, 지연 실행 패턴을 구현하는 데 유용하기 떄문이다. 가볍게 역사를 살펴보면 다음과 같다: 
- 클로저의 개념은 함수형 프로그래밍 시초가 된 lambda calculus(람다 대수)에서 기원되었다. 
- 1950년 탄생한 LISP 언어에는 람다 대수의 아이디어를 실용적인 프로그래밍 언어로 구현한 것으로, 함수가 일급 객체로 취급되며 클로저의 개념을 포함하고 있다. 
- 2000년대 초반 클로저의 기능을 탑재한 Javascript, Python, Ruby의 새로운 언어 부흥과 기타 이유들로 인해 클로저의 존재감 또한 올라오게 되었다. 
- 1991년에 탄생한 Java 언어도 2014년 Java 8 버전에서 클로저 기능을 추가했다. [Like it or not, closures are coming to Java](https://www.infoworld.com/article/2078659/like-it-or-not--closures-are-coming-to-java.html)

클로저는는 함수 내에서 정의된 함수로, 외부 함수의 변수에 접근할 수 있는 기능을 가지고 있다. 클로저는 내부 함수가 외부 함수의 범위 내에서 변수를 "기억"하고 참조할 수 있도록 한다. Go는 초기 버전부터 클로저를 지원한다. Go의 클로저는 동시성 프로그래밍과 같은 여러 고급 기능을 구현하는 데 유용하게 다뤄지고 있다.

## Links
Go 언어에 대한 문서는 다른 언어에 비해 비교적 많은 편이다. C, C++, Java 등에 비교해보았을 때 최신 언어이기도 하고 Go의 장점을 좋아하는 사람들이 많아서 한글로 잘 작성된 문서들이 많다.
- [effective go(한국어 버전)](https://gosudaweb.gitbooks.io/effective-go-in-korean/content/)
- [Tucker의 Go 언어 프로그래밍](https://www.youtube.com/playlist?list=PLy-g2fnSzUTBHwuXkWQ834QHDZwLx6v6j)
- [golang korea github](https://github.com/golangkorea)

## Resources
1. Go Docs, "The Go Programming Language Specification: Language version go1.22", Feb 6. 2024, https://go.dev/ref/spec



