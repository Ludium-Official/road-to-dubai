  # Map, Filter, Fold

  ## 학습 목표
  - Map, Filter, Fold의 개념과 사용법을 이해한다.
  - Rust에서 이들 함수를 구현하고 사용하는 방법을 학습한다.
  - Fold를 사용하여 Map과 Filter를 구현하는 방법을 익힌다.
  - 이들 함수의 함수형 프로그래밍에서의 중요성을 이해한다.
  - 실제 코드에서 Map, Filter, Fold를 활용하는 방법을 습득한다.

  ## Map, Filter, Fold의 개념

  Map, Filter, Fold는 함수형 프로그래밍의 핵심 개념으로, 데이터 변환과 처리를 선언적이고 간결하게 표현할 수 있게 해준다.
  반복문 보다는 Map, Filter, Fold를 통해서 데이터를 처리하는 것이 함수형 프로그래밍 스타일이다.

  ### Map, Filter, Fold의 필요성

  1. 코드 간결성: 복잡한 루프를 간단한 함수 호출로 대체할 수 있다.
  2. 가독성: 데이터 처리 의도를 명확하게 표현할 수 있다.
  3. 재사용성: 고차 함수를 사용하여 로직을 쉽게 재사용할 수 있다.

  ## Map, Filter, Fold의 구현과 사용

  Rust에서 Map, Filter, Fold는 Iterator 트레이트의 메서드로 구현되어 있다.

  ### Map

  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
  ```

  ### Filter

  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
  ```

  ### Fold

  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
  ```

  ## Fold를 사용한 Map과 Filter 구현

  Fold는 매우 강력한 연산으로, Map과 Filter를 구현하는 데도 사용할 수 있다.

  ### Fold로 Map 구현

  ```rust
  fn map_with_fold<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
  where
      F: Fn(T) -> U,
  {
      vec.into_iter().fold(Vec::new(), |mut acc, x| {
          acc.push(f(x));
          acc
      })
  }

  let numbers = vec![1, 2, 3, 4, 5];
  let squared = map_with_fold(numbers, |x| x * x);
  ```

  ### Fold로 Filter 구현

  ```rust
  fn filter_with_fold<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
  where
      F: Fn(&T) -> bool,
  {
      vec.into_iter().fold(Vec::new(), |mut acc, x| {
          if predicate(&x) {
              acc.push(x);
          }
          acc
      })
  }

  let numbers = vec![1, 2, 3, 4, 5];
  let evens = filter_with_fold(numbers, |&x| x % 2 == 0);
  ```

  ## VSCode에서 실습

  1. VSCode를 열고 새 Rust 프로젝트를 생성한다: `cargo new rust_functional`
  2. `src/main.rs` 파일에 다음 코드를 작성한다:

  ```rust
  fn map_with_fold<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
  where
      F: Fn(T) -> U,
  {
      vec.into_iter().fold(Vec::new(), |mut acc, x| {
          acc.push(f(x));
          acc
      })
  }

  fn filter_with_fold<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
  where
      F: Fn(&T) -> bool,
  {
      vec.into_iter().fold(Vec::new(), |mut acc, x| {
          if predicate(&x) {
              acc.push(x);
          }
          acc
      })
  }

  fn main() {
      let numbers = vec![1, 2, 3, 4, 5];
      
      let squared = map_with_fold(numbers.clone(), |x| x * x);
      println!("Squared: {:?}", squared);

      let evens = filter_with_fold(numbers, |&x| x % 2 == 0);
      println!("Evens: {:?}", evens);
  }
  ```

  3. 터미널에서 `cargo run` 명령어를 실행하여 코드를 컴파일하고 실행한다.

  ## 테스트 코드

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_map_with_fold() {
          let numbers = vec![1, 2, 3, 4, 5];
          let squared = map_with_fold(numbers, |x| x * x);
          assert_eq!(squared, vec![1, 4, 9, 16, 25]);
      }

      #[test]
      fn test_filter_with_fold() {
          let numbers = vec![1, 2, 3, 4, 5];
          let evens = filter_with_fold(numbers, |&x| x % 2 == 0);
          assert_eq!(evens, vec![2, 4]);
      }
  }
  ```

  ## Reference

  1. Rust 공식 문서 - Iterator: https://doc.rust-lang.org/std/iter/trait.Iterator.html
  2. Rust by Example - Iterator: https://doc.rust-lang.org/rust-by-example/trait/iter.html
  3. "Programming Rust" by Jim Blandy and Jason Orendorff
  4. "Hands-On Functional Programming in Rust" by Andrew Johnson
  5. Rust 공식 포럼 - 함수형 프로그래밍 토론: https://users.rust-lang.org/c/rust-users/10
  6. "Functional Programming in Scala" by Paul Chiusano and Rúnar Bjarnason (Scala 책이지만 개념은 유사함)
  7. Haskell Wiki - Fold: https://wiki.haskell.org/Fold
  8. Rust RFC - Iterator 메서드 추가: https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md
