# Crate와 Module: Rust의 코드 구조화와 모듈화 전략

## 학습 목표
- Rust의 crate와 module 시스템을 이해한다.
- 코드 구조화와 모듈화의 중요성을 인식한다.
- 실제 프로젝트에서 crate와 module을 효과적으로 사용하는 방법을 학습한다.
- 소프트웨어 공학 관점에서 모듈화 전략을 이해한다.

## 크레이트 (Crate)

크레이트는 Rust의 컴파일 단위이다. 하나의 크레이트는 모듈들의 트리를 형성하며, 이 트리는 라이브러리나 실행 파일을 생성한다.

크레이트는 두 가지 형태가 있다:
1. 바이너리 크레이트: 실행 파일을 만드는 크레이트
2. 라이브러리 크레이트: 다른 프로그램에서 사용할 수 있는 코드를 포함하는 크레이트

### 바이너리 크레이트 생성하기

```bash
cargo new my_project
cd my_project
```

이렇게 하면 `src/main.rs` 파일이 생성되며, 이 파일이 크레이트의 루트가 된다.

### 라이브러리 크레이트 생성하기

```bash
cargo new my_library --lib
cd my_library
```

이 경우 `src/lib.rs` 파일이 생성되며, 이 파일이 라이브러리 크레이트의 루트가 된다.

## 모듈 (Module)

모듈은 코드를 그룹화하고 가독성과 재사용성을 높이는 데 사용된다. 

### 모듈 정의하기

`src/lib.rs` 또는 `src/main.rs`에서 모듈을 정의할 수 있다:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

### 모듈 트리 구조

모듈은 트리 구조를 형성한다. 크레이트 루트에서 시작하여 하위 모듈로 뻗어나간다.

```
crate
 └── front_of_house
     └── hosting
         └── add_to_waitlist
```

### 모듈과 파일 구조

모듈은 다른 파일로 분리할 수 있다. 예를 들어:

`src/lib.rs`:
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`src/front_of_house.rs`:
```rust
pub mod hosting;
```

`src/front_of_house/hosting.rs`:
```rust
pub fn add_to_waitlist() {}
```

이렇게 하면 코드를 논리적으로 구성하고 관리하기 쉬워진다.

## 외부 크레이트 사용하기

외부 크레이트를 사용하려면 `Cargo.toml` 파일에 의존성을 추가해야 한다.

1. `Cargo.toml`에 의존성 추가:

```toml
[dependencies]
rand = "0.8.5"
```

2. 코드에서 사용:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("비밀 번호: {}", secret_number);
}
```

이렇게 하면 `rand` 크레이트의 기능을 프로젝트에서 사용할 수 있다.


### Cargo.toml과 의존성 관리
`Cargo.toml` 파일은 프로젝트의 메타데이터와 의존성을 정의한다.

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

### 가시성과 캡슐화
Rust의 모든 항목은 기본적으로 private이다. `pub` 키워드를 사용하여 public으로 만들 수 있다.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 퍼블릭이 아님! 
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries"); // 컴파일 에러!
}
```

## 소프트웨어 공학 관점에서의 모듈화 전략

### 1. 단일 책임 원칙 
각 모듈은 하나의 명확한 책임만을 가져야 한다. 이는 코드의 응집도를 높이고 결합도를 낮춘다.
기억하자. 각 모듈은 하나의 특정 기능이나 목적에 집중해야 한다.
```rust
// auth.rs
pub mod auth {
    pub fn login() { /* ... */ }
    pub fn logout() { /* ... */ }
}

// database.rs
pub mod database {
    pub fn connect() { /* ... */ }
    pub fn query() { /* ... */ }
}

// business_logic.rs
pub mod business_logic {
    pub fn process_data() { /* ... */ }
}
```

### 2. 의존성 역전 원칙
추상화 계층은 시스템을 여러 층으로 구성하여, 각 층이 아래 층의 세부사항을 숨기고 위 층에 추상화된 인터페이스를 제공하는 전략이다.
즉, 시스템을 논리적 계층으로 나눈다 (예: UI, 비즈니스 로직, 데이터 접근).
각 계층은 아래 계층에 의존하되, 구체적인 구현보다는 추상화된 인터페이스(`trait`)에 의존한다.

예를 들어, 고수준 모듈을 비즈니스 로직(사용자 관리 시스템), 저수준 모듈을 데이터베이스 연결 모듈이라고 하자. 만일 비즈니스 로직이 데이터베이스 모듈에 직접 의존한다면,데이터베이스를 바꿀 때, 비즈니스 로직도 따라서 바뀌어야한다. 

따라서 UserManagement는 구체적인 Database가 아닌 DataStorage 트레이트에 의존해야한다.
Database도 DataStorage 트레이트를 구현한다.
따라서 두 모듈 모두 추상화(DataStorage)에 의존하게 된다.

```rust
// data_access.rs
pub trait DataAccess {
    fn fetch_data(&self) -> Vec<String>;
}

// business_logic.rs
pub struct BusinessLogic<T: DataAccess> {
    data_access: T,
}

impl<T: DataAccess> BusinessLogic<T> {
    pub fn process(&self) {
        let data = self.data_access.fetch_data();
        // 데이터 처리 로직
    }
}

// ui.rs
pub struct UI<T: BusinessLogic<dyn DataAccess>> {
    logic: T,
}

impl<T: BusinessLogic<dyn DataAccess>> UI<T> {
    pub fn display(&self) {
        // UI 표시 로직
    }
}
```

### 3. 정보 은닉 
정보 은닉은 모듈의 내부 구현 세부사항을 숨기고 잘 정의된 인터페이스만을 외부에 노출하는 원칙이다.
즉, 모듈 내부의 구현 세부사항은 private으로 유지한다.
필요한 인터페이스만 public으로 노출한다.

```rust
Copypub mod user_management {
    struct User {
        id: u64,
        name: String,
    }

    impl User {
        fn new(name: String) -> Self {
            // 내부 구현
        }
    }

    pub fn create_user(name: String) -> u64 {
        let user = User::new(name);
        // 사용자 생성 로직
        user.id
    }
}
```


## 왜 Rust의 기본 가시성은 Private인가?

Rust에서 모든 항목의 기본 가시성이 private인 이유는 다음과 같다:

1. **캡슐화 강화**: 명시적으로 public으로 선언하지 않은 항목은 모듈 외부에서 접근할 수 없다. 이는 모듈의 내부 구현을 숨기고, 의도하지 않은 외부 사용을 방지한다.

2. **인터페이스 안정성**: public 인터페이스를 신중하게 설계하도록 유도한다. 모든 항목을 public으로 만들면 나중에 변경하기 어려워질 수 있다.

3. **실수 방지**: 의도치 않게 내부 구현 세부사항을 노출하는 것을 방지한다.

4. **점진적 공개**: 필요에 따라 항목을 점진적으로 public으로 만들 수 있다. 이는 "최소 권한의 원칙"과 일맥상통한다.

