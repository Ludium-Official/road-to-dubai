# Struct

## 1. Struct의 기본 개념

Struct는 Rust에서 사용자 정의 데이터 타입을 만드는 핵심적인 방법이다. 여러 관련된 값들을 하나의 의미 있는 그룹으로 묶을 수 있다.

### 1.1 Struct 정의하기

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

이 코드는 `User`라는 이름의 struct를 정의한다. 각 필드는 이름과 타입을 가진다. Rust에서 struct 필드의 순서는 중요하지 않지만, 메모리 레이아웃 최적화를 위해 필드 순서를 고려할 수 있다.

### 1.2 Struct 인스턴스 생성하기

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

struct의 모든 필드에 값을 지정해야 하며, 순서는 정의와 달라도 된다. 이는 Rust의 타입 안전성을 보장하는 방법 중 하나이다.

### 1.3 필드 초기화 축약법

함수의 매개변수 이름과 struct의 필드 이름이 같을 때, 필드 초기화 축약법을 사용할 수 있다:

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

이 축약법은 코드의 가독성을 높이고 반복을 줄인다.

### 1.4 Struct 업데이트 문법

기존 인스턴스의 대부분의 값을 재사용하면서 일부만 변경하고 싶을 때 사용한다:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

`..user1`은 명시되지 않은 나머지 필드들을 `user1`에서 가져온다. 이는 소유권 이전이 발생할 수 있음에 주의해야 한다.
자세하게 들여다보자면, Copy trait가 구현된 타입은 얕은 복사를 수행하며(바이트 단위 복사), 그렇지 않은 타입은 move 연산을 수행한다  

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        address: String::from("123 Main St"),
    };

    // person1의 일부 필드를 사용하여 person2를 생성
    let person2 = Person {
        age: 31,
        ..person1
    };

    // 여기서 소유권 문제가 발생합니다
    println!("person1: {:?}", person1);
    println!("person2: {:?}", person2);
}
```
이 코드를 실행하면 컴파일 에러가 발생합니다. 그 이유를 살펴보겠다:

String 타입의 소유권 이전:

person1의 name과 address 필드는 String 타입이다.
String은 Copy 트레이트를 구현하지 않았으므로, 값을 복사하는 대신 소유권이 이전된다.
..person1 문법을 사용하면 person1의 name과 address 필드의 소유권이 person2로 이전된다.

부분적 소유권 이전:

age 필드는 u32 타입으로, Copy 트레이트를 구현했기 때문에 복사된다.
따라서 person1.age는 여전히 사용 가능하다.

컴파일 에러:

println!("person1: {:?}", person1); 라인에서 에러가 발생한다.
person1의 name과 address 필드의 소유권이 이미 person2로 이전되었기 때문이다.


## 2. Struct의 고급 기능

### 2.1 메서드 정의

Struct에 메서드를 추가하는 것은 관련 기능을 그룹화하고 캡슐화하는 좋은 방법이다.

```rust
impl User {
    fn is_active(&self) -> bool {
        self.active
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn post_count(&self) -> u64 {
        // 가정: 이 사용자의 게시물 수를 반환
        42
    }
}
```

여기서 `&self`는 메서드가 struct 인스턴스를 불변 참조로 빌려온다는 의미이다. `&mut self`는 가변 참조를 의미한다.

### 2.2 연관 함수

`impl` 블록 내에서 `self` 매개변수를 갖지 않는 함수를 정의할 수 있다. 이를 연관 함수라고 하며, 주로 생성자로 사용된다.

```rust
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
}

// 사용 예
let user2 = User::new(String::from("user2"), String::from("user2@example.com"));
```

연관 함수는 `::` 구문을 사용하여 호출한다.

### 2.3 Derive 매크로

Rust는 `derive` 속성을 통해 struct에 자동으로 특정 트레이트를 구현할 수 있게 해준다.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

이 코드는 `Point` struct에 `Debug`, `Clone`, `PartialEq` 트레이트를 자동으로 구현한다. 이는 보일러플레이트 코드를 줄이고 생산성을 높인다.

- `Debug`: `println!("{:?}", point);`와 같이 디버그 출력을 가능하게 한다.
- `Clone`: `point.clone()`으로 깊은 복사를 가능하게 한다.
- `PartialEq`: `==` 연산자로 두 `Point` 인스턴스를 비교할 수 있게 한다.

## 3. Struct와 메모리

### 3.1 메모리 레이아웃

Struct의 메모리 레이아웃은 필드의 순서와 타입에 따라 결정된다. Rust는 필드 간 패딩을 추가하여 메모리 정렬을 최적화한다.

```rust
struct Foo {
    a: u8,
    b: u32,
    c: u8,
}
```

이 struct의 실제 메모리 크기는 12바이트가 될 수 있다 (1 + 3(패딩) + 4 + 1 + 3(패딩)). 이는 메모리 접근 효율성을 위해 4바이트 정렬을 하기 때문이다.

메모리 레이아웃을 최적화하려면 다음과 같이 필드를 재정렬할 수 있다:

```rust
struct OptimizedFoo {
    b: u32,
    a: u8,
    c: u8,
}
```

이제 `OptimizedFoo`의 크기는 6바이트가 된다 (4 + 1 + 1).

### 3.2 컴파일 시간 레이아웃 결정

Rust의 struct 레이아웃은 컴파일 시간에 결정된다. 이는 몇 가지 중요한 이점을 제공한다:

1. 런타임 오버헤드 없음: 필드 접근이 단순한 오프셋 계산으로 이루어진다.
2. 메모리 사용 예측 가능: 프로그램의 메모리 사용을 정확히 예측할 수 있다.
3. 캐시 친화적: 데이터 구조의 레이아웃을 알기 때문에 캐시 최적화가 가능하다.

예를 들어:

```rust
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };
    println!("Size of Point: {}", std::mem::size_of::<Point>());
}
```

이 코드는 컴파일 시간에 `Point` struct의 크기를 결정하고, 런타임에 추가 비용 없이 이 정보를 사용한다.


## 4. 고급 Struct 패턴

### 4.1 타입 상태 패턴

타입 상태 패턴은 컴파일 시간에 상태를 검사하여 런타임 오류를 방지하는 강력한 기법이다.

```rust
struct Open;
struct Closed;

struct Door<State> {
    state: std::marker::PhantomData<State>,
}

impl Door<Closed> {
    fn new() -> Self {
        Door { state: std::marker::PhantomData }
    }

    fn open(self) -> Door<Open> {
        println!("Opening the door");
        Door { state: std::marker::PhantomData }
    }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> {
        println!("Closing the door");
        Door { state: std::marker::PhantomData }
    }
}

fn main() {
    let door = Door::new();
    let open_door = door.open();
    // let still_open = open_door.open(); // 컴파일 에러: 이미 열린 문을 열 수 없음
    let closed_door = open_door.close();
}
```

이 패턴을 사용하면 열린 문을 열거나 닫힌 문을 닫는 등의 잘못된 동작을 컴파일 시간에 방지할 수 있다. `PhantomData`는 컴파일러에게 `State` 타입 매개변수가 사용되고 있음을 알려주는 역할을 한다.

### 4.2 빌더 패턴

빌더 패턴은 복잡한 객체의 생성을 단순화하고 명확하게 만든다. 특히 선택적 매개변수가 많은 경우에 유용하다.

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct UserBuilder {
    username: Option<String>,
    email: Option<String>,
    sign_in_count: Option<u64>,
    active: Option<bool>,
}

impl UserBuilder {
    fn new() -> UserBuilder {
        UserBuilder {
            username: None,
            email: None,
            sign_in_count: None,
            active: None,
        }
    }

    fn username(mut self, username: String) -> UserBuilder {
        self.username = Some(username);
        self
    }

    fn email(mut self, email: String) -> UserBuilder {
        self.email = Some(email);
        self
    }

    fn sign_in_count(mut self, count: u64) -> UserBuilder {
        self.sign_in_count = Some(count);
        self
    }

    fn active(mut self, active: bool) -> UserBuilder {
        self.active = Some(active);
        self
    }

    fn build(self) -> Result<User, &'static str> {
        let username = self.username.ok_or("Username is required")?;
        let email = self.email.ok_or("Email is required")?;

        Ok(User {
            username,
            email,
            sign_in_count: self.sign_in_count.unwrap_or(0),
            active: self.active.unwrap_or(false),
        })
    }
}

fn main() {
    let user = UserBuilder::new()
        .username("johndoe".to_string())
        .email("john@example.com".to_string())
        .active(true)
        .build();

    match user {
        Ok(u) => println!("User created: {:?}", u),
        Err(e) => println!("Error creating user: {}", e),
    }
}
```

이 패턴을 사용하면 복잡한 객체를 단계적으로 생성할 수 있으며, 필수 필드가 누락된 경우 컴파일 시간이 아닌 런타임에 오류를 발생시킬 수 있다.

### 4.3 뉴타입 패턴

뉴타입 패턴은 기존 타입을 감싸 새로운 타입을 만드는 방법이다. 이는 타입 안전성을 높이고 의미론적 명확성을 제공한다.

```rust
struct Meters(f64);
struct Kilometers(f64);

impl Meters {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }
}

impl Kilometers {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1000.0)
    }
}

fn main() {
    let distance = Meters(5000.0);
    let km_distance = distance.to_kilometers();
    println!("{} meters is {} kilometers", distance.0, km_distance.0);
}
```

이 패턴을 사용하면 단위 변환 오류를 방지하고 코드의 의도를 명확히 할 수 있다.


## 5. VSCode에서 실습

`src/main.rs` 파일에 다음 코드를 작성한다:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

struct RectangleBuilder {
    width: Option<u32>,
    height: Option<u32>,
}

impl RectangleBuilder {
    fn new() -> Self {
        RectangleBuilder {
            width: None,
            height: None,
        }
    }

    fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    fn build(self) -> Result<Rectangle, &'static str> {
        let width = self.width.ok_or("Width is required")?;
        let height = self.height.ok_or("Height is required")?;
        Ok(Rectangle { width, height })
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    let square = Rectangle::square(20);

    println!("rect1 is {:?}", rect1);
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square: {:?}", square);

    let rect4 = RectangleBuilder::new()
        .width(35)
        .height(55)
        .build();

    match rect4 {
        Ok(r) => println!("Built rectangle: {:?}", r),
        Err(e) => println!("Error building rectangle: {}", e),
    }

    let rect5 = RectangleBuilder::new()
        .width(40)
        .build();

    match rect5 {
        Ok(r) => println!("Built rectangle: {:?}", r),
        Err(e) => println!("Error building rectangle: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn area_calculation() {
        let rect = Rectangle::new(5, 5);
        assert_eq!(rect.area(), 25);
    }

    #[test]
    fn square_creation() {
        let square = Rectangle::square(5);
        assert_eq!(square.width, 5);
        assert_eq!(square.height, 5);
    }

    #[test]
    fn builder_success() {
        let rect = RectangleBuilder::new()
            .width(10)
            .height(20)
            .build();
        assert!(rect.is_ok());
        let rect = rect.unwrap();
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn builder_failure() {
        let rect = RectangleBuilder::new()
            .width(10)
            .build();
        assert!(rect.is_err());
    }
}
```

이 코드를 `src/main.rs` 파일에 작성한 후, 터미널에서 `cargo run`을 실행하여 프로그램을 실행하고, `cargo test`를 실행하여 테스트를 수행할 수 있다.