# Rust의 함수형 프로그래밍 문법

## 학습 목표
- Rust에서 함수형 프로그래밍의 주요 개념을 이해한다.
- 순수 함수, 불변성, 고차 함수, 재귀, 패턴 매칭 등의 개념을 Rust 코드로 구현할 수 있다.
- 함수형 프로그래밍 패러다임이 Rust에서 어떻게 적용되는지 파악한다.

## 1. 순수 함수 (Pure Functions)

순수 함수는 함수형 프로그래밍의 핵심이다. 이들은 같은 입력에 대해 항상 같은 출력을 반환하고, 부작용이 없다. 
구분하는 법은 암묵적인 입출력이 없어야 한다.
예를 들면, 암묵적인 입력은 전역변수 등을 말하고,
출력은 전역변수를 바꾼다던지, 콘솔에 출력한다든지 등의 부작용을 말한다.

### 특징:
- 입력에만 의존한다.
- 외부 상태를 변경하지 않는다.
- 예측 가능한 결과를 제공한다.


```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(5, 3);
    println!("Result: {}", result);  // 항상 8을 출력한다
}
```

이 `add` 함수는 순수 함수다. 같은 입력에 대해 항상 같은 결과를 반환하고, 외부 상태를 변경하지 않는다.

## 2. 불변성 (Immutability)

함수형 프로그래밍에서는 한 번 생성된 데이터를 변경하지 않는다. 대신, 새로운 데이터를 만들어 반환한다.

### 장점:
- 예측 가능성이 향상된다.
- 동시성 처리가 용이해진다.
- 부작용이 감소한다.


```rust
fn main() {
    let original = vec![1, 2, 3];
    let doubled = double_vector(&original);
    
    println!("Original: {:?}", original);
    println!("Doubled: {:?}", doubled);
}

fn double_vector(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|&x| x * 2).collect()
}
```

이 예시에서 `double_vector` 함수는 원본 벡터를 변경하지 않고 새로운 벡터를 만들어 반환한다.

## 3. 고차 함수 (Higher-Order Functions)

고차 함수는 다른 함수를 인자로 받거나 함수를 반환할 수 있는 함수다. 이를 통해 추상화 수준을 높이고 코드 재사용성을 증가시킬 수 있다.

### 특징:
- 함수를 값처럼 다룬다.
- 코드의 모듈성을 향상시킨다.
- 유연한 추상화가 가능하다.


```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared: {:?}", squared);

    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);
}
```

여기서 `map`과 `fold`는 고차 함수다. 이들은 다른 함수(클로저)를 인자로 받아 연산을 수행한다.

## 4. 재귀 (Recursion)

재귀는 함수가 자기 자신을 호출하는 프로그래밍 기법이다. 함수형 프로그래밍에서는 루프 대신 재귀를 자주 사용한다.


```rust
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Factorial of 5: {}", factorial(5));
}
```

이 예시에서 `factorial` 함수는 자기 자신을 호출하며 계산을 수행한다.

## 5. 패턴 매칭 (Pattern Matching)

패턴 매칭은 데이터의 구조를 분석하고 그에 따라 코드를 실행하는 강력한 기능이다.

### 특징:
- 복잡한 데이터 구조를 쉽게 처리한다.
- 코드의 가독성을 향상시킨다.
- 컴파일 시점 오류 검출이 가능하다.


```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 3.0);
    
    println!("Circle area: {}", area(circle));
    println!("Rectangle area: {}", area(rectangle));
}
```

이 예시에서 `area` 함수는 패턴 매칭을 사용해 다양한 도형의 면적을 계산한다.

## 6. 클로저 (Closures)

클로저는 자신의 환경을 캡처할 수 있는 익명 함수다. Rust에서 클로저는 함수형 프로그래밍의 중요한 부분이다.


```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    
    let y = 4;
    assert!(equal_to_x(y));
}
```

이 예시에서 `equal_to_x`는 자신의 환경에서 `x`의 값을 캡처하는 클로저다.

## 7. 이터레이터 (Iterators)

이터레이터는 일련의 항목들을 처리하는 방법을 제공한다. Rust의 이터레이터는 지연 평가(lazy evaluation)를 사용하여 효율적인 처리를 가능하게 한다.


```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter()
                    .filter(|&x| x % 2 == 0)
                    .map(|&x| x * x)
                    .sum();
    println!("Sum of squares of even numbers: {}", sum);
}
```

이 예시에서는 이터레이터 메서드 체인을 사용하여 벡터의 짝수 요소들의 제곱의 합을 계산한다.

## Reference

1. "The Rust Programming Language" 공식 문서: https://doc.rust-lang.org/book/
2. "Rust by Example": https://doc.rust-lang.org/rust-by-example/
3. "Programming Rust: Fast, Safe Systems Development" by Jim Blandy and Jason Orendorff
4. "Hands-On Functional Programming in Rust" by Andrew Johnson
