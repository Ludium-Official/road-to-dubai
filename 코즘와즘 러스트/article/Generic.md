# 제네릭

제네릭은 Rust의 강력한 기능 중 하나야. 제네릭을 사용하면 구체적인 타입 대신 추상적인 스탠드인(stand-in)을 사용해서 코드를 작성할 수 있어. 이렇게 하면 코드의 중복을 줄이고 다양한 상황에서 재사용할 수 있는 유연한 코드를 만들 수 있지.

## 함수 정의에서의 제네릭

제네릭을 사용하는 가장 간단한 방법은 함수에 적용하는 거야. 여기 두 개의 숫자 중 큰 값을 반환하는 함수가 있어:

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

이 함수는 `T` 타입의 슬라이스를 받아서 같은 타입의 참조를 반환해. `T: std::cmp::PartialOrd`는 `T`가 비교 가능해야 한다는 제약을 나타내. 이 함수는 정수 리스트와 문자 리스트 모두에 사용할 수 있어.

## 구조체 정의에서의 제네릭

구조체 정의에서도 제네릭을 사용할 수 있어:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

이 `Point<T>` 구조체는 `x`와 `y`가 같은 타입 `T`를 가져. 이를 통해 정수형 좌표와 부동소수점 좌표를 모두 표현할 수 있지.

다른 타입을 사용하고 싶다면 이렇게 할 수 있어:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## 열거형 정의에서의 제네릭

`Option<T>`와 `Result<T, E>`처럼 열거형에서도 제네릭을 사용할 수 있어:

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

## 메서드 정의에서의 제네릭

메서드를 정의할 때도 제네릭을 사용할 수 있어:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

여기서 `impl<T>`는 우리가 제네릭 타입 `T`에 대한 메서드를 구현하고 있다는 걸 나타내. 

## 성능에 대해서

Rust의 제네릭은 컴파일 타임에 단형화(monomorphization)라는 과정을 거쳐. 이 과정에서 제네릭 코드는 실제 사용되는 구체적인 타입으로 된 코드로 변환돼. 이 덕분에 제네릭 사용으로 인한 런타임 비용이 없어!

```rust
let integer = Some(5);
let float = Some(5.0);
```

이 코드는 컴파일 시에 대략 이렇게 변환돼:

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

이렇게 Rust의 제네릭에 대해 알아봤어. 제네릭을 사용하면 코드의 중복을 줄이고 다양한 타입에 대해 동작하는 유연한 코드를 작성할 수 있어. 게다가 Rust의 단형화 덕분에 런타임 성능 저하 없이 이런 유연성을 얻을 수 있지!
