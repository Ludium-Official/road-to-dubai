# 컬렉션

## 학습 목표
- Rust의 주요 컬렉션 타입(Vec, String, HashMap)의 내부 구조를 이해한다.
- 각 컬렉션 타입의 메모리 레이아웃과 성능 특성을 파악한다.
- 컬렉션 타입들의 주요 메서드와 사용 패턴을 학습한다.
- 각 컬렉션 타입의 최적화 기법을 이해한다.

## Vec<T>

Vec<T>는 Rust의 동적 배열 구현체이다. 내부 구조는 다음과 같다:

```rust
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
```

- `ptr`: T 타입 요소들의 연속된 메모리 블록을 가리키는 포인터
- `len`: 현재 벡터에 저장된 요소의 수
- `cap`: 할당된 메모리의 총 용량 (요소 개수 단위)

Vec<T>의 메모리 레이아웃:

```
Stack:
+--------+--------+--------+
|  ptr   |  len   |  cap   |
+--------+--------+--------+
    |
    v
Heap:
+------+------+------+------+------+------+
|  T1  |  T2  |  T3  |  T4  |  ... |  Tn  |
+------+------+------+------+------+------+
```

Vec<T>는 용량이 가득 차면 새로운 메모리를 할당하고 요소를 복사한다. 성장 전략은 다음과 같다:

1. 현재 용량이 0이면, 4 또는 `T`의 크기에 따라 적절한 초기 용량으로 설정
2. 그 외의 경우, 현재 용량의 2배로 증가

주요 메서드:
- `push(value)`: 벡터 끝에 요소 추가
- `pop()`: 벡터 끝의 요소 제거 및 반환
- `insert(index, value)`: 지정된 인덱스에 요소 삽입
- `remove(index)`: 지정된 인덱스의 요소 제거 및 반환

## String

String은 UTF-8 인코딩된 가변 길이 문자열이다. 내부적으로 Vec<u8>을 래핑한 구조체이다:

```rust
pub struct String {
    vec: Vec<u8>,
}
```

String의 메모리 레이아웃은 Vec<u8>과 동일하다:

```
Stack:
+--------+--------+--------+
|  ptr   |  len   |  cap   |
+--------+--------+--------+
    |
    v
Heap:
+------+------+------+------+------+------+
| byte | byte | byte | byte |  ... | byte |
+------+------+------+------+------+------+
```

String은 항상 유효한 UTF-8 시퀀스를 보장한다. 이는 다음과 같은 특성을 가진다:

- 1~4바이트로 문자를 표현
- ASCII 문자는 1바이트로 표현되어 효율적
- 문자 단위 인덱싱이 O(n) 시간 복잡도를 가짐

주요 메서드:
- `push_str(&str)`: 문자열 끝에 str 추가
- `push(char)`: 문자열 끝에 문자 추가
- `insert_str(index, &str)`: 지정된 바이트 인덱스에 str 삽입
- `remove(index)`: 지정된 바이트 인덱스의 문자 제거 및 반환

## HashMap<K, V>

HashMap<K, V>는 키-값 쌍을 저장하는 해시 테이블 구현체이다. 내부 구조는 다음과 같다:

```rust
pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

struct HashMap<K, V, S> {
    hash_builder: S,
    table: RawTable<(K, V)>,
}
```

- `hash_builder`: 키를 해시값으로 변환하는 해시 함수
- `table`: 실제 키-값 쌍을 저장하는 테이블

HashMap의 메모리 레이아웃:

```
Stack:
+----------------+----------------+
|  hash_builder  |     table      |
+----------------+----------------+
                       |
                       v
Heap:
+------+------+------+------+------+------+
| Slot | Slot | Slot | Slot |  ... | Slot |
+------+------+------+------+------+------+
```

각 Slot은 다음 중 하나이다:
- 비어있음
- 삭제됨
- (해시, 키, 값) 튜플

HashMap은 기본적으로 SipHash-1-3 알고리즘을 사용하며, Robin Hood 해싱 기법으로 충돌을 해결한다.

주요 메서드:
- `insert(key, value)`: 키-값 쌍 삽입
- `get(&key)`: 키에 해당하는 값 참조 반환
- `remove(&key)`: 키-값 쌍 제거 및 값 반환
- `contains_key(&key)`: 키 존재 여부 확인

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new collections_example`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
use std::collections::HashMap;

fn main() {
    // Vec 예제
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec: {:?}", vec);

    // String 예제
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("String: {}", s);

    // HashMap 예제
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    println!("HashMap: {:?}", map);
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
    fn test_vec() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_string() {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        assert_eq!(s, "Hello, world!");
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert(String::from("Blue"), 10);
        map.insert(String::from("Yellow"), 50);
        assert_eq!(map.get("Blue"), Some(&10));
        assert_eq!(map.get("Yellow"), Some(&50));
    }
}
```

이 테스트 코드를 `src/main.rs` 파일의 끝에 추가하고, `cargo test` 명령어를 실행하여 테스트를 수행할 수 있다. 모든 테스트가 통과하면 예제 코드가 올바르게 작성되었음을 확인할 수 있다.