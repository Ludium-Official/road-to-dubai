# 스마트 포인터 (Smart Pointers)

## 학습 목표
- 스마트 포인터의 개념과 필요성을 이해한다.
- Rust에서 제공하는 주요 스마트 포인터 타입들을 학습한다.
- 각 스마트 포인터의 특징, 사용 사례, 내부 동작 원리를 파악한다.
- 스마트 포인터와 소유권, 수명 관리의 관계를 이해한다.
- 사용자 정의 스마트 포인터를 구현하는 방법을 익힌다.
- 스마트 포인터의 성능 특성과 최적화 기법을 학습한다.
- 실제 코드에서 스마트 포인터를 효과적으로 활용하는 방법을 습득한다.

## 스마트 포인터의 개념

스마트 포인터는 포인터처럼 동작하면서 추가적인 메타데이터와 기능을 제공하는 데이터 구조다. 일반 참조자와 달리, 스마트 포인터는 대부분 데이터를 소유한다. Rust에서 스마트 포인터는 주로 `Deref`와 `Drop` 트레이트를 구현하여 만들어진다.

### 스마트 포인터의 필요성

1. 메모리 관리 자동화: 자원의 수명을 자동으로 관리한다.
2. 다중 소유권: 데이터를 여러 소유자가 공유할 수 있게 한다.
3. 내부 가변성: 불변 참조를 통해 가변 데이터에 접근할 수 있게 한다.
4. 런타임 다형성: 트레이트 객체를 통해 동적 디스패치를 가능하게 한다.
5. 지연 초기화: 필요한 시점에 데이터를 초기화할 수 있다.

## Rust의 주요 스마트 포인터

### 1. Box<T>

`Box<T>`는 가장 단순한 형태의 스마트 포인터다. 힙에 데이터를 할당하고, 스택에는 그 힙 데이터를 가리키는 포인터를 저장한다.
참고로, 커링에서 impl Fn을 중첩할 수 없어서 `impl Fn(String) -> Box<dyn Fn(F) -> () >` 이런 식으로 한 바 있다.
모든 타입에 대해 자유롭다.

특징:
- 컴파일 시점에 크기를 알 수 없는 타입을 다룰 때 유용하다.
- 큰 데이터를 복사하지 않고 소유권을 이전할 때 사용한다.
- 트레이트 객체를 만들 때도 사용된다. `Box<dyn Trait>`

### 2. Rc<T>

`Rc<T>`는 Reference Counting의 약자로, 다중 소유권을 제공한다. 동일한 데이터에 대해 여러 개의 불변 참조를 가능하게 한다.

특징:
- 데이터를 여러 부분에서 공유해야 할 때 사용한다.
- 참조 카운트를 통해 마지막 소유자가 드롭될 때 메모리를 해제한다.
- 순환 참조를 만들 수 있어 메모리 누수의 가능성이 있다.

### 3. RefCell<T>

`RefCell<T>`는 내부 가변성을 제공하는 스마트 포인터다. 불변 참조를 통해 가변 데이터에 접근할 수 있게 해준다.

특징:
- 컴파일 시간이 아닌 런타임에 차용 규칙을 검사한다.
- 단일 스레드 환경에서만 안전하다.
- `borrow()`와 `borrow_mut()` 메서드를 통해 내부 값에 접근한다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new smart_pointers`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));

    child1.add_child(Rc::clone(&child2));

    println!("Root: {:?}", root);
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.


## 테스트 코드

`src/main.rs` 파일의 끝에 다음 테스트 코드를 추가한다:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(5);
        assert_eq!(node.value, 5);
        assert!(node.children.borrow().is_empty());
    }

    #[test]
    fn test_add_child() {
        let parent = Node::new(1);
        let child = Node::new(2);
        parent.add_child(Rc::clone(&child));
        assert_eq!(parent.children.borrow().len(), 1);
        assert_eq!(parent.children.borrow()[0].value, 2);
    }

    #[test]
    fn test_multiple_ownership() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);

        node1.add_child(Rc::clone(&node2));
        node1.add_child(Rc::clone(&node3));
        node2.add_child(Rc::clone(&node3));

        assert_eq!(Rc::strong_count(&node3), 3);
    }
}
```

이 테스트 코드는 노드 생성, 자식 노드 추가, 그리고 다중 소유권을 검증한다.

## Reference

1. Rust 공식 문서 - 스마트 포인터: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
2. Rust 표준 라이브러리 문서: https://doc.rust-lang.org/std/
3. "Programming Rust, 2nd Edition" by Jim Blandy, Jason Orendorff, and Leonora F.S. Tindall (2021)
4. "Rust in Action" by Tim McNamara (2021)
5. Rust RFC - Stabilize Weak::new(): https://github.com/rust-lang/rfcs/blob/master/text/3110-weak-new.md
6. Rust 블로그 - Rust 2021 Edition: https://blog.rust-lang.org/2021/05/11/edition-2021.html
7. "The Rustonomicon" - Rust의 unsafe 프로그래밍 가이드: https://doc.rust-lang.org/nomicon/
8. Rust 성능 북: https://nnethercote.github.io/perf-book/
9. Rust 디자인 패턴: https://rust-unofficial.github.io/patterns/
