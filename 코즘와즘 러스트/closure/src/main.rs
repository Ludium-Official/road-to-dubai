use std::thread;
use std::time::Duration;
fn main() {
    
    /*
    함수형 프로그래밍 ->
    변경 가능한 상태를 불변상태(immutable)로 만들어 side effect를 없애자.
    모든 것은 객체이다.
    코드를 간결하게 하고 가독성을 높여 구현할 로직에 집중시키자.
    동시성 작업을 보다 쉽게 안전하게 구현 하자.
    */

    /*
    FnOnce: 클로저가 환경에서 값을 소유권으로 캡처. 한 번만 호출 가능.
    Fn: 클로저가 환경에서 값을 불변으로 빌려옴. 여러 번 호출 가능.
    FnMut: 클로저가 환경에서 값을 가변으로 빌려옴. 여러 번 호출 가능.

    별개로 move || -> 소유권을 가져옴 
    */
    let intensity = 10;
    let random_number = 7;
    generate_workout(intensity, random_number);

    let program = |x| { x+1 }; 
    println!("{}", program(1)); // rust analyzer를 보면 타입 추론을 이런 문맥을 가지고 함 ! 



}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}



struct Cacher<T>
where
    T: Fn(u32) -> u32, // 불변 참조해서 가져옴 
{
    calculation: T,
    value: Option<u32>,  // Option - Some or None
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 { // &self - 불변참조, self - 소유권이 넘어감
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // 이런 식으로 상태를 담을 수 있다 ! 
                v
            },
        }
    }
}
