# Variable and Constant 

Rust에서 변수는 let을 통해 정의할 수 있어. 여기서 주의할 점은, 다른 프로그래밍 언어들과 다르게 러스트의 기본 변수는 **불변성**이라는 것이야. 불변성이기 때문에, 변수가 한번 선언되었을 때, 이 변수의 값은 변경할 수가 없어.  

```
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
아래 코드를 실행하면, 불변이어야 할 x를 6으로 변경하려고 했기 때문에 오류가 나는 것을 확인할 수 있어. 
```
error[E0384]: re-assignment of immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ re-assignment of immutable variable
```
## 불변성이 왜 필요할까?

Rust가 기본 변수를 불변성으로 둔 이유는 Rust가 제공하는 안전성과 손쉬운 동시성이라는 장점을 취할 수 있도록 코드를 작성하게끔 강제하는 위해서야.

Rust에서는 컴파일러가 변경되지 않은 값에 대한 보증을 해주고, 실제로 이는 바뀌지 않지. 이것이 의미하는 바는 코드를 작성하거나 분석할 시에 변수의 값이 어떻게 변경되는지 추적할 필요가 없기 때문에 코드를 더 합리적으로 만들어준다는 거야.

그럼에도, 가변성은 매우 유용하게 사용될 수 있기에 Rust는 가변성을 mut 키워드를 변수에 추가하는 것을 통해 제공해. 

### 가변 변수 

Rust는 변수명의 접두어로 mut을 추가하는 것을 통해 가변성 변수를 선언할 수 있어

```
fn main() {
    let mut x = 5;  
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

위와 같이 mut을 통해 프로그램을 실행한다면 코드가 잘 실행되는 것을 확인할 수 있어. 

### Constant 

Rust의 기본 변수가 불변성을 지닌다고 했는데, 이는 constant와 어떤 차이를 가질까?  
Rust에서 constant는 const 키워드를 통해 생성할 수 있어.  

- const는 기본 변수와는 달리, 기본 설정이 불변성인 것이 아니라 불변성 그 자체라고 할 수 있어.  
- const는 뒤에서 배우게 될 Data Type을 반드시 선언해야 한다는 특징이 있지. 
- const는 전체 영역을 포함하여 어떤 영역에서도 선언될 수 있어 
- const는 오직 const 표현식만 설정될 수 있지, 함수 호출의 결과값이나 그 외에 실행 시간에 결정되는 값이 설정될 수는 없어.
```
const MAX_POINTS: u32 = 100_000;
```

### Shadowing

Rust는 Shadowing을 통해 이전에 선언한 변수와 같은 이름의 변수를 선언할 수 있고 새 변수는 이전 변수를 shadow하게 돼. 예시 코드를 통해 살펴보자
```
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```
이 프로그램은 처음 x에 값 5를 bind 하는데, 이후 반복된 let x = 구문으로 x를 shadow하고 원본 값에 1을 더해서 x의 값은 6이 되는 것을 확인할 수 있어. 같은 원리로 x = x*2 를 실행하여 아래와 같은 결과가 나오지. 
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

mut와는 다르게 shadowing은  let키워드를 사용하지 않고 변수에 새로 값을 대입하려고 하면 컴파일-시에 에러가 얻게 돼. 또한, 값을 변경할 수는 있지만 그 이후에 변수는 그대로 불변성을 갖게 되는 것이지.  
더 많은 예시를 통해 shadowing을 이해해보자.  
```
let spaces = "   ";
let spaces = spaces.len();
```
위와 같이 문자열 유형의 변수 spaces를 선언하고 이를 다음 spaces에 shadowing 하는 것을 확인할 수 있지. 

shadowing을 할 때 주의해야 할 경우에 대해 알아보자. 

```
let mut spaces = "   ";
spaces = spaces.len();
```
위와 같이 shadowing을 하게 된다면, 컴파일 시 에러가 나는 것을 확인할 수 있어. 이는 mut로 선언된 spaces 변수를 불변성을 지닌 기본변수에 shadowing을 시도할 경우 에러가 나는 것을 확인할 수 있어. 
