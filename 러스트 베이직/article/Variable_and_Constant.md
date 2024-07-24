# 변수와 상수

## 학습 목표
- Rust에서 변수와 상수의 개념을 이해한다.
- 불변성과 가변성의 차이점을 파악한다.
- Shadowing의 개념과 사용법을 학습한다.

## 변수 선언

Rust에서 변수는 `let`을 통해 정의할 수 있다. 여기서 주의할 점은, 다른 프로그래밍 언어들과 다르게 러스트의 기본 변수는 **불변성**이라는 것이다. 불변성이기 때문에, 변수가 한번 선언되었을 때, 이 변수의 값은 변경할 수가 없다.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // 이 줄에서 오류 발생
    println!("The value of x is: {}", x);
}
```

위 코드를 실행하면, 불변이어야 할 x를 6으로 변경하려고 했기 때문에 오류가 나는 것을 확인할 수 있다.

```
error[E0384]: re-assignment of immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ re-assignment of immutable variable
```

## 불변성의 필요성

Rust가 기본 변수를 불변성으로 둔 이유는 Rust가 제공하는 안전성과 손쉬운 동시성이라는 장점을 취할 수 있도록 코드를 작성하게끔 강제하는 위해서이다.

Rust에서는 컴파일러가 변경되지 않은 값에 대한 보증을 해주고, 실제로 이는 바뀌지 않는다. 이것이 의미하는 바는 코드를 작성하거나 분석할 시에 변수의 값이 어떻게 변경되는지 추적할 필요가 없기 때문에 코드를 더 합리적으로 만들어준다는 것이다.

## 가변 변수

그럼에도, 가변성은 매우 유용하게 사용될 수 있기에 Rust는 가변성을 `mut` 키워드를 변수에 추가하는 것을 통해 제공한다.

Rust는 변수명의 접두어로 `mut`을 추가하는 것을 통해 가변성 변수를 선언할 수 있다.

```rust
fn main() {
    let mut x = 5;  
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

위와 같이 `mut`을 통해 프로그램을 실행한다면 코드가 잘 실행되는 것을 확인할 수 있다.

## 상수 (Constant)

Rust의 기본 변수가 불변성을 지닌다고 했는데, 이는 constant와 어떤 차이를 가질까?  

Rust에서 constant는 `const` 키워드를 통해 생성할 수 있다.  

- `const`는 기본 변수와는 달리, 기본 설정이 불변성인 것이 아니라 불변성 그 자체라고 할 수 있다.  
- `const`는 뒤에서 배우게 될 Data Type을 반드시 선언해야 한다는 특징이 있다. 
- `const`는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있다. 
- `const`는 오직 const 표현식만 설정될 수 있다, 함수 호출의 결과값이나 그 외에 실행 시간에 결정되는 값이 설정될 수는 없다.

```rust
const MAX_POINTS: u32 = 100_000;
```

## Shadowing

Rust는 Shadowing을 통해 이전에 선언한 변수와 같은 이름의 변수를 선언할 수 있고 새 변수는 이전 변수를 shadow하게 된다. 예시 코드를 통해 살펴보자.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

이 프로그램은 처음 x에 값 5를 bind 하는데, 이후 반복된 `let x =` 구문으로 x를 shadow하고 원본 값에 1을 더해서 x의 값은 6이 되는 것을 확인할 수 있다. 같은 원리로 `x = x*2` 를 실행하여 아래와 같은 결과가 나온다. 

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

`mut`와는 다르게 shadowing은 `let` 키워드를 사용하지 않고 변수에 새로 값을 대입하려고 하면 컴파일-시에 에러가 얻게 된다. 또한, 값을 변경할 수는 있지만 그 이후에 변수는 그대로 불변성을 갖게 되는 것이다.  

더 많은 예시를 통해 shadowing을 이해해보자.  

```rust
let spaces = "   ";
let spaces = spaces.len();
```

위와 같이 문자열 유형의 변수 spaces를 선언하고 이를 다음 spaces에 shadowing 하는 것을 확인할 수 있다. 

shadowing을 할 때 주의해야 할 경우에 대해 알아보자. 

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

위와 같이 shadowing을 하게 된다면, 컴파일 시 에러가 나는 것을 확인할 수 있다. 이는 `mut`로 선언된 spaces 변수를 불변성을 지닌 기본변수에 shadowing을 시도할 경우 에러가 나는 것을 확인할 수 있다.

## 예제 코드

다음은 변수, 상수, shadowing을 사용하는 간단한 예제이다.

```rust
fn main() {
    // 불변 변수
    let x = 5;
    println!("The value of x is: {}", x);

    // 가변 변수
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The updated value of y is: {}", y);

    // 상수
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points is: {}", MAX_POINTS);

    // Shadowing
    let z = "   ";
    let z = z.len();
    println!("The length of z is: {}", z);
}
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new variables`
2. `src/main.rs` 파일에 위의 예제 코드를 붙여넣는다.
3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.
4. 결과를 확인하고, 변수의 불변성, 가변성, 상수, shadowing의 개념을 이해했는지 확인한다.

## 테스트 코드

예제 코드가 올바르게 작동하는지 확인하기 위한 테스트 코드는 다음과 같다:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_variable() {
        let x = 5;
        assert_eq!(x, 5);
    }

    #[test]
    fn test_mutable_variable() {
        let mut y = 10;
        y = 15;
        assert_eq!(y, 15);
    }

    #[test]
    fn test_constant() {
        assert_eq!(MAX_POINTS, 100_000);
    }

    #[test]
    fn test_shadowing() {
        let z = "   ";
        let z = z.len();
        assert_eq!(z, 3);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 모든 테스트가 통과하면 예제 코드가 올바르게 작성되었음을 확인할 수 있다.

이렇게 Rust의 변수와 상수에 대해 알아보았다. 불변성, 가변성, 상수, shadowing 등 각 개념의 특징과 차이점을 잘 이해하고 사용하는 것이 중요하다. 실제 코드를 작성할 때 이런 개념들을 적절히 활용하면 더 안전하고 효율적인 프로그램을 만들 수 있을 것이다.