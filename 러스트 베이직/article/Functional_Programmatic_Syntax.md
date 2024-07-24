
# 함수형 프로그래밍 문법

Rust의 함수형 프로그래밍 문법에 대해서 알아보자! 

## 1. 순수 함수 (Pure Functions)

순수 함수는 함수형 프로그래밍의 핵심이야. 이들은 같은 입력에 대해 항상 같은 출력을 반환하고, 부작용이 없어. 
구분하는 법은 암묵적인 입, 출력이 없어야해.
예를 들면, 암묵적인 입력은 전역변수 등을 말하고,
출력을 전역변수를 바꾼다던지, 콘솔에 출력한다든지 등의 부작용을 말해

### 특징:
- 입력에만 의존
- 외부 상태를 변경하지 않음
- 예측 가능한 결과

예시 (Rust):

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(5, 3);
    println!("Result: {}", result);  // 항상 8을 출력
}
```

이 `add` 함수는 순수 함수야. 같은 입력에 대해 항상 같은 결과를 반환하고, 외부 상태를 변경하지 않지.

## 2. 불변성 (Immutability)

함수형 프로그래밍에서는 한 번 생성된 데이터를 변경하지 않아. 대신, 새로운 데이터를 만들어 반환해.

### 장점:
- 예측 가능성 향상
- 동시성 처리 용이
- 부작용 감소

예시 (Rust):

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

이 예시에서 `double_vector` 함수는 원본 벡터를 변경하지 않고 새로운 벡터를 만들어 반환해.

## 3. 고차 함수 (Higher-Order Functions)

고차 함수는 다른 함수를 인자로 받거나 함수를 반환할 수 있는 함수야. 이를 통해 추상화 수준을 높이고 코드 재사용성을 증가시킬 수 있어.

### 특징:
- 함수를 값처럼 다룸
- 코드의 모듈성 향상
- 유연한 추상화 가능

예시 (Rust):

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared: {:?}", squared);

    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);
}
```

여기서 `map`과 `fold`는 고차 함수야. 이들은 다른 함수(클로저)를 인자로 받아 연산을 수행해.

## 4. 재귀 (Recursion)

재귀는 함수가 자기 자신을 호출하는 프로그래밍 기법이야. 함수형 프로그래밍에서는 루프 대신 재귀를 자주 사용해.

예시 (Rust):

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

이 예시에서 `factorial` 함수는 자기 자신을 호출하며 계산을 수행해.

## 5. 패턴 매칭 (Pattern Matching)

패턴 매칭은 데이터의 구조를 분석하고 그에 따라 코드를 실행하는 강력한 기능이야.

### 특징:
- 복잡한 데이터 구조 쉽게 처리
- 코드의 가독성 향상
- 컴파일 시점 오류 검출

예시 (Rust):

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

이 예시에서 `area` 함수는 패턴 매칭을 사용해 다양한 도형의 면적을 계산해.


