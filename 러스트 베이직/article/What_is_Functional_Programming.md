# 함수형 프로그래밍

## 학습 목표
- 함수형 프로그래밍의 개념과 필요성을 깊이 이해한다.
- 함수형 프로그래밍의 주요 개념과 방식을 상세히 학습한다.


## 함수형 프로그래밍의 정의

함수형 프로그래밍은 계산을 수학적 함수의 평가로 취급하고, 상태 변경과 가변 데이터를 피하는 프로그래밍 패러다임이다. 이 접근 방식은 프로그램의 동작을 함수들의 연속적인 실행으로 표현하며, 각 함수는 입력을 받아 출력을 생성하지만 프로그램의 상태를 변경하지 않는다.

함수형 프로그래밍의 핵심 개념은 다음과 같다:

1. 순수 함수 (Pure Functions): 같은 입력에 대해 항상 같은 출력을 반환하며, 부작용이 없는 함수를 사용한다. 이는 프로그램의 동작을 예측 가능하게 만들고, 테스트와 디버깅을 용이하게 한다.

2. 불변성 (Immutability): 한 번 생성된 데이터의 상태를 변경하지 않는다. 이는 예기치 않은 부작용을 방지하고, 병렬 처리를 안전하게 만든다.

3. 고차 함수 (Higher-Order Functions): 함수를 다른 함수의 인자로 전달하거나, 함수에서 함수를 반환할 수 있다. 이를 통해 코드의 추상화 수준을 높이고 재사용성을 증가시킬 수 있다.

4. 선언적 프로그래밍 (Declarative Programming): '어떻게' 계산할 것인가보다 '무엇을' 계산할 것인가에 초점을 맞춘다. 이는 코드의 의도를 더 명확하게 표현할 수 있게 해준다.

5. 재귀 (Recursion): 반복문 대신 재귀를 사용하여 문제를 해결한다. 이는 상태 변경을 최소화하고 코드를 더 간결하게 만들 수 있다.

6. 지연 평가 (Lazy Evaluation): 결과가 필요할 때까지 계산을 미루는 기법이다. 이를 통해 불필요한 계산을 피하고 성능을 최적화할 수 있다.

이러한 특징들은 코드의 예측 가능성을 높이고, 병렬 처리를 용이하게 하며, 버그를 줄이는 데 도움을 준다. 또한, 함수형 프로그래밍은 프로그램의 정확성을 보장하기 쉽게 만든다.

이건 물론 내 생각이지만, 수학이 그렇듯 아주 작은 공리에서부터 하나씩 체계를 쌓아나가는데, 함수형 프로그래밍도 순수함수들을 모아서 하나씩 무결성을 보장하는 프로그램으로 쌓아나간다고 할 수 있다. 

## 함수형 프로그래밍의 필요성

함수형 프로그래밍은 다음과 같은 이유로 필요하다:

### 1. 코드의 예측 가능성 증가

함수형 프로그래밍은 부작용을 최소화하고 데이터의 불변성을 강조한다. 이로 인해 프로그램의 동작을 더 쉽게 예측할 수 있다. 예를 들어, 순수 함수는 항상 같은 입력에 대해 같은 출력을 생성하므로, 함수의 동작을 이해하고 예측하기가 훨씬 쉬워진다.

```rust
// 예측 가능한 순수 함수
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 예측하기 어려운 비순수 함수
static mut TOTAL: i32 = 0;

fn add_to_total(value: i32) -> i32 {
    unsafe {
        TOTAL += value;
        TOTAL
    }
}

fn main() {
    println!("Pure function result: {}", add(3, 4));
    
    unsafe {
        println!("Impure function result: {}", add_to_total(5));
        println!("Impure function result: {}", add_to_total(3));
    }
}
```

`add` 함수는 항상 같은 입력에 대해 같은 출력을 생성하므로 예측이 쉽다. 반면, `add_to_total` 함수는 전역 변수 `TOTAL`의 현재 상태에 따라 결과가 달라지므로 예측하기 어렵다. 

이러한 예측 가능성은 특히 대규모 시스템에서 매우 중요하다. 복잡한 시스템에서는 상태 변경으로 인한 부작용이 예기치 않은 문제를 일으킬 수 있기 때문이다. 함수형 프로그래밍을 사용하면 각 함수의 동작을 독립적으로 이해하고 예측할 수 있어, 전체 시스템의 동작을 더 쉽게 추론할 수 있다.

또한, 예측 가능한 코드는 디버깅과 유지보수를 훨씬 쉽게 만든다. 버그가 발생했을 때, 상태 변경을 추적할 필요 없이 입력과 출력만을 검사하면 되므로 문제의 원인을 빠르게 파악할 수 있다.

### 2. 병렬 처리 용이성

함수형 프로그래밍의 불변성과 부작용 없는 함수는 병렬 처리를 더 안전하고 쉽게 만든다. 데이터가 변경되지 않기 때문에, 여러 스레드나 프로세스가 동시에 같은 데이터에 접근해도 race condition이 발생하지 않는다. 

```rust
use rayon::prelude::*;

fn process_data(data: &[i32]) -> Vec<i32> {
    data.iter().map(|&x| x * 2).collect()
}

fn main() {
    let data: Vec<i32> = (0..1_000_000).collect();
    let chunks: Vec<&[i32]> = data.chunks(1000).collect();
    
    let result: Vec<i32> = chunks.par_iter()
        .flat_map(|chunk| process_data(chunk))
        .collect();
    
    println!("Processed data length: {}", result.len());
}
```

이 예제에서 `process_data` 함수는 순수 함수이므로, 입력 데이터를 변경하지 않고 새로운 결과를 반환한다. 따라서 여러 프로세스에서 안전하게 병렬 실행될 수 있다.

이러한 특성은 멀티코어 프로세서를 효과적으로 활용할 수 있게 해준다. 요즘 핫하다고 볼 수 있는 빅데이터, 과학 계산, 기계 학습 등 계산 집약적인 작업에서 유용하게 쓰일 수 있다!. 복잡한 동기화 메커니즘 없이도 안전하게 병렬 처리를 구현할 수 있어, 성능을 크게 향상시킬 수 있다.

또한, 우리가 web3에 있는 만큼, 상태 변경이 적어 무결성을 보장하므로 분산 시스템에서도 함수형 프로그래밍의 이점을 활용할 수 있다. 

### 3. 테스트 용이성

순수 함수는 입력과 출력만으로 동작이 결정되므로 테스트하기 쉽다. 외부 상태나 부작용에 의존하지 않기 때문에, 단위 테스트를 작성하고 실행하기가 훨씬 간단해진다.

```rust
fn calculate_discount(price: f64, discount_rate: f64) -> f64 {
    price * (1.0 - discount_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_discount() {
        assert_eq!(calculate_discount(100.0, 0.1), 90.0);
        assert_eq!(calculate_discount(50.0, 0.5), 25.0);
    }
}

fn main() {
    println!("Discount calculation: {}", calculate_discount(100.0, 0.1));
}
```

이 예제에서 `calculate_discount` 함수는 순수 함수이므로, 입력값만 제공하면 쉽게 테스트할 수 있다. 외부 상태에 의존하지 않기 때문에, 테스트 결과가 항상 일관성 있게 나온다.

함수형 프로그래밍의 이러한 특성은 테스트 주도 개발(TDD)과 같은 방법론과 잘 어울린다. 각 함수를 독립적으로 테스트할 수 있기 때문에, 전체 시스템을 구축하기 전에 각 구성 요소의 정확성을 보장할 수 있다. mock objects나 stubs의 사용을 줄일 수 있다. 순수 함수는 외부 의존성이 적기 때문에, 복잡한 설정 없이도 테스트를 작성할 수 있다. 이는 테스트 코드를 더 간단하고 이해하기 쉽게 만든다.

함수형 프로그래밍의 불변성은 테스트의 신뢰성을 높인다. 테스트 중에 상태가 변경되지 않기 때문에, 테스트 순서에 상관없이 항상 같은 결과를 얻을 수 있다. 이는 특히 복잡한 시스템에서 테스트의 일관성과 신뢰성을 보장하는 데 큰 도움이 된다.

테스트의 관점에서, 순수함수는 테스트를 여러번 하지 않아도 되며, 순수함수에 의존하는 고차함수들에 대해서도 무결성을 보장할 수 있다. 

### 5. 코드 재사용성 향상

함수형 프로그래밍에서는 작은, 독립적인 함수들을 조합하여 복잡한 연산을 구성한다. 이러한 접근 방식은 코드의 재사용성을 크게 향상시킨다.

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn increment(x: i32) -> i32 {
    x + 1
}

fn transform(x: i32) -> i32 {
    increment(double(x))
}

fn main() {
    let result = transform(3);
    println!("Transform result: {}", result);
}
```

이 예제에서 `double`과 `increment` 함수는 독립적으로 사용될 수 있으며, `transform` 함수에서 재사용되고 있다. 이러한 작은 함수들은 다양한 상황에서 재사용될 수 있어, 코드의 중복을 줄이고 유지보수성을 향상시킨다.

함수형 프로그래밍에서는 이러한 작은 함수들을 "조합"하여 더 복잡한 동작을 만들어내는 것이 일반적이다. 이는 레고 블록을 조립하는 것과 유사하다고 볼 수 있다. 각각의 작은 함수는 하나의 레고 블록과 같아서, 다양한 방식으로 조합하여 복잡한 구조를 만들어낼 수 있다. 이는 앞서 내가 말한 수학의 체계와 비슷하다고 할 수 있다. 

예를 들어, 다음과 같은 방식으로 함수들을 조합할 수 있다:

```rust
fn compose<F, G, T>(f: F, g: G) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    G: Fn(T) -> T,
    T: Copy,
{
    move |x| f(g(x))
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let transform = compose(double, add_one);
    let result = transform(3);  // (3 + 1) * 2 = 8
    println!("Result: {}", result);
}
```

이 예제에서 `compose` 함수는 두 함수를 받아 새로운 함수를 만든다. 이렇게 만들어진 `transform` 함수는 `add_one`과 `double` 함수를 재사용하여 새로운 기능을 만들어낸다.

## 함수형 프로그래밍의 주요 개념과 방식

### 1. 순수 함수 (Pure Functions)

순수 함수는 동일한 입력에 대해 항상 동일한 출력을 반환하며, 부작용이 없는 함수를 말한다. 순수 함수는 함수형 프로그래밍의 핵심 개념 중 하나이다.

순수 함수의 특징:
1. 같은 입력에 대해 항상 같은 출력을 반환한다.
2. 함수 외부의 상태를 변경하지 않는다 (부작용 없음).
3. 외부 상태에 의존하지 않는다.

예시:
```rust
// 순수 함수
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 순수하지 않은 함수
static mut TOTAL: i32 = 0;

fn add_to_total(value: i32) -> i32 {
    unsafe {
        TOTAL += value;
        TOTAL
    }
}

fn main() {
    println!("Pure function result: {}", add(3, 4));
    
    unsafe {
        println!("Impure function result: {}", add_to_total(5));
    }
}
```

순수 함수를 사용하면 코드의 동작을 이해하고 예측하기가 훨씬 쉬워진다. 이는 특히 복잡한 시스템에서 버그를 줄이고 유지보수성을 향상시키는 데 큰 도움이 된다. 사이드 이펙트를 일으키지 않는다.

### 2. 불변성 (Immutability)

불변성은 한 번 생성된 데이터의 상태를 변경하지 않는 원칙이다. 좀 헷갈릴 수 있는데, 변수에 관한 불변성보다는 데이터에 관한 불변성을 추구한다고 보는 것이 옳다. 함수형 프로그래밍에서는 데이터를 변경하는 대신 새로운 데이터를 생성한다.

예시:
```rust
fn add_to_vector(mut vec: Vec<i32>, item: i32) -> Vec<i32> {
    vec.push(item);
    vec
}

fn add_to_vector_immutable(vec: &[i32], item: i32) -> Vec<i32> {
    let mut new_vec = vec.to_vec();
    new_vec.push(item);
    new_vec
}

fn main() {
    let mut mutable_vec = vec![1, 2, 3];
    mutable_vec = add_to_vector(mutable_vec, 4);
    println!("Mutable vector: {:?}", mutable_vec);

    let immutable_vec = vec![1, 2, 3];
    let new_vec = add_to_vector_immutable(&immutable_vec, 4);
    println!("Original immutable vector: {:?}", immutable_vec);
    println!("New vector: {:?}", new_vec);
}
```

불변성을 통해 예측 가능성을 보장할 수 있고, 동시성 처리에도 용이하다. 
그리고 데이터의 모든 변경 사항을 새로운 객체로 표현할 수 있어 시간에 따른 상태 변화를 쉽게 추적할 수 있다.

### 3. 고차 함수 (Higher-Order Functions)

고차 함수는 함수를 인자로 받거나 함수를 반환하는 함수를 말한다. 이는 함수를 일급 객체(first-class citizens)로 취급하는 함수형 프로그래밍의 핵심 개념이다.

예시:
```rust
fn apply_operation<F>(func: F, x: i32, y: i32) -> i32
    where F: Fn(i32, i32) -> i32
{
    func(x, y)
}

fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let add = |x, y| x + y;
    let multiply = |x, y| x * y;

    println!("Apply add: {}", apply_operation(add, 3, 4));
    println!("Apply multiply: {}", apply_operation(multiply, 3, 4));

    let double = create_multiplier(2);
    let triple = create_multiplier(3);

    println!("Double 5: {}", double(5));
    println!("Triple 5: {}", triple(5));
}
```

고차 함수를 활용하면 코드의 재사용성과 모듈성을 높일 수 있으며, 더 추상화된 수준에서 프로그래밍할 수 있게 된다.

함수형 프로그래밍은 코드의 예측 가능성, 테스트 용이성, 그리고 병렬 처리 능력을 향상시키는 강력한 패러다임이다. 순수 함수, 불변성, 재귀, 고차 함수 등의 개념을 적용함으로써, 개발자는 더 안전하고 유지보수가 쉬운 코드를 작성할 수 있다.
물론 함수형 프로그래밍이 모든 상황에 최적인 것은 아니지만, 이러한 개념들을 이해하고 적절히 활용하면 코드 품질을 크게 향상시킬 수 있다. 현대 프로그래밍에서는 함수형 접근 방식과 다른 패러다임을 조화롭게 결합하여 사용하는 것이 일반적이며, 이를 통해 각 패러다임의 장점을 최대한 활용할 수 있다.
