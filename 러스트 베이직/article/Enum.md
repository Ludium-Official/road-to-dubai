# 열거형과 패턴 매칭

## 학습 목표
- 열거형의 개념과 사용법을 이해한다.
- 열거형과 구조체의 차이점을 파악한다.
- Option 열거형의 필요성과 사용법을 익힌다.
- 열거형의 내부 구현과 메모리 사용을 이해한다.

## 열거형 정의하기

열거형은 어떤 값이 여러 개의 가능한 값의 집합 중 하나라는 것을 나타내는 방법을 제공한다. 예를 들어, IP 주소는 IPv4와 IPv6 두 종류가 있다. Rust에서는 이를 다음과 같이 표현할 수 있다:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```
V4,V6은 열거형의 Variant라고 한다. 
이제 IpAddrKind는 코드 어디에서나 사용할 수 있는 커스텀 데이터 타입이 되었다.
이 코드로 알 수 있듯, 열거형 배리언트에는 어떤 종류의 데이터라도 넣을 수 있다. 문자열, 숫자 타입, 구조체 등은 물론, 다른 열거형마저도 포함할 수 있다.

## 열거형 값

열거형의 인스턴스는 다음과 같이 생성할 수 있다:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

열거형을 함수의 매개변수로 사용할 수도 있다:

```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

## 열거형 vs 구조체

구조체 사용 예시 
```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```
열거형 사용 예시
```rust
enum IpAddr {
    V4(String),
    V6(String),
} 

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

이 방식은 구조체를 사용하는 것보다 더 간결하다. 또한, 각 배리언트는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // address 하나의 변수에 바인딩 된 것보다 더 자유로움
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## 열거형 메서드

구조체와 마찬가지로 열거형에도 `impl` 블록을 사용하여 메서드를 정의할 수 있다:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메서드 본문
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Option 열거형

Rust의 표준 라이브러리에는 `Option` 열거형이 정의되어 있다. 이 열거형은 널 값의 개념을 대체하는 데 사용된다.

`Option`의 필요성
Rust는 null 값을 직접적으로 지원하지 않는다. 대신 Option<T>를 사용하여 값의 존재 여부를 표현한다. 이는 다음과 같은 이점을 제공한다:

명시적인 null 체크: 개발자가 null 가능성을 `명시적으로` 처리해야 한다.
타입 안정성: null 참조로 인한 런타임 에러를 컴파일 타임에 방지할 수 있다.
의도의 명확한 표현: 어떤 값이 없을 수 있다는 것을 타입 시스템을 통해 `명시적으로` 표현한다.

`Option`은 열거형 중 하나로, 다음과 같이 구현되어있다. :

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>`를 사용하면 값이 있거나 없을 수 있는 상황을 안전하게 처리할 수 있다:

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

`Option<T>`와 `T`는 다른 타입이므로, 컴파일러는 `Option<T>` 값을 유효한 값처럼 사용하지 못하도록 한다. 이는 null로 인한 오류를 방지하는 데 도움이 된다.

`Option`의 구성요소들에 대해서 알아보자. 

Some(T): 값이 존재함을 나타내며, 그 값을 포함한다.
None: 값이 없음을 나타낸다.

Some은 값이 존재할 때 사용된다.
```rust
Copylet x: Option<i32> = Some(5);
let name: Option<String> = Some(String::from("Alice"));
```
Some을 사용할 때의 이점:

값의 존재를 명시적으로 표현한다.
타입 안정성을 제공한다.
컴파일러가 모든 경우를 처리했는지 확인할 수 있게 한다.

None은 값이 없음을 나타낸다.
```rust
Copylet y: Option<i32> = None;
let empty_name: Option<String> = None;
```
None을 사용할 때의 이점:

null 대신 사용되어 null 참조 오류를 방지한다.
값의 부재를 명시적으로 처리하도록 강제한다.
컴파일 시점에 값의 부재 가능성을 체크할 수 있게 한다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new enums_demo`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip_addr(&home);
    print_ip_addr(&loopback);
}

fn print_ip_addr(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 Address: {}", addr),
    }
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
    fn test_ipv4() {
        let home = IpAddr::V4(127, 0, 0, 1);
        match home {
            IpAddr::V4(a, b, c, d) => {
                assert_eq!(a, 127);
                assert_eq!(b, 0);
                assert_eq!(c, 0);
                assert_eq!(d, 1);
            },
            _ => panic!("Expected IPv4"),
        }
    }

    #[test]
    fn test_ipv6() {
        let loopback = IpAddr::V6(String::from("::1"));
        match loopback {
            IpAddr::V6(addr) => assert_eq!(addr, "::1"),
            _ => panic!("Expected IPv6"),
        }
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 모든 테스트가 통과하면 예제 코드가 올바르게 작성되었음을 확인할 수 있다.

이렇게 Rust의 열거형과 패턴 매칭에 대해 알아보았다. 열거형은 Rust의 강력한 기능 중 하나로, 다양한 상황에서 유용하게 사용될 수 있다. 특히 `Option<T>`를 통한 널 값 처리는 프로그램의 안정성을 크게 향상시킬 수 있다.