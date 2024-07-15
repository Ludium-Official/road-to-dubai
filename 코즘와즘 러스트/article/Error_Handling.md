# 에러 핸들링

Rust는 에러 핸들링에 있어서 매우 강력한 기능을 제공해. 다른 언어들과는 달리, Rust는 예외를 사용하지 않아. 대신에 에러 처리를 위한 두 가지 주요 카테고리를 가지고 있지: 복구 가능한 에러와 복구 불가능한 에러야.

## 복구 불가능한 에러와 `panic!`

때때로, 코드에서 나쁜 일이 발생하고 그것에 대해 우리가 할 수 있는 게 없을 때가 있어. 이런 경우에 Rust는 `panic!` 매크로를 가지고 있어. 이 매크로가 실행되면, 프로그램은 실패 메시지를 출력하고, 스택을 되감고 정리한 다음, 종료돼. 이건 주로 버그를 발견했고 프로그래머가 이 에러를 어떻게 처리할지 명확하지 않을 때 사용해.

```rust
fn main() {
    panic!("crash and burn");
}
```

이 코드를 실행하면, 다음과 같은 출력을 볼 수 있어:

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

## Result와 함께하는 복구 가능한 에러

대부분의 에러들은 완전히 프로그램을 중단시킬 정도로 심각하지 않아. 가끔, 함수가 실패하면, 그건 쉽게 해석하고 대응할 수 있는 이유인 경우가 많지. 예를 들어, 파일을 열려고 했는데 그 파일이 존재하지 않아서 실패한 경우, 프로그램을 종료하는 대신 파일을 새로 만들고 싶을 수 있어.

이런 상황을 위해 Rust는 `Result` 열거형을 가지고 있어:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

T와 E는 제네릭 타입 파라미터야. T는 성공한 경우 Ok 변형 안에 반환될 값의 타입을 나타내고, E는 실패한 경우 Err 변형 안에 반환될 에러의 타입을 나타내.

여기 `Result`를 사용하는 예제가 있어:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

이 코드는 파일을 열려고 시도하고, 결과를 `match` 표현식으로 처리해. 만약 `File::open`이 성공하면, 파일 핸들이 `f`의 값이 돼. 실패하면, `panic!` 매크로를 호출해.

## 에러 전파하기

때로는 에러를 직접 처리하는 대신, 함수를 호출한 코드에게 에러를 반환하고 싶을 수 있어. 이걸 "에러 전파"라고 불러. 여기 예제가 있어:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

이 함수는 파일에서 사용자 이름을 읽으려고 해. 만약 파일을 열 수 없거나 내용을 읽을 수 없다면, 이 함수는 그 에러를 호출자에게 반환해.

## `?` 연산자

Rust는 에러 전파를 더 쉽게 만들기 위해 `?` 연산자를 제공해. `?`를 `Result`를 반환하는 표현식 뒤에 놓으면, 그건 거의 위의 `match` 표현식과 같은 방식으로 작동해.

여기 `?` 연산자를 사용해 다시 작성한 이전 예제가 있어:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` 연산자를 사용하면 코드가 훨씬 간결해지고 읽기 쉬워져.

이렇게 Rust의 에러 핸들링에 대해 알아봤어. Rust의 에러 처리 방식은 안전하고 명시적이며, 프로그래머가 모든 가능한 실패 케이스를 고려하도록 강제해. 이는 더 견고하고 신뢰할 수 있는 프로그램을 만드는 데 도움을 줘!
