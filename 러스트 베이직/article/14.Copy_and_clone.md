# Copy와 Clone

## 학습 목표
- Copy와 Clone 트레이트의 개념과 차이점을 이해한다.
- Copy와 Clone 트레이트의 구현 조건을 파악한다.
- 소유권 모델과 Copy, Clone의 관계를 이해한다.
- Copy와 Clone을 적절히 사용하는 방법을 익힌다.

## Copy 트레이트

Copy 트레이트는 값을 복제하는 가장 간단한 방법을 제공한다. 이는 비트 단위의 복사만으로 충분한 타입에 사용된다.

### Copy 트레이트의 특징

1. 마커 트레이트: Copy는 자체 메서드가 없는 마커 트레이트이다.
2. 암시적 복사: Copy가 구현된 타입은 할당이나 함수 호출 시 암시적으로 복사된다.(얕은 복사라고 근사하여 이해할 수 있다.)
3. Clone 의존성: Copy 트레이트는 Clone 트레이트에 의존한다(즉, Super-Trait).(Copy 트레이트를 구현하려면, Clone 트레이트를 구현해야한다.)

### Copy 구현 제한

모든 타입이 Copy를 구현할 수 있는 것은 아니다. 다음과 같은 경우 Copy를 구현할 수 없다: `Drop` 트레이트를 구현한 타입

따라서 힙에 할당된 데이터를 포함하는 타입 (예: `String`, `Vec<T>`)은 Copy를 구현할 수 없다. 이는 소유권 시스템을 ambiguity하게 만들기 때문이다. 

```rust
struct NoCopy;

impl Drop for NoCopy {
    fn drop(&mut self) {}
}

// 컴파일 에러: Copy와 Drop은 동시에 구현할 수 없음
// impl Copy for NoCopy {}
```

## Clone 트레이트

Clone 트레이트는 Copy보다 더 유연한 복제 방법을 제공한다. 힙에 할당된 데이터를 포함한 복잡한 타입도 Clone을 구현할 수 있다.

### Clone 트레이트의 특징

1. 명시적 복제: `clone()` 메서드를 호출하여 명시적으로 복제해야 한다.
2. 깊은 복사 가능: 힙에 할당된 데이터도 완전히 새로운 인스턴스로 복제할 수 있다.
3. Copy의 상위 트레이트: Copy를 구현하려면 반드시 Clone도 구현해야 한다.

```rust
#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

let person1 = Person {
    name: String::from("Alice"),
    age: 30,
};

let person2 = person1.clone();
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new copy_clone_demo`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Copy
    println!("p1: {:?}, p2: {:?}", p1, p2);

    let l1 = Line { start: Point { x: 0, y: 0 }, end: Point { x: 5, y: 5 } };
    let l2 = l1.clone();  // Clone
    println!("l1: {:?}", l1);
    println!("l2: {:?}", l2);
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
    fn test_point_copy() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1;
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }

    #[test]
    fn test_line_clone() {
        let l1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 5, y: 5 },
        };
        let l2 = l1.clone();
        assert_eq!(l1.start.x, l2.start.x);
        assert_eq!(l1.start.y, l2.start.y);
        assert_eq!(l1.end.x, l2.end.x);
        assert_eq!(l1.end.y, l2.end.y);
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 