# Rust의 Flow 제어

## 학습 목표
- Rust의 제어 흐름 구조를 이해하고 효과적으로 사용할 수 있다.
- 이터레이터의 개념과 사용법을 숙지한다.
- 패턴 매칭의 고급 기능을 익히고 활용할 수 있다.

## 제어 흐름 구조

Rust의 제어 흐름 구조는 프로그램의 실행 경로를 결정하는 핵심 요소이다. 주요 제어 흐름 구조로는 조건문, 반복문, 그리고 패턴 매칭이 있다.

### 조건문 (if 표현식)

Rust의 `if` 표현식은 다른 언어의 조건문과 유사하지만, 표현식으로 사용될 수 있다는 특징이 있다.

```rust
fn main() {
    let number = 6;
    let result = if number % 2 == 0 {
        "짝수"
    } else {
        "홀수"
    };
    println!("숫자 {}는 {}입니다.", number, result);
}
```

이 코드에서 `if` 표현식의 결과가 `result` 변수에 직접 할당된다. 이는 Rust의 표현식 중심 특성을 보여주는 좋은 예시이다.

### 반복문

Rust는 세 가지 주요 반복문을 제공한다: `loop`, `while`, `for`.

#### loop

`loop`는 무한 반복을 위한 키워드이다. `break`를 사용하여 루프를 종료하고 값을 반환할 수 있다.

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("결과: {}", result); // 출력: 결과: 20
}
```

#### while

`while` 루프는 조건이 참인 동안 계속해서 실행된다.

```rust
fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
```

#### for

`for` 루프는 이터레이터를 순회하는 데 사용된다.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}
```

## 이터레이터

이터레이터는 일련의 값들을 순회하는 방법을 제공한다. Rust의 이터레이터는 지연 평가(lazy evaluation)되며, 필요할 때만 값을 생성한다.

### 이터레이터 생성

컬렉션에서 이터레이터를 생성하는 방법:

```rust
let v = vec![1, 2, 3, 4, 5];
let iter = v.iter(); // 불변 참조 이터레이터
let iter_mut = v.iter_mut(); // 가변 참조 이터레이터
let into_iter = v.into_iter(); // 소유권을 가져가는 이터레이터
```

### 이터레이터 메서드

이터레이터는 다양한 유용한 메서드를 제공한다:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map: 각 요소를 변환
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("제곱: {:?}", squared);

    // filter: 조건에 맞는 요소만 선택
    let even: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("짝수: {:?}", even);

    // fold: 초기값과 함께 모든 요소를 누적
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("합계: {}", sum);
}
```

## 패턴 매칭

Rust의 패턴 매칭은 강력하고 표현력이 뛰어난 기능이다. `match` 표현식을 통해 복잡한 데이터 구조를 분해하고 처리할 수 있다.

### 기본 패턴 매칭

```rust
fn main() {
    let x = 1;
    match x {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("그 외"),
    }
}
```

### 구조체 분해

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("x축 위의 점: ({}, 0)", x),
        Point { x: 0, y } => println!("y축 위의 점: (0, {})", y),
        Point { x, y } => println!("다른 점: ({}, {})", x, y),
    }
}
```

### 열거형 매칭

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("종료"),
        Message::Move { x, y } => println!("이동: x={}, y={}", x, y),
        Message::Write(text) => println!("텍스트: {}", text),
        Message::ChangeColor(r, g, b) => println!("색상 변경: R={}, G={}, B={}", r, g, b),
    }
}
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_flow_control`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
fn main() {
    // 조건문
    let number = 6;
    let result = if number % 2 == 0 { "짝수" } else { "홀수" };
    println!("숫자 {}는 {}입니다.", number, result);

    // 반복문
    for i in 1..=5 {
        println!("{}의 제곱: {}", i, i * i);
    }

    // 이터레이터
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("합계: {}", sum);

    // 패턴 매칭
    let message = Message::Write(String::from("안녕하세요"));
    match message {
        Message::Quit => println!("종료"),
        Message::Move { x, y } => println!("이동: x={}, y={}", x, y),
        Message::Write(text) => println!("텍스트: {}", text),
        Message::ChangeColor(r, g, b) => println!("색상 변경: R={}, G={}, B={}", r, g, b),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

## 테스트 코드

예제 코드가 올바르게 작동하는지 확인하기 위한 테스트 코드는 다음과 같다:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_expression() {
        let number = 6;
        let result = if number % 2 == 0 { "짝수" } else { "홀수" };
        assert_eq!(result, "짝수");
    }

    #[test]
    fn test_iterator() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_pattern_matching() {
        let message = Message::Write(String::from("테스트"));
        match message {
            Message::Write(text) => assert_eq!(text, "테스트"),
            _ => panic!("예상치 못한 메시지 유형"),
        }
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. Rust 공식 문서 - 제어 흐름: https://doc.rust-lang.org/book/ch03-05-control-flow.html
2. Rust 공식 문서 - 이터레이터: https://doc.rust-lang.org/book/ch13-02-iterators.html
3. Rust 공식 문서 - 패턴과 매칭: https://doc.rust-lang.org/book/ch18-00-patterns.html
4. "Programming Rust" by Jim Blandy and Jason Orendorff, O'Reilly Media
5. "Rust in Action" by Tim McNamara, Manning Publications
6. Rust by Example - 흐름 제어: https://doc.rust-lang.org/rust-by-example/flow_control.html
