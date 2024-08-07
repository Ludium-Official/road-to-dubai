# 모나드 (Monad)

## 학습 목표
- 모나드의 개념과 필요성을 이해한다.
- Rust에서 모나드를 구현하는 방법을 학습한다.
- Option과 Result 타입을 통해 모나드의 실제 사용을 익힌다.
- 모나드 법칙과 그 의미를 파악한다.
- 함수형 프로그래밍에서 모나드의 역할을 이해한다.
- Rust의 타입 시스템과 모나드의 관계를 학습한다.
- 실제 코드에서 모나드를 활용하는 방법을 습득한다.

## 모나드의 개념
(참고: 모나드는 직접 개념을 찾아보기 바란다.)
모나드(Monad)는 값을 래핑하고 계산을 추상화하는 구조체다. 이는 함수형 프로그래밍에서 부작용을 관리하고, 복잡한 계산을 단순화하는 데 사용된다. Rust에서는 직접적으로 모나드라는 용어를 사용하지 않지만, Option과 Result 같은 타입들이 모나드의 개념을 구현하고 있다.

### 모나드의 필요성

1. 부작용 관리: 순수 함수형 컨텍스트에서 부작용을 다룰 수 있게 해준다.
2. 계산의 추상화: 복잡한 연산을 단순하고 읽기 쉬운 형태로 표현할 수 있다.
3. 에러 처리: 예외 처리를 함수형 방식으로 구현할 수 있다.
4. 컨텍스트 제공: 값에 추가적인 컨텍스트를 부여할 수 있다.

## Rust에서의 모나드 구현

Rust에서 모나드는 주로 Option과 Result 타입을 통해 구현된다. 이들은 각각 '값이 있거나 없을 수 있는 상황'과 '성공 또는 실패할 수 있는 연산'을 표현한다.

### Option을 이용한 모나드 예제

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = Some(4.0)
        .and_then(|x| divide(x, 2.0))
        .and_then(|y| divide(y, 2.0));

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Division by zero"),
    }
}
```

이 예제에서 `and_then` 메서드는 모나드의 `bind` 연산을 구현한다.

### Result를 이용한 모나드 예제

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn divide(x: f64, y: f64) -> Result<f64, MathError> {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn main() {
    let result = divide(1.0, 2.0)
        .and_then(|x| sqrt(x))
        .and_then(|y| divide(y, 2.0));

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

여기서 `Result`는 성공 또는 실패의 컨텍스트를 제공하는 모나드로 작동한다.

## 모나드 법칙

모나드는 다음 세 가지 법칙을 만족해야 한다:

1. 왼쪽 항등 법칙: `return a >>= f` ≡ `f a`
2. 오른쪽 항등 법칙: `m >>= return` ≡ `m`
3. 결합 법칙: `(m >>= f) >>= g` ≡ `m >>= (\x -> f x >>= g)`

Rust에서 이는 `Option`과 `Result`의 `and_then` 메서드를 통해 구현된다.

## Rust 표준 라이브러리의 모나드 예

Rust의 `Option`과 `Result` 타입은 모나드의 특성을 가지고 있다:

```rust
fn main() {
    let x = Some(3);
    let y = x.map(|i| i * 2).and_then(|i| Some(i + 1));
    println!("{:?}", y);  // Prints: Some(7)

    let x: Result<i32, &str> = Ok(3);
    let y = x.map(|i| i * 2).and_then(|i| Ok(i + 1));
    println!("{:?}", y);  // Prints: Ok(7)
}
```

여기서 `map`과 `and_then` 메서드는 모나드의 연산을 구현한다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_monads`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn find_user(id: i32) -> Option<User> {
    if id == 1 {
        Some(User { id: 1, name: String::from("Alice") })
    } else {
        None
    }
}

fn get_company(user: &User) -> Option<String> {
    if user.id == 1 {
        Some(String::from("Acme Corp"))
    } else {
        None
    }
}

fn main() {
    let user_company = find_user(1)
        .as_ref()
        .and_then(get_company);

    match user_company {
        Some(company) => println!("User works at: {}", company),
        None => println!("Company not found"),
    }
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_user() {
        assert!(find_user(1).is_some());
        assert!(find_user(2).is_none());
    }

    #[test]
    fn test_get_company() {
        let user = User { id: 1, name: String::from("Alice") };
        assert_eq!(get_company(&user), Some(String::from("Acme Corp")));
    }

    #[test]
    fn test_monad_chain() {
        let result = find_user(1)
            .as_ref()
            .and_then(get_company);
        assert_eq!(result, Some(String::from("Acme Corp")));

        let result = find_user(2)
            .as_ref()
            .and_then(get_company);
        assert_eq!(result, None);
    }
}
```

## Reference

1. Rust 공식 문서 - Option: https://doc.rust-lang.org/std/option/enum.Option.html
2. Rust 공식 문서 - Result: https://doc.rust-lang.org/std/result/enum.Result.html
3. "Programming Rust" by Jim Blandy and Jason Orendorff
4. "Rust in Action" by Tim McNamara
5. Haskell Wiki - Monad: https://wiki.haskell.org/Monad
6. "Learn You a Haskell for Great Good!" by Miran Lipovača (모나드 개념)
7. Rust 공식 포럼 - 모나드 토론: https://users.rust-lang.org/t/monad-in-rust/14351
8. "Category Theory for Programmers" by Bartosz Milewski
9. Rust RFC - ? 연산자 (모나드와 관련): https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md
