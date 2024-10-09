# 커링 (Currying)

## 학습 목표
- 커링의 개념과 필요성을 이해한다.
- 커링의 구현 방법을 학습한다.
- Rust에서의 커링 구현 방법을 익힌다.
- 실제 사용 사례를 통해 커링의 활용법을 학습한다.
- 커링과 관련된 함수형 프로그래밍 개념을 이해한다.

## 커링의 개념

커링은 여러 개의 인자를 가진 함수를 단일 인자를 가진 함수들의 체인으로 변환하는 기법이다. 이는 함수형 프로그래밍의 핵심 개념 중 하나이다.

### 커링의 필요성

1. 함수의 재사용성 증가: 일부 인자만 적용된 새로운 함수를 만들 수 있다.
2. 부분 적용(Partial Application): 일부 인자만 미리 적용하여 새로운 함수를 만들 수 있다.
3. 함수 조합(Function Composition): 단일 인자 함수들을 쉽게 조합할 수 있다.
4. 지연 평가(Lazy Evaluation): 모든 인자가 제공될 때까지 함수 실행을 지연시킬 수 있다.

## 커링의 구현

Rust에서 커링을 구현하는 방법은 다음과 같다:

```rust
fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

이 함수는 하나의 인자를 받아 새로운 함수를 반환한다. 반환된 함수는 또 다른 인자를 받아 최종 결과를 계산한다.


## 커링의 장단점

### 장점
1. 함수의 재사용성 증가
2. 코드의 모듈화 향상
3. 함수 조합의 용이성

## Rust에서의 커링 구현

Rust에서는 클로저와 `move` 키워드를 사용하여 커링을 구현할 수 있다:

```rust
fn curry_multiply(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

fn main() {
    let multiply_by_5 = curry_multiply(5);
    println!("5 * 3 = {}", multiply_by_5(3)); // 출력: 5 * 3 = 15
}
```

## 실제 사용 사례

로깅 함수에 커링을 적용한 예:
```rust
fn curry_log<F>(prefix: String) -> impl Fn(String) -> Box<dyn Fn(F) -> ()>
// impl 중첩 안됨 
// Box<dyn Fn(F) -> ()>는 동적 디스패치를 사용하며, 다양한 클로저 타입을 반환할 수 있다.
//impl Fn(F) -> ()는 정적 디스패치를 사용하며, 단일 구체적 타입을 반환한다.

where
    F: Fn(String) + 'static
{
    move |message| {
        let full_message = format!("{}: {}", prefix, message);
        Box::new(move |log_fn: F| log_fn(full_message.clone()))
    }
}

fn main() {
    let error_log = curry_log("ERROR".to_string());
    let print_error = error_log("Something went wrong".to_string());
    
    print_error(|msg| println!("{}", msg));
    // 출력: ERROR: Something went wrong
}
```


## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_currying`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    let add_5 = curry_add(5);
    println!("5 + 3 = {}", add_5(3));
    println!("5 + 7 = {}", add_5(7));
    
    let add_10 = curry_add(10);
    println!("10 + 3 = {}", add_10(3));
    println!("10 + 7 = {}", add_10(7));
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
    fn test_curry_add() {
        let add_5 = curry_add(5);
        assert_eq!(add_5(3), 8);
        assert_eq!(add_5(7), 12);

        let add_10 = curry_add(10);
        assert_eq!(add_10(3), 13);
        assert_eq!(add_10(7), 17);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. "Functional Programming in Rust" by Florian Gilcher
2. "Programming Rust" by Jim Blandy and Jason Orendorff
3. Rust 공식 문서 - 클로저: https://doc.rust-lang.org/book/ch13-01-closures.html
4. "Haskell Programming from First Principles" by Christopher Allen and Julie Moronuki (커링 개념에 대한 심도 있는 설명)
5. "Functional Programming in JavaScript" by Luis Atencio
6. Rust RFC - 클로저 개선: https://github.com/rust-lang/rfcs/blob/master/text/2132-clojure-closure-reform.md
7. Rust 공식 포럼 - 커링 관련 토론: https://users.rust-lang.org/t/currying-in-rust/5140
8. "Category Theory for Programmers" by Bartosz Milewski (커링과 관련된 수학적 개념)
