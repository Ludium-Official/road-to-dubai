use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
    // join은 소유권을 가져가는 메서드이다 ! 

    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum:usize = numbers.into_iter().sum();
        sum/len
    }); // 클로저의 리턴값은 join이 호출될 떄 Result로 감싸져 호출된다 

    let average = t.join().unwrap();
    // 결과값을 unwrap! join만 하면 Result 타입이다.

    println!("{}", average);
}