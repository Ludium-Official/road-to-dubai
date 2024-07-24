# 패턴 매칭

## 학습 목표
- Rust의 패턴 매칭 시스템의 기본 개념과 중요성을 이해한다.
- 다양한 패턴 매칭 문법과 사용 방법을 숙지한다.
- 패턴 매칭의 고급 기법과 활용 사례를 학습한다.
- 함수형 프로그래밍에서 패턴 매칭의 중요성을 이해한다.
- 패턴 매칭이 Rust의 타입 시스템과 어떻게 상호작용하는지 파악한다.
- VS Code에서 실제 코드를 작성한다.


## 패턴 매칭의 기본 개념

Rust의 패턴 매칭은 강력하고 유연한 기능으로, 복잡한 데이터 구조를 분해하고 조건부 로직을 간결하게 표현할 수 있게 해준다. 이는 Rust의 타입 시스템과 긴밀하게 연결되어 있어, 컴파일 시점에 많은 오류를 잡아낼 수 있다.

패턴 매칭은 값의 구조를 분석하고 그에 따라 코드를 실행하는 방법이다. 이는 단순한 조건문보다 더 표현력이 뛰어나며, 특히 복잡한 데이터 구조를 다룰 때 유용하다.

### 기본 문법

패턴 매칭의 기본 형태는 `match` 표현식이다:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

여기서 `value`는 매칭할 값이고, `pattern1`, `pattern2` 등은 매칭될 수 있는 패턴이다. `_`는 모든 나머지 경우를 포괄하는 와일드카드 패턴이다.
만일 패턴 내의 변수가 있다면, 패턴 내의 변수들은 매칭되는 값의 부분들을 "캡처"하여 그 값들을 바인딩한다. 이를 통해 매칭된 값의 특정 부분에 쉽게 접근하고 사용할 수 있게 된다.

```rust
let x = 5;

match x {
    n @ 1..=5 => println!("Got a number between 1 and 5: {}", n),
    n => println!("Got something else: {}", n),
}
```
n@ 1..=5는, 1부터 5라면, n에 값을 바인딩한다는 것과 일치한다. 여기서 주의할 점은, 이 값 바인딩 또한 소유권의 영향을 받는 move를 수행한다는 점이다. copy 트레잇이 구현되어있지 않으면, 이는 move를 수행하게 된다:

```rust
    let s = String::from("hello");

    let x= match s {
        x => x,
        };
        
    println!("{}",s); // 컴파일 에러 

    let s = String::from("hello");

    match &s {
        x => println!("Got: {}", x),
    }

    println!("Original: {}", s);  // 이것은 가능하다.

    let mut x = 5;

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    println!("x is still {}", x);  // 이것은 가능하다.

```
따라서 참조를 넘기거나, ref 키워드를 통해서 match를 선언해야한다 


## 다양한 패턴 매칭 기법

### 1. 리터럴 매칭

가장 간단한 형태의 패턴 매칭으로, 리터럴 값과 직접 비교한다.

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### 2. 범위 매칭

값의 범위를 매칭할 수 있다. Rust 1.26부터는 포함적 범위 문법 `..=`이 도입되었다.

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

### 3. 구조체 분해

구조체의 필드를 개별적으로 매칭할 수 있다.

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

### 4. 열거형 매칭

열거형의 각 변형을 매칭할 수 있으며, 이는 Rust에서 매우 자주 사용되는 패턴이다.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
}
```

## 함수형 프로그래밍과 패턴 매칭

패턴 매칭은 함수형 프로그래밍의 핵심 개념 중 하나이다. Rust의 패턴 매칭은 함수형 프로그래밍 패러다임을 강력하게 지원한다.

1. **데이터 분해**: 복잡한 데이터 구조를 쉽게 분해하고 필요한 부분만 추출할 수 있다.

2. **패턴 기반 함수 정의**: 함수의 인자를 패턴으로 정의하여 더 명확하고 안전한 코드를 작성할 수 있다.

3. **재귀 함수**: 재귀 함수에서 베이스 케이스와 재귀 케이스를 명확하게 구분할 수 있다.

4. **Option과 Result 처리**: 함수형 프로그래밍에서 자주 사용되는 `Option`과 `Result` 타입을 우아하게 처리할 수 있다.

예를 들어, 다음과 같이 리스트를 처리하는 재귀 함수를 작성할 수 있다:

```rust
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn sum_list(list: List<i32>) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(head, tail) => head + sum_list(*tail),
    }
}
```

예제에서 패턴 매칭은 리스트의 구조를 분해하고, 재귀의 베이스 케이스와 재귀 케이스를 명확하게 구분하는 데 사용된다.

## 패턴 매칭과 타입 시스템의 상호작용

Rust의 패턴 매칭은 타입 시스템과 긴밀하게 연결되어 있어, 컴파일 시점에 많은 오류를 잡아낼 수 있다.

1. **패턴의 완전성 검사**: 컴파일러는 `match` 표현식이 모든 가능한 경우를 처리하는지 검사한다.

2. **타입 추론**: 패턴 매칭은 타입 추론에도 도움을 준다. 예를 들어, 튜플을 분해할 때 각 요소의 타입을 추론할 수 있다.

3. **타입 안전성**: 패턴 매칭은 타입 안전성을 높이는 데 기여한다. 잘못된 타입의 값을 매칭하려고 하면 컴파일 오류가 발생한다.

이러한 특성들은 Rust 코드의 안전성과 신뢰성을 크게 향상시킨다.

## VS Code에서 실습

이제 VS Code를 사용하여 패턴 매칭의 다양한 기능을 직접 실습해보자.

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new patterns_demo`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) if text.len() < 10 => println!("Short message: {}", text),
        Message::Write(text) => println!("Long message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b),
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::Write(String::from("This is a long message")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        process_message(msg);
    }
}
```

3. 터미널에서 `cargo run`을 실행하여 코드를 컴파일하고 실행한다.

이번에는 따로 테스트 코드는 생략한다. 