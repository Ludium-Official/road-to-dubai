# Rust의 Struct

Struct는 Rust에서 사용자 정의 데이터 타입을 만드는 핵심적인 방법이야. 여러 관련된 값들을 하나의 의미 있는 그룹으로 묶을 수 있어.

## Struct 정의하기

struct를 정의하는 방법은 간단해. `struct` 키워드를 사용하고, 그 뒤에 struct의 이름과 중괄호 안에 필드들을 나열하면 돼.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

이렇게 하면 `User`라는 이름의 struct가 만들어졌어. 이 struct는 `username`, `email`, `sign_in_count`, `active`라는 필드를 가지고 있지.

## Struct 인스턴스 생성하기

struct를 정의했다면, 이제 이를 사용해 인스턴스를 만들 수 있어:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

여기서 주의할 점은 struct의 모든 필드에 값을 지정해야 한다는 거야. 그리고 순서는 struct를 정의할 때와 달라도 괜찮아.

## 필드 값 변경하기

만약 struct 인스턴스가 가변(mutable)이라면, 점(.) 표기법을 사용해 특정 필드의 값을 변경할 수 있어:

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

## 필드 초기화 축약법

함수의 매개변수 이름과 struct의 필드 이름이 같을 때, 필드 초기화 축약법을 사용할 수 있어:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

여기서 `email`과 `username`은 축약해서 썼어. 이렇게 하면 코드가 더 간결해지지.

## Struct 업데이트 문법

기존의 struct 인스턴스로부터 대부분의 값을 가져오면서 일부만 변경하고 싶을 때 사용할 수 있는 문법이야:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

`..user1`은 나머지 필드들을 `user1`에서 가져온다는 뜻이야.

## 튜플 struct

필드에 이름이 없는 struct도 만들 수 있어. 이걸 튜플 struct라고 불러:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

`Color`와 `Point`는 다른 타입이지만, 둘 다 세 개의 `i32` 값을 가지고 있어.

## Unit-Like Struct

필드가 하나도 없는 struct도 정의할 수 있어. 이런 struct를 unit-like struct라고 불러:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

이런 struct는 특정 타입에 대한 트레이트를 구현하고 싶지만, 타입에 저장할 데이터가 없을 때 유용해.

이렇게 Rust의 struct에 대해 알아봤어. struct는 Rust에서 데이터를 조직화하는 강력한 도구야. 실제로 코드를 작성하면서 struct를 사용해보면 더 깊이 이해할 수 있을 거야. 직접 해보는 걸 추천해!
