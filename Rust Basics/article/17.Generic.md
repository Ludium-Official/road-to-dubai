# 제네릭 

## 학습 목표
- Rust의 제네릭 개념과 그 필요성을 이해한다.
- 함수, 구조체, 열거형, 메서드에서 제네릭을 사용하는 방법을 익힌다.
- 트레이트 바운드를 이용해 제네릭 타입에 제약을 거는 방법을 학습한다.
- 제네릭의 성능 영향과 단형화(monomorphization) 개념을 파악한다.
- 연관 타입(associated types)과 제네릭 상수 매개변수의 활용법을 습득한다.
- 제네릭의 실제 사용 사례와 모범 사례를 이해한다.

## Rust의 제네릭 개념

제네릭은 Rust의 강력한 기능 중 하나로, 코드의 재사용성을 높이고 중복을 줄이는 데 큰 역할을 한다. 제네릭을 사용하면 구체적인 타입 대신 추상적인 스탠드인(stand-in) 타입을 사용하여 함수나 데이터 구조를 정의할 수 있다.

### 제네릭의 필요성

1. 코드 재사용: 다양한 타입에 대해 동일한 로직을 적용할 수 있다.
2. 타입 안전성: 컴파일 시간에 타입 검사를 수행하여 런타임 오류를 줄인다.
3. 추상화: 구체적인 타입에 의존하지 않는 일반적인 알고리즘을 작성할 수 있다. 이를 통해서 트레이트를 제네릭하게 구현하여, 기존에 있는 타입에 메서드를 붙일 수 있는 등의 활용을 할 수 있다. 

## 제네릭의 기본 사용법

### 함수에서의 제네릭

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

이 함수는 `PartialOrd` 트레이트를 구현한 모든 타입 `T`에 대해 동작한다.

### 구조체에서의 제네릭

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

### 열거형에서의 제네릭

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 메서드 정의에서의 제네릭

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32>{
    fn x(&self) -> &i32{
        &self
    }
} // 임의로 작성한 구현이지만, 이것은 제네릭한 trait에 대해 특정한 type에 대해서만 구현한 것을 의미한다.
```

## 트레이트 바운드

트레이트 바운드를 사용하면 제네릭 타입이 특정 동작을 가져야 한다고 지정할 수 있다.

```rust
fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

### 다중 트레이트 바운드

```rust
fn process<T: Clone + Debug>(item: T) {
    // ...
}
```

### where 절

복잡한 트레이트 바운드는 `where` 절을 사용하여 더 명확하게 표현할 수 있다:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // 함수 본문
}
```

## 제네릭의 성능과 단형화

Rust의 제네릭은 컴파일 시간에 단형화(monomorphization)라는 과정을 통해 컴파일  타임에 구체적인 타입으로 변환된다. 이로 인해 런타임 성능 저하 없이 제네릭의 이점을 누릴 수 있다.

```rust
let integer = Some(5);
let float = Some(5.0);
```

이 코드는 컴파일 시 다음과 같이 변환된다:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);
```


## 실제 사용 사례

제네릭은 실제 프로그래밍에서 다양하게 활용된다:

1. 컬렉션: Vec, HashMap 등의 표준 컬렉션들은 제네릭을 사용한다.
2. 결과 처리: Option과 Result 타입은 제네릭을 활용한 대표적인 예이다.
3. 알고리즘: 정렬, 검색 등의 일반적인 알고리즘은 제네릭으로 구현된다.
4. 데이터 구조: 링크드 리스트, 트리 등의 자료구조도 제네릭으로 구현할 수 있다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_generics`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
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
    fn test_point_creation() {
        let int_point = Point::new(1, 2);
        assert_eq!(int_point.x, 1);
        assert_eq!(int_point.y, 2);

        let float_point = Point::new(1.5, 2.5);
        assert_eq!(float_point.x, 1.5);
        assert_eq!(float_point.y, 2.5);
    }

    #[test]
    fn test_generic_function() {
        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        assert_eq!(*result, 100);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        assert_eq!(*result, 'y');
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. Rust 공식 문서 - 제네릭: https://doc.rust-lang.org/book/ch10-00-generics.html
2. Rust by Example - 제네릭: https://doc.rust-lang.org/rust-by-example/generics.html
3. The Rust Programming Language (2nd Edition) by Steve Klabnik and Carol Nichols
4. Programming Rust (2nd Edition) by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall
5. Rust 공식 문서 - std::marker 모듈: https://doc.rust-lang.org/std/marker/index.html
6. Rust 공식 문서 - 트레이트: https://doc.rust-lang.org/book/ch10-02-traits.html
7. Rust 제네릭 성능에 관한 블로그 포스트: https://blog.rust-lang.org/2018/01/31/Rust-1.24.html#zero-cost-abstractions
8. Rust RFC - 제네릭 연관 타입: https://rust-lang.github.io/rfcs/0195-associated-items.html