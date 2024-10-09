# 지연 평가 (Lazy Evaluation)

## 학습 목표
- 지연 평가의 개념과 필요성을 이해한다.
- Rust에서 지연 평가를 구현하는 방법을 학습한다.
- Iterator와 지연 평가의 관계를 파악한다.
- 클로저를 이용한 지연 평가 기법을 익힌다.
- 지연 평가의 장점을 이해한다.
- Rust의 표준 라이브러리에서 지연 평가가 사용되는 예를 학습한다.
- 실제 코드에서 지연 평가를 활용하는 방법을 습득한다.

## 지연 평가의 개념

지연 평가(Lazy Evaluation)는 표현식의 평가를 실제로 그 값이 필요할 때까지 미루는 전략이다. 이는 불필요한 계산을 피하고, 무한한 데이터 구조를 다룰 수 있게 해주며, 성능을 최적화하는 데 도움을 준다.

### 지연 평가의 필요성

1. **성능 최적화**: 필요한 계산만 수행하여 리소스를 절약한다.
2. **무한 시퀀스**: 이론적으로 무한한 데이터 구조를 다룰 수 있다.
3. **부작용 제어**: 평가 순서를 제어하여 부작용을 관리할 수 있다.
4. **메모리 효율성**: 필요한 데이터만 메모리에 로드할 수 있다.

## Rust에서의 지연 평가 구현

Rust는 기본적으로 엄격한 평가(Strict Evaluation)를 사용하지만, Iterator 트레이트와 클로저를 통해 지연 평가를 구현할 수 있다.

### Iterator를 이용한 지연 평가

```rust
let numbers = 1..;
let even_squares = numbers
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .take(5);

for num in even_squares {
    println!("{}", num);
}
```

이 예제에서 `1..`는 1부터 시작하는 무한 시퀀스를 생성하지만, `take(5)`로 인해 실제로는 처음 5개의 짝수 제곱만 계산된다.

### 클로저를 이용한 지연 평가

```rust
fn lazy_fibonacci() -> impl Fn(usize) -> u64 {
    move |n| {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = a;
            a = b;
            b = temp + b;
        }
        a
    }
}

fn main() {
    let fib = lazy_fibonacci();
    println!("10th Fibonacci number: {}", fib(10));
}
```

이 예제에서 `lazy_fibonacci` 함수는 피보나치 수열을 계산하는 클로저를 반환한다. 실제 계산은 클로저가 호출될 때만 이루어진다.

## 지연 평가의 장점

장점:
1. **불필요한 계산 회피**: 필요할 때만 계산하여 자원을 절약한다.
2. **무한 데이터 구조 처리 가능**: 무한 시퀀스나 데이터 구조를 다룰 수 있다.
3. **메모리 사용 최적화**: 필요한 데이터만 메모리에 로드하여 메모리 사용을 최소화한다.
4. **복잡한 알고리즘의 간결한 표현**: 코드의 가독성과 유지보수성을 높인다.

## Rust 표준 라이브러리의 지연 평가 예

Rust의 `std::iter` 모듈은 다양한 지연 평가 메서드를 제공한다:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum::<i32>();

println!("Sum of squares of even numbers: {}", sum);
```

여기서 `filter`와 `map` 연산은 `sum`이 호출될 때까지 실제로 실행되지 않는다.

## VSCode에서 실습

1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_lazy_eval`
2. `src/main.rs` 파일에 다음 코드를 작성한다:

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let fib = fibonacci().take(10);
    for num in fib {
        println!("{}", num);
    }
}
```

3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = fibonacci().take(10).collect();
        assert_eq!(fib, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    #[test]
    fn test_lazy_evaluation() {
        let mut iter = fibonacci();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }
}
```

## Reference

1. [Rust 공식 문서 - Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
2. "Programming Rust" by Jim Blandy and Jason Orendorff
3. "Rust in Action" by Tim McNamara
4. [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
5. [Rust 공식 포럼 - 지연 평가 토론](https://users.rust-lang.org/t/lazy-evaluation-in-rust/5082)
6. "Functional Programming in Scala" by Paul Chiusano and Rúnar Bjarnason (지연 평가 개념)
7. [Haskell Wiki - Lazy Evaluation](https://wiki.haskell.org/Lazy_evaluation)
8. [Rust RFC - Iterator 메서드 추가](https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md)
