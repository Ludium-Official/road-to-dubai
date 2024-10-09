# Rust의 함수 (Function)

## 학습 목표
- Rust 함수의 내부 동작 원리를 이해한다.
- 함수와 소유권(Ownership) 간의 관계를 파악한다.
- 정적 디스패치와 동적 디스패치의 차이점과 사용 사례를 학습한다.
- 함수 포인터와 클로저의 개념과 사용법을 익힌다.

## 함수의 내부 동작

Rust 함수는 컴파일 시점에 정적으로 디스패치되며, 이는 함수 호출의 오버헤드를 최소화한다. 함수 호출 시 다음과 같은 과정이 일어난다:

1. 스택 프레임 생성: 함수 호출 시 새로운 스택 프레임이 생성된다.
2. 매개변수 복사: 함수 매개변수가 스택에 복사된다.
3. 제어 흐름 이동: 프로그램 카운터가 함수의 시작 주소로 이동한다.
4. 함수 실행: 함수 본문이 실행된다.
5. 반환 값 처리: 반환 값이 있다면 지정된 레지스터나 스택에 저장된다.
6. 스택 프레임 정리: 함수 종료 시 스택 프레임이 제거된다.

예시:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(5, 3);
    println!("Result: {}", result);
}
```

이 코드의 `add` 함수 호출은 다음과 같이 처리된다:

1. `add` 함수를 위한 새 스택 프레임 생성
2. `a`와 `b` 매개변수 값(5와 3)을 스택에 복사
3. `add` 함수 본문 실행
4. 결과 값(8)을 반환 레지스터에 저장
5. `add` 함수의 스택 프레임 제거
6. `main` 함수에서 반환 값을 `result` 변수에 저장

## 함수와 소유권

Rust의 소유권 시스템은 함수 호출에도 적용된다. 함수에 값을 전달할 때 소유권이 이동하거나 대여될 수 있다.

### 소유권 이동
```rust
fn take_ownership(s: String) {
    println!("{}", s);
} // 여기서 s가 드롭됨

fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // 컴파일 에러: s의 소유권이 이동됨
}
```

이 예제에서 `s`의 소유권은 `take_ownership` 함수로 이동한다. 함수 호출 후에는 `main`에서 `s`를 더 이상 사용할 수 없다.

### 참조 대여
```rust
fn borrow(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    borrow(&s);
    println!("{}", s); // 정상 작동: s는 여전히 유효함
}
```

여기서는 `s`의 참조만 `borrow` 함수에 전달되므로, `main`에서 계속 `s`를 사용할 수 있다.

## 정적 디스패치 vs 동적 디스패치

Rust는 기본적으로 정적 디스패치를 사용하지만, 트레이트 객체를 통해 동적 디스패치도 지원한다.

### 정적 디스패치
컴파일 시점에 어떤 함수가 호출될지 결정된다. 이는 제로 비용 추상화의 핵심이다.

```rust
fn static_dispatch<T: Display>(t: T) {
    println!("{}", t);
}

fn main() {
    static_dispatch("hello");
    static_dispatch(5);
}
```

이 코드에서 `static_dispatch` 함수는 컴파일 시점에 각 타입(`&str`과 `i32`)에 대해 특수화된다.

### 동적 디스패치
런타임에 어떤 메서드가 호출될지 결정된다. 이는 트레이트 객체를 통해 구현된다.

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    animal_sound(&dog);
    animal_sound(&cat);
}
```

여기서 `animal_sound` 함수는 런타임에 실제 객체의 `make_sound` 메서드를 호출한다.

## 함수 포인터와 클로저

### 함수 포인터
함수 포인터를 사용하면 함수를 값으로 전달할 수 있다.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn main() {
    let result = do_twice(add_one, 5);
    println!("Result: {}", result); // 출력: Result: 7
}
```

### 클로저
클로저는 환경을 캡처할 수 있는 익명 함수다.

```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
```

클로저는 내부적으로 세 가지 트레이트 중 하나로 구현된다: `Fn`, `FnMut`, `FnOnce`. 컴파일러는 클로저가 환경을 어떻게 캡처하는지에 따라 적절한 트레이트를 선택한다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_functions`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
fn main() {
    // 정적 디스패치 예제
    println!("Static dispatch:");
    static_dispatch("hello");
    static_dispatch(5);

    // 동적 디스패치 예제
    println!("\nDynamic dispatch:");
    let dog = Dog;
    let cat = Cat;
    animal_sound(&dog);
    animal_sound(&cat);

    // 함수 포인터 예제
    println!("\nFunction pointer:");
    let result = do_twice(add_one, 5);
    println!("Result: {}", result);

    // 클로저 예제
    println!("\nClosure:");
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("Is y equal to x? {}", equal_to_x(y));
}

fn static_dispatch<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
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
    fn test_static_dispatch() {
        static_dispatch("test");
        static_dispatch(10);
        // 컴파일되면 성공
    }

    #[test]
    fn test_dynamic_dispatch() {
        let dog = Dog;
        let cat = Cat;
        animal_sound(&dog);
        animal_sound(&cat);
        // 컴파일되면 성공
    }

    #[test]
    fn test_function_pointer() {
        assert_eq!(do_twice(add_one, 5), 7);
    }

    #[test]
    fn test_closure() {
        let x = 4;
        let equal_to_x = |z| z == x;
        assert!(equal_to_x(4));
        assert!(!equal_to_x(5));
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## 참고 자료

1. Rust 공식 문서 - 함수: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
2. Rust 공식 문서 - 클로저: https://doc.rust-lang.org/book/ch13-01-closures.html
3. Rust 공식 문서 - 트레이트 객체: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
4. "Programming Rust" by Jim Blandy and Jason Orendorff, O'Reilly Media
5. "The Rust Programming Language" by Steve Klabnik and Carol Nichols: https://doc.rust-lang.org/book/
6. Rust RFC 0255 - Object Safety: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md