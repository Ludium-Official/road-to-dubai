# 함수형 프로그래밍 패러다임

함수형 프로그래밍(FP) 패러다임은 복잡한 문제를 작은 순수 함수들로 나누고 이를 조합해 해결하는 접근 방식이야. 이 패러다임을 사용하면 여러 가지 장점이 있어. 특히 병렬 처리, 동시성, 비동기 프로그래밍 같은 현대 프로그래밍의 중요한 개념들을 다루는 데 큰 도움이 돼. 이제 이런 장점들을 자세히 살펴보자.

## 1. 함수형 프로그래밍의 주요 장점

### 1.1 코드의 예측 가능성과 테스트 용이성

함수형 프로그래밍에서는 순수 함수를 사용해. 순수 함수는 같은 입력에 대해 항상 같은 출력을 반환하고, 외부 상태를 변경하지 않아. 이런 특성 때문에 코드의 동작을 예측하기 쉽고 테스트하기도 편해져.

```rust
// 순수 함수 예시
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 테스트하기 쉬워
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}
```

### 1.2 부작용 최소화

함수형 프로그래밍에서는 부작용(side effects)을 최소화해. 부작용이란 함수가 자신의 스코프 밖의 상태를 변경하는 것을 말해. 부작용을 줄이면 프로그램의 동작을 이해하고 디버그하기가 더 쉬워져.

```rust
// 부작용이 있는 함수
let mut global_counter = 0;
fn increment_counter() {
    global_counter += 1;  // 부작용: 전역 변수 수정
}

// 부작용이 없는 함수형 접근
fn increment(counter: i32) -> i32 {
    counter + 1  // 새로운 값을 반환하고 원본은 변경하지 않음
}
```

### 1.3 코드의 모듈성과 재사용성

함수형 프로그래밍에서는 작은 순수 함수들을 조합해 복잡한 로직을 만들어. 이렇게 하면 각 함수를 독립적으로 테스트하고 재사용하기 쉬워져.

```rust
fn double(x: i32) -> i32 { x * 2 }
fn increment(x: i32) -> i32 { x + 1 }

// 함수 조합
fn double_then_increment(x: i32) -> i32 {
    increment(double(x))
}
```

### 1.4 불변성을 통한 안전성

함수형 프로그래밍에서는 데이터 불변성을 중요하게 여겨. 데이터를 직접 수정하는 대신 새로운 데이터를 생성해. 이렇게 하면 예상치 못한 데이터 변경으로 인한 버그를 줄일 수 있어.

```rust
// 불변성을 이용한 리스트 처리
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
// numbers는 그대로 유지되고, doubled에는 새로운 결과가 저장돼
```

## 2. 병렬 처리와 함수형 프로그래밍

병렬 처리는 여러 작업을 동시에 수행해 전체 처리 시간을 줄이는 기법이야. 함수형 프로그래밍은 이런 병렬 처리에 매우 적합해.

### 2.1 병렬 처리의 개념

병렬 처리는 하나의 큰 작업을 여러 개의 작은 작업으로 나누고, 이를 여러 처리 장치(예: CPU 코어)에서 동시에 실행하는 거야. 이렇게 하면 전체 작업 시간을 크게 줄일 수 있지.

<antArtifact identifier="parallel-processing-diagram" type="image/svg+xml" title="Parallel Processing Diagram">
<svg viewBox="0 0 500 300" xmlns="http://www.w3.org/2000/svg">
  <rect x="10" y="10" width="480" height="280" fill="none" stroke="black" />
  <text x="250" y="40" text-anchor="middle" font-size="20">병렬 처리</text>
  
  <rect x="30" y="70" width="440" height="50" fill="lightblue" />
  <text x="250" y="100" text-anchor="middle">큰 작업</text>
  
  <line x1="250" y1="120" x2="250" y2="150" stroke="black" stroke-width="2" />
  <line x1="100" y1="150" x2="400" y2="150" stroke="black" stroke-width="2" />
  
  <rect x="30" y="170" width="100" height="40" fill="lightgreen" />
  <text x="80" y="195" text-anchor="middle" font-size="14">작은 작업 1</text>
  
  <rect x="140" y="170" width="100" height="40" fill="lightgreen" />
  <text x="190" y="195" text-anchor="middle" font-size="14">작은 작업 2</text>
  
  <rect x="250" y="170" width="100" height="40" fill="lightgreen" />
  <text x="300" y="195" text-anchor="middle" font-size="14">작은 작업 3</text>
  
  <rect x="360" y="170" width="100" height="40" fill="lightgreen" />
  <text x="410" y="195" text-anchor="middle" font-size="14">작은 작업 4</text>
  
  <line x1="80" y1="210" x2="80" y2="240" stroke="black" stroke-width="2" />
  <line x1="190" y1="210" x2="190" y2="240" stroke="black" stroke-width="2" />
  <line x1="300" y1="210" x2="300" y2="240" stroke="black" stroke-width="2" />
  <line x1="410" y1="210" x2="410" y2="240" stroke="black" stroke-width="2" />
  
  <rect x="30" y="240" width="440" height="40" fill="lightyellow" />
  <text x="250" y="265" text-anchor="middle">결과 조합</text>
</svg>
