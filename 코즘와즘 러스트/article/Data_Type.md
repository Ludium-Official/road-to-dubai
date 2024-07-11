# Data_Type

Rust에서 사용되는 모든 값들은 어떤 타입을 갖고 있기 Rust에게 어떤 형태의 데이터인지 명시해야해. Rust는 타입이 고정된 언어이기 때문에 모든 변수의 타입이 컴파일 시에 반드시 정해져 있어야 해. 
```
let guess: u32 = "42".parse().expect("Not a number!");
```
parse 함수를 통해 String을 숫자로 변환시키고 guess앞에 u32 데이터 타입을 명시하는 것을 확인할 수 있지. 이와 반대로, 아래와 같이 타입을 명시하지 않으면 
```
let guess = "42".parse().expect("Not a number!");
```
아래와 같이 오류가 나는 것을 확인할 수 있어. 
```
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

## 스칼라 타입들
스칼라는 하나의 값으로 표현되는 타입으로, Rust는 정수형, 부동소수점 숫자, boolean, 그리고 문자, 네 가지 스칼라 타입을 보유하고 있어. 

### 정수형 타입
정수형 타입은 signed unsigned / bit 수에 따라 표와 같은 다양한 타입들을 사용할 수 있어.  
![image](https://github.com/mmingyeomm/nestJS/assets/87323564/cd52cdf1-2099-4054-a843-11297c618a22)

### 부동 소수점 타입 

Rust는 f32와 f64 두가지의 부동 소수점 타입이 존재해.  
```
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### boolean 타입 
대부분의 다른 언어들처럼, boolean 타입은 Rust에서 true와 false 둘 중 하나의 값만을 가질 수 있어. 
```
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### 문자 타입 
Rust는 문자 또한 지원해. 또한, Rust의 char타입은 Unicode Scalar를 표현하는 값이고 이는 ASCII 보다 많은 표현을 가능하게 하해. 억양 표시가 있는 문자, 한국어/중국어/일본어 표의 문자, 이모티콘, 넓이가 0인 공백문자 모두가 Rust에서는 char타입으로 사용할 수 있어.
```
let guess: char = 'f';
let heart_eyed_cat = '😻';
```


## 복합 타입 

복합 타입들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있어. Rust는 두 개의 기본 타입들을 갖고 있지 각각 튜플과 배열이야.

### 튜플 타입 

튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법이.

우리는 괄호 안에 콤마로 구분되는 값들의 목록을 작성하여 튜플을 만들어. 튜플에 포함되는 각 값의 타입이 동일할 필요없이 서로 달라도 됩니다. 다음의 예제에 우리는 선택 사항인 타입 명시를 추가했어.
```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
튜플 값을 밖으로 빼내오기 위해서는 튜플을 구조해체하는 작업이 진행돼야 해. 아래와 같이 let (x, y, z,) 를 선언하여 튜플의 값을 각각 x,y,z로 이동시킬 수 있지. 
```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```
구조해체 말고도, 마침표(.) 뒤에 우리가 접근하길 원하는 값의 색인을 넣는 것을 통해 튜플의 요소에 직접적으로 접근할 수 있어. 
```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### 배열 
배열은 튜플과는 다르게 배열 안의 모든 요소가 같은 타입이어야 해.  Rust의 배열이 몇 다른 언어들의 배열과 다른 점은 Rust에서는 배열은 고정된 길이를 갖는다는 점이야: 한번 선언되면, 이들은 크기는 커지거나 작아지지 않.
```
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
배열은 벡터타입과는 다르게 가변적이지 않기 때문에, 고정된 값을 가지는 요소들을 위해서는 배열을 선택할 수 있어.  
다른 언어와 마찬가지로 배열은 아래와 같이 접근할 수 있어.  
```
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
