# Rust의 스마트 포인터 (Smart Pointers)

스마트 포인터는 Rust의 핵심 기능 중 하나야. 단순한 포인터보다 더 많은 기능을 제공하면서 메모리를 안전하고 효율적으로 관리할 수 있게 해줘. 여기서는 Rust의 주요 스마트 포인터들과 그 사용법, 그리고 내부 동작 원리에 대해 자세히 알아볼 거야.

## 1. 스마트 포인터란?

스마트 포인터는 포인터처럼 동작하지만 추가적인 기능과 메타데이터를 가진 데이터 구조야. Rust에서 스마트 포인터는 보통 구조체로 구현되며, `Deref`와 `Drop` 트레이트를 구현해.

- `Deref` 트레이트: 스마트 포인터가 참조하는 값에 접근할 수 있게 해줘.
- `Drop` 트레이트: 스마트 포인터가 스코프를 벗어날 때 실행될 코드를 정의해.

## 2. Box<T>

`Box<T>`는 가장 간단한 스마트 포인터야. 힙에 데이터를 할당하고, 그 데이터에 대한 포인터를 스택에 저장해.

### 사용 예시:
```rust
let b = Box::new(5);
println!("b = {}", b);
```

### 언제 사용할까?
1. 컴파일 타임에 크기를 알 수 없는 타입을 사용할 때
2. 큰 데이터를 복사하지 않고 소유권을 전달하고 싶을 때
3. 특정 타입을 구현하는 값을 소유하고 싶을 때, 정확한 타입은 신경 쓰지 않을 때

### 내부 동작:
`Box<T>`는 내부적으로 단순한 포인터야. `Deref` 트레이트를 구현해서 `*` 연산자로 내부 값에 접근할 수 있게 해주고, `Drop` 트레이트로 메모리를 해제해.

## 3. Rc<T>

`Rc<T>`는 "Reference Counting"의 약자로, 여러 소유자를 가질 수 있는 타입이야.

### 사용 예시:
```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);
let c = Rc::clone(&a);

println!("count after creating c = {}", Rc::strong_count(&a));
```

### 언제 사용할까?
1. 데이터를 여러 부분에서 읽기 전용으로 공유해야 할 때
2. 컴파일 타임에 어느 부분이 마지막으로 데이터를 사용할지 알 수 없을 때

### 내부 동작:
`Rc<T>`는 내부적으로 참조 카운트를 관리해. `clone`을 호출할 때마다 카운트가 증가하고, `Rc`가 스코프를 벗어날 때마다 감소해. 카운트가 0이 되면 데이터가 해제돼.

## 4. RefCell<T>

`RefCell<T>`는 실행 시간에 빌림 규칙을 확인하는 타입이야. 내부 가변성을 제공해.

### 사용 예시:
```rust
use std::cell::RefCell;

let data = RefCell::new(5);

{
    let mut m = data.borrow_mut();
    *m += 1;
}

println!("data: {:?}", data.borrow());
```

### 언제 사용할까?
1. 컴파일 타임에는 불변이지만 내부 값을 변경해야 할 때
2. 런타임에 빌림 규칙을 확인하고 싶을 때

### 내부 동작:
`RefCell<T>`는 내부적으로 빌림 상태를 추적해. `borrow`와 `borrow_mut` 메서드로 내부 값에 접근할 수 있고, 이 때 런타임 검사가 수행돼. 규칙을 위반하면 패닉이 발생해.

## 5. Arc<T>

`Arc<T>`는 "Atomically Reference Counted"의 약자로, 스레드 간에 안전하게 공유할 수 있는 타입이야.

### 사용 예시:
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

for _ in 0..3 {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        println!("{:?}", *data);
    });
}
```

### 언제 사용할까?
1. 여러 스레드 간에 데이터를 공유해야 할 때
2. 스레드 안전성이 필요한 상황에서 `Rc<T>` 대신 사용

### 내부 동작:
`Arc<T>`는 `Rc<T>`와 비슷하지만, 원자적 연산을 사용해 참조 카운트를 관리해. 이로 인해 스레드 간 데이터 레이스를 방지할 수 있어.

## 6. Mutex<T>와 RwLock<T>

이 두 타입은 동시성 상황에서 데이터에 대한 접근을 동기화하는 데 사용돼.

### Mutex<T> 사용 예시:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

### 언제 사용할까?
1. 여러 스레드에서 데이터를 읽고 쓰는 상황을 동기화해야 할 때
2. 상호 배제(mutual exclusion)가 필요할 때

### 내부 동작:
`Mutex<T>`는 내부적으로 락(lock)을 관리해. `lock` 메서드를 호출하면 락을 획득하고 내부 데이터에 접근할 수 있는 가드를 반환해. 가드가 스코프를 벗어나면 자동으로 락이 해제돼.

## 7. 커스텀 스마트 포인터 만들기

필요에 따라 자신만의 스마트 포인터를 만들 수도 있어. 주로 `Deref`와 `Drop` 트레이트를 구현하게 돼.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

이 예제에서 `MyBox`는 `Deref`와 `Drop` 트레이트를 구현해서 스마트 포인터처럼 동작해.

## 결론

스마트 포인터는 Rust의 강력한 기능 중 하나야. 메모리 관리를 안전하고 효율적으로 할 수 있게 해주면서, 다양한 상황에 맞는 유연한 해결책을 제공해. 
`Box<T>`, `Rc<T>`, `RefCell<T>`, `Arc<T>`, `Mutex<T>` 등 각각의 스마트 포인터는 특정 상황에 맞게 설계됐어. 
이들을 적절히 사용하면 안전하고 효율적인 Rust 프로그램을 작성할 수 있어. 
