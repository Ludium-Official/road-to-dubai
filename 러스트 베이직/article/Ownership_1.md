# Rust의 Ownership: 탄생 배경과 중요성

Rust의 가장 독특하고 중요한 특징 중 하나인 ownership 시스템에 대해 알아보자. 이 시스템이 왜 만들어졌고, 어떤 점에서 중요한지 함께 살펴볼 거야.

## 1. Ownership의 탄생 배경

Rust를 만든 사람들은 두 가지 목표를 가지고 있었어: 
1) 메모리 안전성을 보장하는 것
2) 동시성 프로그래밍을 쉽게 만드는 것

이 두 가지 목표를 달성하면서도 프로그램의 성능을 희생하지 않아야 했지. 기존의 프로그래밍 언어들은 이 문제를 해결하기 위해 가비지 컬렉터(GC)를 사용했어. 하지만 GC는 프로그램의 성능에 영향을 미치는 단점이 있었지.

Rust는 이 문제를 ownership이라는 새로운 개념으로 해결했어. Ownership 시스템은 컴파일 시점에 메모리 관리를 검사하므로, 런타임 비용이 들지 않아. 이는 Rust가 안전성과 성능을 동시에 제공할 수 있게 해주는 핵심 메커니즘이야.

## 2. Ownership 규칙

Rust의 ownership 시스템은 다음과 같은 규칙을 따라:

1. Rust에서 각각의 값은 해당 값의 owner라고 불리는 변수를 가져.
2. 한 번에 하나의 owner만 존재할 수 있어.
3. owner가 스코프 밖으로 벗어나면, 값은 삭제돼.

이 규칙들이 어떻게 작동하는지 예제를 통해 살펴보자:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // 이 라인은 컴파일 에러를 발생시켜
}
```

이 코드에서 `s1`의 값은 `s2`로 이동(move)되었기 때문에, `s1`은 더 이상 유효하지 않아. 이렇게 함으로써 Rust는 이중 해제 오류를 방지하고, 메모리 누수를 막을 수 있어.

## 3. 힙 과 스택 

Rust의 ownership 시스템에 대한 설명을 하기 전에 힙 과 스택의 차이점에 대해서 알아보자  
![image](https://github.com/user-attachments/assets/483444c4-5339-46cf-b2cf-6f2ef18f68ee)  
메모리의 스택(stack) 영역은 함수의 호출과 관계되는 지역 변수와 매개변수가 저장되는 영역이야. 스택 영역은 함수의 호출과 함께 할당되며, 함수의 호출이 완료되면 소멸하지.  
스택 영역의 메모리는 위에 사진에서 볼 수 있듯이, 컴파일 타임에 메모리 양이 결정이 돼. 함수의 호출이 끝난 후에, 자동으로 소멸하기 때문에, ownership이 스택 메모리에 할당되는 변수에 있어서는 적용되지 않지.  

메모리의 힙(heap) 영역은 사용자가 직접 관리할 수 있는 ‘그리고 해야만 하는’ 메모리 영역이야. 컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 위해서는, 힙에 데이터를 저장할 수 있지. 힙 영역은 사용자에 의해 메모리 공간이 동적으로 할당되고 해제되는데 ownership은 여기서 나오는 오류들을 방지하고자 나온 개념이야.   

힙 데이터를 관리하는 것이 곧 소유권의 존재 이유임을 알게 되는 것은 이것이 어떤 방식으로 작동하는지 설명하는데 도움을 줄 수 있어. 

```rust
fn main() {
    let s = String::from("hello");  // 힙 메모리를 사용하는 s 스코프 진입

    takes_ownership(s);             // s가 scope를 벗어났기 때문에 소유권이 takes_ownership 으로 이전

    let x = 5;                      // 스택 메모리를 사용하는 x 스코프 진입

    makes_copy(x);                  // 스택 메모리를 사용하는 x이기 때문에 copy동작. 소유권 이전되지 않음.

} 

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} 

fn makes_copy(some_integer: i32) { 
    println!("{some_integer}");
}
```

위의 코드를 보면 s는 힙 메모리를 사용하기 때문에, ownership 이전이 발생하고 x는 스택 메모리르 사용하기 때문에 ownership이전이 발생하지 않는 것을 확인할 수 있지. 



## 3. 참조와 대여

값의 소유권을 이전하지 않고 값을 사용하는 방법으로 Rust는 '참조'를 제공해. 참조를 사용하면 값을 '대여(borrow)'할 수 있어.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("'{}의 길이는 {}입니다.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

여기서 `&s1`은 `s1`의 참조를 생성해. 함수는 `String`의 참조를 받아 길이를 계산하고, 소유권은 그대로 `main` 함수에 남아있어.

참조는 Rust가 소유권 이전 없이 데이터를 안전하게 공유할 수 있게 해줘. 이는 특히 큰 데이터 구조를 다룰 때 성능상 이점이 있어.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) -> {
    some_string.push_str(", world");
}
```

mutable 변수의 참조를 넘겨주기 위해서는 위와 이 mutable 참조를 사용해. 

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{} , {}", r1, r2);\
```
위의 코드는 에러가 발생하게 되는데, rust는 동일 scope안에서 mutable참조자는 하나만 만들 수 있게 하기 때문이야. 이를 통해 Data race로 버그/에러가 발생하는 것을 방지하지.

## 4. Dangling References

포인터를 사용하는 프로그래밍 언어에서는 댕글링 포인터(dangling pointer)를 실수로 만들기 쉬워. 댕글링 포인터란 이미 다른 누군가에게 할당되었을 수 있는 메모리 위치를 참조하는 포인터를 말하지. 

반면 Rust에서는 컴파일러가 참조가 절대 댕글링 참조가 되지 않도록 보장해. 어떤 데이터에 대한 참조를 가지고 있다면, 컴파일러는 그 참조가 스코프를 벗어나기 전에 데이터가 스코프를 벗어나지 않도록 보장해.

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```
rust 컴파일러는 댕글링 참조를 감지하고 컴파일 에러를 내보내서 메모리 안전성을 컴파일 시점에 보장하지. 


## 5. Slice 타입

마지막으로, Rust는 소유권을 가지지 않고 컬렉션의 일부분을 참조하는 slice라는 개념을 제공해.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    
    println!("첫 번째 단어: {}", word);
}
```

Slice를 사용하면 컬렉션의 일부분을 안전하게 참조할 수 있어, 더욱 유연한 코드를 작성할 수 있지.

이렇게 Rust의 ownership 시스템은 메모리 안전성을 보장하면서도 효율적인 메모리 관리를 가능하게 해. 처음에는 이해하기 어려울 수 있지만, 이 개념을 숙달하면 더 안전하고 효율적인 코드를 작성할 수 있을 거야. 다음 장에서는 Rust의 ownership에 대해 더 자세히 알아보자.
