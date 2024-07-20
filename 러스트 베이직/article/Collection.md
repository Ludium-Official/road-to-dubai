# 컬렉션

## 학습 목표
- Rust의 주요 컬렉션 타입들을 이해한다.
- 벡터, 스트링, 해시맵의 사용법을 익힌다.
- 각 컬렉션 타입의 특징과 적절한 사용 상황을 파악한다.
- 컬렉션들의 내부 구현과 시간 복잡도를 이해한다.

## Rust의 컬렉션

Rust의 표준 라이브러리에는 컬렉션이라고 불리는 매우 유용한 데이터 구조들이 여러 개 포함되어 있다. 대부분의 다른 데이터 타입들은 하나의 특정한 값을 표현하지만, 컬렉션은 여러 개의 값들을 담을 수 있다. 튜플이나 배열과는 다르게, 이 컬렉션들이 가리키는 데이터는 힙에 저장되기 때문에 컴파일 타임에 크기를 알 필요가 없다. 즉 컬렉션은 프로그램 실행 중에 그 크기가 커지거나 작아질 수 있다는 뜻이다.

이 장에서는 Rust 프로그램에서 매우 자주 사용되는 세 가지 컬렉션에 대해 배워볼 것이다:

- 벡터(Vector): 가변 개수의 값들을 저장할 수 있도록 해준다.
- 스트링(String): 문자들의 모음이다.
- 해시맵(Hash Map): 특정 키와 값을 연관시켜 저장할 수 있도록 해주는 좀 더 일반적인 구현체의 컬렉션이다.

## 벡터

벡터(`Vec<T>`)는 같은 타입의 값을 메모리상에 이웃하도록 배치하여 저장한다. 

### 내부 구현
벡터는 내부적으로 동적 배열로 구현되어 있다. 이는 연속된 메모리 공간에 요소들을 저장하며, 필요에 따라 크기를 조절한다. 벡터는 세 가지 필드를 가지고 있다:
1. 데이터에 대한 포인터
2. 길이 (현재 사용 중인 요소의 수)
3. 용량 (할당된 총 공간)

### 시간 복잡도
- 끝에 요소 추가/제거 (push/pop): O(1) 평균
- 임의의 위치에 요소 삽입/제거: O(n)
- 인덱스로 접근: O(1)
- 순회: O(n)

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

## 스트링

Rust의 `String` 타입은 UTF-8로 인코딩된 가변 길이 문자열이다.

### 내부 구현
`String`은 내부적으로 `Vec<u8>`을 래핑한 구조체다. 이는 바이트의 벡터를 사용하여 UTF-8 인코딩된 텍스트를 저장한다.

### 시간 복잡도
- 끝에 추가: O(1) 평균
- 문자열 연결: O(n), 여기서 n은 두 번째 문자열의 길이
- 길이 확인: O(1)
- 인덱스로 접근: O(n), UTF-8 인코딩 때문

```rust
let mut s = String::new();
s.push_str("Hello");
s.push(',');
s += " world!";
```

## 해시맵

`HashMap<K, V>`는 키-값 쌍을 저장하는 데 사용된다.

### 내부 구현
해시맵은 내부적으로 해시 테이블을 사용한다. 키는 해시 함수를 통해 해시값으로 변환되고, 이 해시값은 값을 저장할 "버킷"을 결정한다. Rust의 해시맵은 충돌 해결을 위해 선형 탐사(linear probing)를 사용한다.

### 시간 복잡도
- 삽입/검색/삭제: O(1) 평균, 최악의 경우 O(n)
- 순회: O(n)

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new collections`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
use std::collections::HashMap;

fn main() {
    // 벡터
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector: {:?}", v);

    // 스트링
    let mut s = String::new();
    s.push_str("Hello");
    s.push(',');
    s += " world!";
    println!("String: {}", s);

    // 해시맵
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("HashMap: {:?}", scores);
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
    fn test_vector() {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        assert_eq!(v, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_string() {
        let mut s = String::new();
        s.push_str("Hello");
        s.push(',');
        s += " world!";
        assert_eq!(s, "Hello, world!");
    }

    #[test]
    fn test_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        assert_eq!(scores.get("Blue"), Some(&10));
        assert_eq!(scores.get("Yellow"), Some(&50));
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 모든 테스트가 통과하면 예제 코드가 올바르게 작성되었음을 확인할 수 있다.

이렇게 Rust의 주요 컬렉션 타입들에 대해 알아보았다. 각 컬렉션의 내부 구현과 시간 복잡도를 이해하는 것은 효율적인 프로그램을 작성하는 데 매우 중요하다. 상황에 따라 적절한 컬렉션을 선택하고 사용하는 것이 Rust 프로그래밍의 핵심이 된다.