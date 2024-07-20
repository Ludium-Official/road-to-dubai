# Ownership

## 학습 목표
- Rust의 Ownership 시스템의 탄생 배경과 중요성을 이해한다.
- Ownership의 규칙과 작동 방식을 숙지한다.
- 저비용 추상화와 Ownership의 관계를 파악한다.
- 운영체제 관점에서 Ownership의 장점을 이해한다.
- 고급 Ownership 기법을 학습하고 활용할 수 있다.

## Ownership의 탄생 배경

Rust의 Ownership 시스템은 메모리 안전성과 동시성 프로그래밍의 용이성을 동시에 달성하기 위해 탄생했다. 기존 프로그래밍 언어들이 가비지 컬렉터(GC)를 통해 이 문제를 해결하려 했던 것과 달리, Rust는 컴파일 시점에 메모리 관리를 검사하는 새로운 접근 방식을 택했다.

### 저비용 추상화

Rust의 Ownership 시스템은 "제로 비용 추상화(Zero-cost abstractions)"라는 개념을 구현한다. 이는 런타임 오버헤드 없이 고수준의 추상화를 제공한다는 의미이다. Ownership 규칙은 컴파일 시점에 적용되므로, 실행 시 추가적인 비용이 발생하지 않는다.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 이 for 루프는 C++의 수동 반복자 사용과 동일한 성능을 보인다
    for i in v {
        println!("{}", i);
    }
}
```

이 코드에서 `for` 루프는 고수준의 추상화를 제공하면서도, 수동으로 작성한 저수준 코드와 동일한 성능을 보인다.

## Ownership의 핵심 개념

Rust의 Ownership 시스템은 메모리 관리를 안전하고 효율적으로 수행하기 위한 핵심 메커니즘이다. 이 시스템의 주요 개념들을 자세히 살펴보자.

### 1. 스택과 힙

Rust에서 메모리 관리를 이해하기 위해서는 스택(Stack)과 힙(Heap)의 차이를 아는 것이 중요하다.

- **스택**: 정해진 크기의 데이터를 저장하는 빠른 메모리 영역. 함수 호출 시 지역 변수들이 여기에 저장된다.
- **힙**: 크기가 가변적이거나 컴파일 시점에 크기를 알 수 없는 데이터를 저장하는 영역. 런타임에 메모리 할당이 이루어진다.

```rust
fn main() {
    let x = 5; // 스택에 저장
    let y = Box::new(10); // 힙에 저장, y는 힙의 데이터를 가리키는 포인터
}
```

### 2. 이동 (Move)

Rust에서 값을 다른 변수에 할당하면, 기본적으로 '이동'이 발생한다. 이는 힙에 할당된 데이터의 소유권이 이전되는 것을 의미한다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 이동

    // println!("{}", s1); // 컴파일 에러: s1은 이미 이동됨
    println!("{}", s2); // 정상 작동
}
```

### 3. 복사 (Copy)

일부 타입(주로 스택에 저장되는 기본 타입들)은 `Copy` 트레이트를 구현하고 있어, 값을 복사한다.

```rust
fn main() {
    let x = 5;
    let y = x; // x의 값이 y로 복사됨

    println!("x = {}, y = {}", x, y); // 둘 다 사용 가능
}
```

### 4. 소유권과 함수

함수에 값을 전달할 때도 소유권 규칙이 적용된다.

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s의 소유권이 함수로 이동
    // println!("{}", s); // 컴파일 에러: s는 이미 이동됨

    let x = 5;
    makes_copy(x); // x의 값이 복사됨
    println!("{}", x); // 여전히 사용 가능
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string이 스코프를 벗어나고 drop 됨

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer가 스코프를 벗어남, 특별한 일은 없음
```

### 5. 참조와 대여 (References and Borrowing)

값의 소유권을 이전하지 않고 참조를 사용하여 값을 '대여'할 수 있다.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 6. 가변 참조 (Mutable References)

가변 참조를 사용하면 대여한 값을 수정할 수 있다. 단, 동시에 하나의 가변 참조만 허용된다.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // "hello, world" 출력
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 7. 수명 (Lifetimes)

Rust의 컴파일러는 참조의 유효 범위를 추적하기 위해 수명 개념을 사용한다. 대부분의 경우 이는 암시적으로 처리되지만, 때로는 명시적으로 지정해야 한다.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

이 함수에서 `'a`는 수명 매개변수로, 반환되는 참조가 입력 매개변수의 수명과 연관되어 있음을 나타낸다.

## Ownership 두들겨보기

다음 코드를 VSCode에서 두들겨보며 Ownership의 개념을 실제로 체험해보자:

```rust
fn main() {
    // 1. 이동 (Move)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 주석 해제 시 컴파일 에러

    // 2. 복사 (Copy)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 3. 소유권과 함수
    let s3 = String::from("world");
    takes_ownership(s3);
    // println!("{}", s3); // 주석 해제 시 컴파일 에러

    // 4. 참조와 대여
    let s4 = String::from("hello world");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);

    // 5. 가변 참조
    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("Changed string: {}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

이 실습을 통해 Rust의 Ownership 시스템이 실제로 어떻게 작동하는지 이해할 수 있을 것이다.

## Ownership 규칙

Rust의 Ownership 시스템은 다음 세 가지 규칙을 따른다:

1. Rust에서 각각의 값은 해당 값의 owner라고 불리는 변수를 가진다.
2. 한 번에 하나의 owner만 존재할 수 있다.
3. owner가 스코프 밖으로 벗어나면, 값은 삭제된다.

### 예시

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); // 컴파일 에러: s1의 값이 s2로 이동됨
    println!("{}", s2); // 정상 작동
}
```

이 예시에서 `s1`의 값이 `s2`로 이동되어, `s1`은 더 이상 유효하지 않다. 이를 통해 Rust는 이중 해제 오류와 메모리 누수를 방지한다.

## 운영체제 관점에서의 장점

Rust의 Ownership 시스템은 운영체제 개발에 있어 여러 장점을 제공한다:

1. **메모리 안전성**: 버퍼 오버플로우, 댕글링 포인터 등의 메모리 관련 버그를 컴파일 시점에 방지한다.
2. **동시성 안전성**: 데이터 레이스와 같은 동시성 관련 문제를 컴파일 시점에 검출한다.
3. **리소스 관리**: 파일 핸들, 네트워크 소켓 등의 시스템 리소스를 안전하게 관리할 수 있다.
4. **성능**: GC 없이도 메모리 안전성을 보장하여, 예측 가능한 성능을 제공한다.

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

이 예시에서 `File`은 자동으로 닫히며, 에러 처리도 명시적으로 이루어진다. 이는 운영체제 수준의 리소스 관리에 매우 유용하다.

## 고급 Ownership 기법

### 함수 포인터와 클로저

Rust에서는 함수 포인터와 클로저를 통해 고급 Ownership 기법을 구현할 수 있다. 다음은 주어진 코드 예시이다:

```rust
fn wrap_logging<F>(target: F) -> impl Fn()
where
    F: Fn()
{
    move || {
        println!("Logging Start");
        target();
        println!("Logging End");
    }
}
```

이 함수는 다른 함수(`target`)를 인자로 받아, 로깅 기능을 추가한 새로운 클로저를 반환한다. `FnOnce`를 사용함으로써, `target` 함수가 한 번만 호출되도록 보장한다.
여기서 소유권 관점에서 주목할 점은 target도 변수로, 소유권에 영향을 받는다. 
따라서, 해당 함수가 끝나는 지점에서, target 변수도 해제되는데, 이 때, `move`가 없다면.. Rust에서 캡쳐 명시자가 없다면 기본적으로 Fn(불변 참조)로 동작하는데, 이 때 wrap_logging 함수의 반환값인 Fn 클로저가 target을 여전히 참조하고 있으므로 댕글링 포인터가 발생할 수 있어 컴파일 에러가 나타난다.
그러므로, move를 통해서 불변 참조 캡쳐가 아닌 소유권 캡쳐를 통해서 target의 소유권을 반환되는 함수 안으로 가져와야한다. 

### 사용 예시

```rust
fn main() {
    let print_hello = || println!("Hello, World!");
    let logged_hello = wrap_logging(print_hello);
    logged_hello();
}
```

이 코드의 출력:
```
Logging Start
Hello, World!
Logging End
```

이 예시는 Rust의 Ownership 시스템이 어떻게 고급 프로그래밍 패턴을 안전하게 구현할 수 있게 하는지 보여준다. 이는 `함수 생성 함수` 패턴이라고 볼 수 있다. 

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_ownership`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
fn wrap_logging<F>(target: F) -> impl FnOnce()
where
    F: FnOnce()
{
    || {
        println!("Logging Start");
        target();
        println!("Logging End");
    }
}

fn main() {
    // 기본 Ownership 예제
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 컴파일 에러
    println!("{}", s2);

    // 고급 Ownership 예제
    let print_hello = || println!("Hello, World!");
    let logged_hello = wrap_logging(print_hello);
    logged_hello();
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
    fn test_ownership_move() {
        let s1 = String::from("hello");
        let s2 = s1;
        assert_eq!(s2, "hello");
        // s1은 이미 이동되어 사용할 수 없음
    }

    #[test]
    fn test_wrap_logging() {
        let mut output = Vec::new();
        let print_to_vec = || output.push("Hello, World!");
        let logged_print = wrap_logging(print_to_vec);
        logged_print();
        assert_eq!(output, vec!["Hello, World!"]);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다.

## Reference

1. Rust 공식 문서 - Ownership: [https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
2. Rust 공식 문서 - 저비용 추상화: [https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#zero-cost-abstraction](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#zero-cost-abstraction)
3. "Programming Rust" by Jim Blandy and Jason Orendorff, O'Reilly Media
4. "Rust in Action" by Tim McNamara, Manning Publications
5. Rust RFC 2094 - Non-lexical lifetimes: [https://rust-lang.github.io/rfcs/2094-nll.html](https://rust-lang.github.io/rfcs/2094-nll.html)
6. Rust Blog - Fearless Concurrency with Rust: [https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)