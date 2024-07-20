# 함수형 프로그래밍이 뭐야?

함수형 프로그래밍(Functional Programming, FP)은 소프트웨어 개발의 한 방식이야. 복잡한 문제를 작은 순수 함수들로 쪼개고 이걸 조합해서 해결하는 거지. 단순히 코딩 기법이 아니라 문제를 바라보고 해결하는 사고방식 자체를 바꾸는 거야.

## 1. 함수형 프로그래밍의 정의와 장점

함수형 프로그래밍은 이런 특징이 있어:

- **불변성(Immutability)**: 데이터를 직접 바꾸지 않고, 새로운 데이터를 만들어서 작업해.
- **순수 함수(Pure Functions)**: 같은 입력에는 항상 같은 출력이 나오고, 부작용(side effects)이 없는 함수를 써.
- **선언적 프로그래밍(Declarative Programming)**: '어떻게' 하는지가 아니라 '뭘' 할지를 명시해.
- **고차 함수(Higher-Order Functions)**: 함수를 다른 함수의 인자로 넘기거나 함수에서 함수를 반환할 수 있어.

이런 특징들 때문에 다음과 같은 장점이 생겨:

1. **코드 읽기 쉽고 유지보수하기 좋아짐**: 작은 순수 함수들로 이뤄진 코드는 이해하기 쉽고 테스트하기도 편해.
2. **병렬 처리와 동시성 프로그래밍이 쉬워짐**: 데이터를 안 바꾸고 부작용 없는 함수들을 쓰니까 동시성 문제가 줄어들지.
3. **버그가 줄어듦**: 상태 변경이 최소화되니까 예측 가능한 코드를 짤 수 있어.
4. **재사용성이 높아짐**: 작은 순수 함수들은 다른 상황에서도 쉽게 다시 쓸 수 있어.

## 2. 함수형 사고 (Functional Thinking)

함수형 프로그래밍을 제대로 쓰려면 '함수형 사고'가 필요해. 이건 문제를 바라보는 관점을 바꾸는 거야.

### 액션, 계산, 데이터의 분리

함수형 프로그래밍에서는 프로그램을 이렇게 나눠:

- **액션(Actions)**: 외부 세계와 상호작용하는 부분 (예: 파일 읽기/쓰기, 네트워크 요청)
- **계산(Calculations)**: 입력 받아서 출력 만드는 순수 함수들
- **데이터(Data)**: 변하지 않는 값들

이렇게 나누면 프로그램 복잡성을 관리하기 쉽고, 부작용을 줄이고, 테스트와 디버깅도 편해져.

### 순수 함수 추구

순수 함수가 함수형 프로그래밍의 핵심이야. 순수 함수는:

1. 같은 입력에는 항상 같은 출력이 나와.
2. 외부 상태를 바꾸지 않아 (부작용이 없음).

이런 특성 때문에 순수 함수는 예측 가능하고, 테스트하기 쉽고, 병렬 처리하기도 좋아.

## 3. Rust의 데이터 불변성 (Data Immutability)

Rust는 기본적으로 변수를 불변(immutable)으로 선언해. 하지만, 함수형 프로그래밍에서의 '불변성'은 초점은 변수 자체의 불변성이라기보다는 데이터 불변성에 더 주안점이 있다고 볼 수 있어. 


```rust
let data = vec![1, 2, 3];  // 불변 벡터
// data.push(4);  // 이러면 컴파일 에러 나!

// 새로운 데이터를 만들어서 작업해
let new_data = [&data[..], &[4]].concat();
```

이런 데이터 불변성은 함수형 프로그래밍의 핵심 원칙을 지원하고, 동시성 문제를 줄이고 코드를 안전하게 만들어.

## 4. 소프트웨어 공학적으로 함수형 리팩토링하기

함수형 프로그래밍 원칙을 적용해서 리팩토링하면 코드 품질이 많이 좋아질 수 있어:

### 복잡한 로직을 고차 함수로 추상화하기

```rust
// 이전
let mut result = Vec::new();
for item in items {
    if item.is_valid() {
        result.push(item.process());
    }
}

// 이후
let result: Vec<_> = items.iter()
    .filter(|item| item.is_valid())
    .map(|item| item.process())
    .collect();
```

### 반복문 대신 map, filter, fold 쓰기

```rust
// 이전
let mut sum = 0;
for num in numbers {
    if num % 2 == 0 {
        sum += num * num;
    }
}

// 이후
let sum: i32 = numbers.iter()
    .filter(|&num| num % 2 == 0)
    .map(|&num| num * num)
    .sum();
```

### 순수 함수로 설계해서 테스트 쉽게 만들기

```rust
// 순수 함수 예시
fn calculate_total(items: &[Item]) -> f64 {
    items.iter().map(|item| item.price * item.quantity as f64).sum()
}

// 테스트하기 쉬워
#[test]
fn test_calculate_total() {
    let items = vec![
        Item { price: 10.0, quantity: 2 },
        Item { price: 15.0, quantity: 1 },
    ];
    assert_eq!(calculate_total(&items), 35.0);
}
```

## 5. 함수형으로 추상화하기 (추상화 벽)

함수형 프로그래밍에서는 '추상화 벽(Abstraction Barrier)'이라는 걸로 복잡성을 관리해. 이건 저수준의 구현 세부사항을 고수준의 추상화 뒤에 숨기는 거야.

```rust
// 추상화 벽 뒤의 저수준 구현
fn squared_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}

// 추상화 벽
fn points_are_close(p1: (f64, f64), p2: (f64, f64), threshold: f64) -> bool {
    squared_distance(p1.0, p1.1, p2.0, p2.1) < threshold * threshold
}

// 클라이언트 코드는 추상화된 함수만 써
fn main() {
    let point1 = (0.0, 0.0);
    let point2 = (1.0, 1.0);
    println!("두 점이 가까워? {}", points_are_close(point1, point2, 1.5));
}
```

이렇게 추상화하면 코드의 모듈성이 좋아지고, 변경에 따른 영향을 최소화할 수 있어.

## 결론

함수형 프로그래밍은 복잡한 문제를 단순하고 조합 가능한 부분들로 쪼개는 강력한 방법이야. Rust에서 이런 함수형 프로그래밍 원칙을 적용하면, 더 안전하고, 유지보수하기 쉽고, 확장성 있는 소프트웨어를 만들 수 있어. 함수형으로 생각하면서 문제에 접근하고, 순수 함수와 불변성을 활용해서 코드를 설계하면, 더 나은 소프트웨어 엔지니어링을 할 수 있지.
