# 트레이트

## 학습 목표
- Trait의 개념과 필요성을 이해한다.
- Trait의 정의와 구현 방법을 학습한다.
- 기본 구현과 트레이트 바운드의 사용법을 익힌다.
- 트레이트 객체와 동적 디스패치의 개념을 파악한다.
- 관련 함수와 연관 타입의 사용법을 학습한다.
- 트레이트의 상속과 트레이트 안전성에 대해 이해한다.
- 최신 Rust 버전의 트레이트 관련 기능을 습득한다.

## Trait의 개념

Trait은 Rust에서 타입의 행동을 정의하는 방법이다. 다른 언어의 인터페이스와 유사하지만, 더 강력한 기능을 제공한다.

### Trait의 필요성

1. 코드 재사용: 여러 타입에 대해 공통된 행동을 정의할 수 있다.
2. 다형성: 트레이트 객체를 통해 런타임 다형성을 구현할 수 있다.
3. 타입 안전성: 컴파일 시간에 타입의 행동을 보장한다.
4. 추상화: 구체적인 구현을 숨기고 인터페이스만 노출할 수 있다.

## Trait의 정의와 구현

Trait의 정의는 다음과 같이 한다:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Trait의 구현은 다음과 같이 한다:

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## 기본 구현

Trait에 기본 구현을 제공할 수 있다:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

구현하는 타입에서 오버라이드하지 않으면 기본 구현이 사용된다.

## 트레이트 바운드

제네릭 함수에서 트레이트 바운드를 사용하여 타입을 제한할 수 있다:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

여러 트레이트 바운드를 지정할 수도 있다:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
    // ...
}
```

## 트레이트 객체와 동적 디스패치

트레이트 객체를 사용하여 런타임 다형성을 구현할 수 있다:

```rust
pub fn notify(item: &dyn Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

이는 동적 디스패치를 사용하여 런타임에 메서드를 호출한다. 여기서 `&dyn Summary`는 Summary 트레이트를 구현한 어떤 타입의 참조라도 받을 수 있다는 의미다.

`동적 디스패치`란?
이는 다형성을 구현하는 방법 중 하나로, 다음을 의미한다:

메서드 호출이 런타임에 결정된다.
컴파일러는 vtable(가상 메서드 테이블)을 생성하여 실제 메서드의 위치를 저장한다.
런타임에 vtable을 통해 적절한 메서드를 찾아 호출한다. 따라서 런타임 오버헤드가 있다. 
따라서, 메모리는 적게 든다(각 타입별로 함수 객체가 생성될 필요는 없음.)


하지만, Rust는 보통의 경우에 정적 디스패치를 선호한다. 

```rust
fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}

let circle = Circle { radius: 2.0 };
print_area(circle);
```

`정적 디스패치`란?
이는 다형성을 구현하는 방법 중 하나로, 이는 다음과 같은 특징을 가진다:

컴파일 시간에 호출될 함수가 결정된다.
런타임 오버헤드가 없어 성능이 우수하다.(시간이 덜 걸린다)
필요 메모리가 증가할 수 있다. (호출될 때마다 각 타입별로 함수 객체가 새로 생성되므로).


## 관련 함수와 연관 타입

트레이트에 관련 함수(연관 함수)를 정의할 수 있다:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    fn new() -> Self;
}
```

연관 타입을 사용하여 트레이트 내에서 사용할 타입을 추상화할 수 있다:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## 트레이트의 상속

트레이트는 다른 트레이트를 상속할 수 있다:

```rust
trait Animal {
    fn name(&self) -> String;
}

trait Dog: Animal {
    fn bark(&self);
}
```

## 트레이트 안전성

Rust 1.34부터 도입된 "트레이트 안전성" 개념은 트레이트 객체로 사용할 수 있는 트레이트를 제한한다. 트레이트가 객체 안전(object safe)하려면 다음 조건을 만족해야 한다:

1. 모든 메서드의 반환 타입이 Self가 아니어야 한다.
2. 메서드에 제네릭 타입 매개변수가 없어야 한다.

## 최신 Rust 버전의 트레이트 관련 기능

1. const trait (Rust 1.61부터): 컴파일 타임 상수에 대해 트레이트를 구현할 수 있다.

```rust
const fn five() -> i32 { 5 }

trait ConstFn {
    fn call() -> i32;
}

impl ConstFn for five {
    fn call() -> i32 {
        five()
    }
}
```

2. GAT (Generic Associated Types, Rust 1.65부터): 연관 타입에 제네릭 매개변수를 사용할 수 있다.

```rust
trait Container {
    type Item<T>;
    fn contains<T>(&self, item: &T) -> bool
    where
        Self: Container<Item<T> = T>;
}
```

3. Specialization (아직 unstable): 더 구체적인 타입에 대해 트레이트 구현을 특화할 수 있다.

```rust
#![feature(specialization)]

trait MyTrait {
    fn foo(&self) -> i32 { 1 }
}

impl<T> MyTrait for T {}

impl MyTrait for u8 {
    fn foo(&self) -> i32 { 2 }
}
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_traits`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

fn animal_sounds(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("The animal says: {}", animal.make_sound());
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    animal_sounds(animals);
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
    fn test_dog_sound() {
        let dog = Dog;
        assert_eq!(dog.make_sound(), "Woof!");
    }

    #[test]
    fn test_cat_sound() {
        let cat = Cat;
        assert_eq!(cat.make_sound(), "Meow!");
    }

    #[test]
    fn test_animal_sounds() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog),
            Box::new(Cat),
        ];
        // This test just ensures that the function runs without panicking
        animal_sounds(animals);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. Rust 공식 문서 - 트레이트: https://doc.rust-lang.org/book/ch10-02-traits.html
2. Rust by Example - 트레이트: https://doc.rust-lang.org/rust-by-example/trait.html
3. The Rust Programming Language (2nd Edition) by Steve Klabnik and Carol Nichols
4. Programming Rust (2nd Edition) by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall
5. Rust RFC - 트레이트 객체의 객체 안전성: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
6. Rust 블로그 - GAT 안정화: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
7. Rust 공식 포럼 - 트레이트 관련 토론: https://users.rust-lang.org/c/rust-users/10
8. Rust 공식 문서 - const trait: https://doc.rust-lang.org/stable/reference/const_trait_impl.html
