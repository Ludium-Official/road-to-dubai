# Rust의 Flow 제어

안녕! 오늘은 Rust의 Flow 제어에 대해 알아볼 거야. Flow 제어는 프로그램의 실행 흐름을 결정하는 중요한 개념이지. Rust에서는 조건문, 반복문, 그리고 패턴 매칭을 통해 Flow를 제어할 수 있어. 하나씩 살펴보자!

## 조건문 (if 표현식)

Rust에서 조건문은 `if` 표현식을 사용해. 다른 언어와 비슷하지만, 몇 가지 특징이 있어.

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("조건은 참이에요");
    } else {
        println!("조건은 거짓이에요");
    }
}
```

여기서 주의할 점은 Rust의 `if`는 표현식이라는 거야. 이게 무슨 말이냐면, `if`의 결과를 변수에 바로 할당할 수 있다는 뜻이지.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

println!("number의 값: {}", number);
```

이렇게 하면 `number`의 값은 5가 돼. 근데 주의해야 할 점은 `if`와 `else` 블록의 타입이 같아야 한다는 거야.

## 반복문

Rust에서는 세 가지 종류의 반복문을 제공해: `loop`, `while`, `for`. 각각 살펴보자.

### loop

`loop`는 무한 반복을 위한 키워드야. `break`를 사용해서 루프를 종료할 수 있지.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("결과는 {}", result);
}
```

이 코드는 `result`에 20을 할당해. `loop`도 표현식이라 결과를 반환할 수 있어.

### while

`while`은 조건이 참인 동안 계속 실행돼.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("발사!");
}
```

### for

`for`는 컬렉션을 순회할 때 주로 사용해.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("값: {}", element);
    }
}
```

범위를 순회할 때는 이렇게 할 수 있어:

```rust
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("발사!");
```

## 패턴 매칭

Rust의 `match` 표현식은 강력한 패턴 매칭 기능을 제공해.

```rust
fn main() {
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
```

`match`는 모든 가능한 경우를 처리해야 해. `_`는 모든 값에 매치되는 와일드카드 패턴이야.

이렇게 Rust의 기본적인 Flow 제어에 대해 알아봤어. 이를 잘 활용하면 더 복잡한 로직도 쉽게 구현할 수 있을 거야. 실제로 코드를 작성하면서 연습해보는 게 가장 좋은 학습 방법이니까, 직접 해보는 걸 추천해!
