# 에러 처리

## 학습 목표
- Rust의 에러 처리 철학과 방식을 이해한다.
- 복구 불가능한 에러와 `panic!` 매크로의 사용법을 익힌다.
- `Result<T, E>` 타입을 이용한 복구 가능한 에러 처리 방법을 학습한다.
- 에러 전파 기법과 `?` 연산자의 활용법을 파악한다.
- 언제 `panic!`을 사용하고 언제 `Result`를 반환할지 결정하는 기준을 이해한다.
- 사용자 정의 에러 타입 생성 방법을 습득한다.

## Rust의 에러 처리 철학

Rust는 안전성과 신뢰성을 중요시하는 언어로, 에러 처리에 있어서도 명확하고 체계적인 접근 방식을 취한다. Rust의 에러 처리 시스템은 크게 두 가지 카테고리로 나뉜다:

1. 복구 불가능한 에러 (Unrecoverable Errors)
2. 복구 가능한 에러 (Recoverable Errors)

이러한 구분은 프로그래머로 하여금 에러의 성격을 명확히 인지하고, 적절한 처리 방법을 선택할 수 있게 한다.

## 복구 불가능한 에러와 `panic!`

복구 불가능한 에러는 프로그램이 더 이상 정상적으로 실행될 수 없는 심각한 상황을 의미한다. Rust에서는 이러한 상황을 `panic!` 매크로를 통해 처리한다.

### `panic!` 매크로의 동작

`panic!`이 발생하면 다음과 같은 과정이 진행된다:

1. 에러 메시지 출력
2. 스택 되감기(unwinding) 또는 즉시 중단(abort)
3. 프로그램 종료

```rust
fn main() {
    panic!("crash and burn");
}
```

이 코드를 실행하면 다음과 같은 출력을 볼 수 있다:

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

### 백트레이스 사용하기

`RUST_BACKTRACE` 환경 변수를 설정하면 `panic!` 발생 시 더 자세한 정보를 얻을 수 있다:

```bash
$ RUST_BACKTRACE=1 cargo run
```

이는 디버깅 과정에서 매우 유용하다.

### `panic!`의 사용 시기

`panic!`은 다음과 같은 상황에서 주로 사용된다:

1. 절대 발생해서는 안 되는 상황에서의 에러
2. 외부 코드의 예기치 못한 동작
3. 더 이상의 에러 처리가 불가능한 상황

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // 이 라인은 panic!을 발생시킨다
}
```

## 복구 가능한 에러와 `Result<T, E>`

복구 가능한 에러는 프로그램의 정상적인 실행을 방해하지 않으면서 처리할 수 있는 에러를 의미한다. Rust에서는 이를 `Result<T, E>` 열거형을 통해 처리한다.

### `Result<T, E>` 타입

`Result<T, E>`는 다음과 같이 정의된다:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

여기서 `T`는 성공 시 반환될 값의 타입이고, `E`는 에러 시 반환될 에러의 타입이다.

### `Result` 사용 예제

파일을 열어 내용을 읽는 간단한 예제를 통해 `Result`의 사용법을 알아보자:

```rust
use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {}", e),
    }
}
```

이 예제에서 `?` 연산자는 `Result`를 반환하는 함수에서 에러를 쉽게 전파할 수 있게 해준다.

### `unwrap`과 `expect`

`Result`의 `unwrap`과 `expect` 메소드는 `Ok` 값을 반환하거나 `panic!`을 발생시킨다:

```rust
let f = File::open("hello.txt").unwrap();
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

이 메소드들은 주로 프로토타이핑이나 테스트에서 사용되며, 실제 프로덕션 코드에서는 더 세밀한 에러 처리가 권장된다.

## 에러 전파하기

함수에서 발생한 에러를 호출자에게 전달하는 것을 에러 전파라고 한다. Rust에서는 `?` 연산자를 통해 이를 간편하게 수행할 수 있다.

### `?` 연산자

`?` 연산자는 `Result`를 반환하는 표현식 뒤에 사용되며, 다음과 같이 동작한다:

1. 결과가 `Ok`이면 `Ok` 내부의 값을 추출한다.
2. 결과가 `Err`이면 해당 `Err`을 즉시 반환한다.

```rust
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

이 코드는 이전 예제와 동일한 기능을 하지만, `?` 연산자를 사용하여 더 간결해졌다.

### 연쇄적인 메소드 호출

`?` 연산자를 사용하면 여러 연산을 연쇄적으로 수행할 수 있다:

```rust
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

이 방식은 코드를 더욱 간결하고 읽기 쉽게 만든다.

## `panic!`이냐 `Result`냐

에러 상황에서 `panic!`을 사용할지 `Result`를 반환할지 결정하는 것은 중요한 설계 결정이다. 다음은 이에 대한 가이드라인이다:

### `panic!`을 사용해야 할 때

1. 예제, 프로토타입, 테스트에서
2. 복구가 불가능한 상황에서
3. 잘못된 값으로 인해 계속 실행하면 보안 문제가 발생할 수 있는 경우

### `Result`를 사용해야 할 때

1. 예상 가능한 에러 상황에서
2. 호출자가 에러를 복구하거나 처리할 수 있는 경우
3. 라이브러리 API를 설계할 때

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            Err(String::from("Guess value must be between 1 and 100"))
        } else {
            Ok(Guess { value })
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

이 예제에서는 `Result`를 사용하여 잘못된 입력을 처리하고 있다. 이는 호출자에게 에러 처리의 유연성을 제공한다.

## 사용자 정의 에러 타입

복잡한 애플리케이션에서는 사용자 정의 에러 타입을 만드는 것이 유용할 수 있다. 이를 통해 더 구체적이고 의미 있는 에러 처리가 가능해진다.

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::CustomError(s) => write!(f, "Custom error: {}", s),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}
```

이 예제에서는 `AppError`라는 사용자 정의 에러 타입을 만들고, 여러 종류의 에러를 하나의 타입으로 통합하고 있다.

## 고급 에러 처리 기법

Rust의 에러 처리 시스템은 기본적인 사용법 외에도 다양한 고급 기법을 제공한다. 이를 통해 더 복잡한 상황에서도 효과적인 에러 처리가 가능하다.

### 1. 다중 에러 타입 처리

여러 종류의 에러를 처리해야 하는 경우, `Box<dyn Error>`를 사용하여 다양한 에러 타입을 하나의 타입으로 통합할 수 있다.

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_and_parse() -> Result<i32, Box<dyn Error>> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
```

이 예제에서는 파일 I/O 에러와 파싱 에러, 두 가지 다른 종류의 에러를 `Box<dyn Error>`로 처리하고 있다. 사실은 밑에서 서술하는 `anyhow`와 같은 역할을 한다. 

### 2. `thiserror` 크레이트 사용

`thiserror` 크레이트를 사용하면 사용자 정의 에러 타입을 더 쉽게 만들 수 있다.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data not found")]
    NotFound(String),
    #[error("invalid data: {0}")]
    InvalidData(String),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}
```

이 예제에서는 `#[derive(Error)]`를 사용하여 자동으로 `Error` 트레이트를 구현하고, `#[error]` 속성으로 에러 메시지를 지정하고 있다.

### 3. `anyhow` 크레이트를 이용한 에러 처리 간소화

`anyhow` 크레이트는 에러 처리를 더욱 간단하게 만들어준다. 특히 애플리케이션 코드에서 유용하다.

```rust
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn read_config() -> Result<String> {
    let mut file = File::open("config.txt")
        .with_context(|| "Failed to open config file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| "Failed to read config file")?;
    Ok(contents)
}
```

`anyhow::Result`는 어떤 에러 타입이든 처리할 수 있으며, `with_context` 메소드를 통해 추가적인 컨텍스트 정보를 제공할 수 있다.
조금 다른 점은 anyhow에서 제네릭 트레이트인 Context를 구현하고 있다는 것.  with_context를 구현하고 있다는 건데, 이는 추가로 컨텍스트 로그를 남길 수 있게 해준다. 이외에도 다른 점이 있지만, 알아서 찾아보도록 한다.

Context 트레이트:
```rust
pub trait Context<T, E>: Sized {
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}
```

Context 트레이트의 구현:
Generic한 Result 타입에 대해서 작동하는 메서드를 구현하여 일반적으로 쓸 수 있게 해준다.  
```rust
impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            let context = f();
            anyhow::Error::new(error).context(context)
        })
    }
}
```


### 4. 에러 변환 (Error Conversion)

때로는 한 타입의 에러를 다른 타입으로 변환해야 할 필요가 있다. Rust에서는 `From` 트레이트를 구현하여 이를 수행할 수 있다.

```rust
#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

fn read_and_parse() -> Result<i32, AppError> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
```

이 예제에서는 `From` 트레이트를 구현하여 `std::io::Error`와 `std::num::ParseIntError`를 `AppError`로 자동 변환하고 있다.



## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_error_handling`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {}", e),
    }

    // panic! 예제
    // let v = vec![1, 2, 3];
    // v[99];
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

## 테스트 코드

예제 코드가 올바르게 작동하는지 확인하기 위한 테스트 코드는 다음과 같다:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_username_from_file() {
        // 테스트용 파일 생성
        let mut file = File::create("test_hello.txt").unwrap();
        file.write_all(b"test_username").unwrap();

        // 함수 테스트
        let result = read_username_from_file();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_username");

        // 테스트 파일 삭제
        std::fs::remove_file("test_hello.txt").unwrap();
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic() {
        let v = vec![1, 2, 3];
        let _ = v[99];
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. Rust 공식 문서 - 에러 처리: https://doc.rust-lang.org/book/ch09-00-error-handling.html
2. Rust by Example - 에러 처리: https://doc.rust-lang.org/rust-by-example/error.html
3. The Rust Programming Language (2nd Edition) by Steve Klabnik and Carol Nichols
4. Programming Rust (2nd Edition) by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall
5. Rust 공식 문서 - std::error 모듈: https://doc.rust-lang.org/std/error/index.html
6. `thiserror` 크레이트 문서: [https://docs.rs/thiserror](https://docs.rs/thiserror)
7. `anyhow` 크레이트 문서: [https://docs.rs/anyhow](https://docs.rs/anyhow)
8. Rust 에러 처리 모범 사례 블로그 포스트: [https://nick.groenen.me/posts/rust-error-handling/](https://nick.groenen.me/posts/rust-error-handling/)

