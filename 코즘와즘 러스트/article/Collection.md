# 컬렉션

Rust의 표준 라이브러리에는 컬렉션이라고 불리는 매우 유용한 데이터 구조들이 여러 개 포함되어 있어. 대부분의 다른 데이터 타입들은 하나의 특정한 값을 표현하지만, 컬렉션은 여러 개의 값들을 담을 수 있지. 튜플이나 배열과는 다르게, 이 컬렉션들이 가리키는 데이터는 힙에 저장되기 때문에 컴파일 타임에 크기를 알 필요가 없어. 즉 컬렉션은 프로그램 실행 중에 그 크기가 커지거나 작아질 수 있다는 뜻이야. 

이 장에서는 Rust 프로그램에서 매우 자주 사용되는 세 가지 컬렉션에 대해 배워볼 거야:

- 벡터(Vector): 가변 개수의 값들을 저장할 수 있도록 해줘
- 스트링(String): 문자들의 모음이야
- 해쉬맵(Hash Map): 특정 키와 값을 연관시켜 저장할 수 있도록 해주는 좀 더 일반적인 구현체의 컬렉션이지

## 벡터

첫 번째 컬렉션 타입은 `Vec<T>`로 벡터라고 불러. 벡터는 같은 타입의 값을 메모리상에 이웃하도록 배치하여 저장할 수 있도록 해줘. 벡터는 오직 같은 타입의 값들만 저장할 수 있어.

새로운 벡터를 만들기 위해서는 `Vec::new` 함수를 사용해:

```rust
let v: Vec<i32> = Vec::new();
```

여기서 우리는 `v`에 타입 명시를 해줬는데, 이는 아직 어떤 값도 벡터에 넣지 않았기 때문이야. Rust는 우리가 어떤 타입의 요소를 저장하려는 건지 알 수 없어서 말이야.

더 일반적인 경우로, 우리는 초기값과 함께 벡터를 만들 텐데, 그러면 Rust가 우리가 저장하고자 하는 데이터의 타입을 추론할 수 있어서 타입 명시가 필요 없지. Rust는 `vec!` 매크로를 제공하는데, 이는 우리가 제공한 값들을 가지고 새 벡터를 생성해줘:

```rust
let v = vec![1, 2, 3];
```

벡터에 새 요소를 추가하려면 `push` 메소드를 사용할 수 있어:

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

벡터 내의 값을 읽는 방법은 두 가지가 있어. 여기 두 가지 방법을 보여주는 예제가 있어:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

첫 번째 `[]` 메소드는 우리가 존재하지 않는 요소에 접근하려고 하면 프로그램을 패닉에 빠뜨리지만, `get` 메소드는 `None`을 반환하지 패닉을 일으키지 않아.

## 스트링

우리는 1장에서 이미 스트링에 대해 다뤘지만, 이번에는 좀 더 자세히 살펴볼 거야. 

새로운 스트링을 만드는 많은 방법들이 벡터에서 사용 가능한 것들과 동일해. `new` 함수로 빈 스트링을 만들 수 있지:

```rust
let mut s = String::new();
```

문자열 리터럴로 `String`을 만들고 싶다면, `to_string` 메소드를 사용할 수 있어:

```rust
let data = "initial contents";
let s = data.to_string();
// 이 메소드는 리터럴에 직접 사용할 수도 있어:
let s = "initial contents".to_string();
```

스트링은 `+` 연산자나 `format!` 매크로를 이용해서 접합할 수 있어:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요
```

## 해쉬맵

마지막 컬렉션은 *해쉬맵*이야. `HashMap<K, V>` 타입은 `K` 타입의 키에 `V` 타입의 값을 매핑하여 저장해. 이는 해쉬 함수를 통해 이 매핑을 수행하는데, 이 해쉬 함수는 이 키와 값을 메모리 어디에 저장할지 결정해.

새로운 해쉬맵을 만들려면 `new`를 사용하고 `insert`를 통해 요소를 추가할 수 있어:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

해쉬맵에서 값을 가져오려면 `get` 메소드를 사용해:

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

해쉬맵을 순회하는 것도 가능해:

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

이렇게 Rust의 주요 컬렉션 타입들에 대해 알아봤어. 이 컬렉션들은 프로그래밍에서 매우 자주 사용되니 잘 익혀두면 좋을 거야!
