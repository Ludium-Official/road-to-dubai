# 데이터 타입

## 학습 목표
- Rust의 기본 데이터 타입을 이해한다.
- 스칼라 타입과 복합 타입의 차이를 파악한다.
- Rust의 타입 추론 기능을 이해한다.

## Rust의 타입 추론

Rust는 강력한 타입 추론 기능을 가지고 있다. 이는 많은 경우에 프로그래머가 명시적으로 타입을 지정하지 않아도 컴파일러가 문맥을 통해 타입을 추론할 수 있다는 것을 의미한다.

예를 들어:

```rust
let x = 5; // 컴파일러는 x를 i32로 추론
let y = 2.0; // 컴파일러는 y를 f64로 추론
```

하지만 때로는 타입을 명시적으로 지정해야 할 때가 있다:

```rust
let guess = "42".parse().expect("Not a number!");
```

이 경우, 컴파일러는 어떤 숫자 타입으로 파싱해야 할지 알 수 없어 오류가 발생한다:

```
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

이런 경우에는 명시적으로 타입을 지정해야 한다:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

## 스칼라 타입들

스칼라는 하나의 값으로 표현되는 타입으로, Rust는 정수형, 부동소수점 숫자, boolean, 그리고 문자, 네 가지 스칼라 타입을 보유하고 있다.

### 정수형 타입

정수형 타입은 signed/unsigned와 bit 수에 따라 다양한 타입들을 사용할 수 있다.

| 길이 | 부호 있는 | 부호 없는 |
|------|-----------|-----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

여기서 `isize`와 `usize` 타입은 프로그램이 동작하는 컴퓨터 아키텍처에 따라 결정된다. 64비트 아키텍처라면 64비트가 되는 식이다.

### 부동 소수점 타입 

Rust는 `f32`와 `f64` 두 가지의 부동 소수점 타입이 존재한다.

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

`f64`가 기본 타입으로, 현대 CPU에서 `f32`와 거의 같은 속도로 더 정밀한 결과를 낼 수 있다.

### boolean 타입 

대부분의 다른 언어들처럼, boolean 타입은 Rust에서 `true`와 `false` 둘 중 하나의 값만을 가질 수 있다.

```rust
fn main() {
    let t = true;
    let f: bool = false; // 명시적 타입 지정
}
```

### 문자 타입 

Rust는 문자 또한 지원한다. Rust의 `char` 타입은 Unicode Scalar를 표현하는 값이고 이는 ASCII보다 많은 표현을 가능하게 한다. 억양 표시가 있는 문자, 한국어/중국어/일본어 표의 문자, 이모티콘, 넓이가 0인 공백문자 모두가 Rust에서는 `char` 타입으로 사용할 수 있다.

```rust
let c: char = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

`char` 리터럴은 작은따옴표로 명시하며, 문자열 리터럴과는 다르게 큰따옴표를 사용하지 않는다.

## 복합 타입 

복합 타입들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있다. Rust는 두 개의 기본 복합 타입을 갖고 있는데, 각각 튜플과 배열이다.

### 튜플 타입 

튜플은 다양한 타입의 몇 개의 값을 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법이다.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

튜플 값을 밖으로 빼내오기 위해서는 튜플을 구조해체하는 작업이 진행돼야 한다:

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

구조해체 말고도, 마침표(.) 뒤에 우리가 접근하길 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있다:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

### 배열 

배열은 튜플과는 다르게 배열 안의 모든 요소가 같은 타입이어야 한다. Rust의 배열이 몇몇 다른 언어들의 배열과 다른 점은 Rust에서는 배열이 고정된 길이를 갖는다는 점이다: 한번 선언되면, 이들의 크기는 커지거나 작아지지 않는다.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

배열은 벡터 타입과는 다르게 크기가 고정되어 있기 때문에, 변경되지 않는 고정된 요소들의 리스트를 다룰 때 유용하다. 

배열의 요소에 접근하는 방법:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new data_types`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
fn main() {
    // 정수형
    let x: i32 = 42;
    println!("x: {}", x);

    // 부동소수점
    let y: f64 = 3.14;
    println!("y: {}", y);

    // boolean
    let t: bool = true;
    println!("t: {}", t);

    // 문자
    let c: char = '🦀';
    println!("c: {}", c);

    // 튜플
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (a, b, d) = tup;
    println!("a: {}, b: {}, d: {}", a, b, d);

    // 배열
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]: {}, arr[4]: {}", arr[0], arr[4]);
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

## 테스트 코드

예제 코드가 올바르게 작동하는지 확인하기 위한 테스트 코드는 다음과 같다:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_integer() {
        let x: i32 = 42;
        assert_eq!(x, 42);
    }

    #[test]
    fn test_float() {
        let y: f64 = 3.14;
        assert!((y - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn test_boolean() {
        let t: bool = true;
        assert_eq!(t, true);
    }

    #[test]
    fn test_char() {
        let c: char = '🦀';
        assert_eq!(c, '🦀');
    }

    #[test]
    fn test_tuple() {
        let tup: (i32, f64, char) = (500, 6.4, 'A');
        assert_eq!(tup.0, 500);
        assert!((tup.1 - 6.4).abs() < f64::EPSILON);
        assert_eq!(tup.2, 'A');
    }

    #[test]
    fn test_array() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr[0], 1);
        assert_eq!(arr[4], 5);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 모든 테스트가 통과하면 예제 코드가 올바르게 작성되었음을 확인할 수 있다.
