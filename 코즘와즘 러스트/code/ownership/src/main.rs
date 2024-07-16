fn main() {
    {
        // scope start 
        let s = "hello";
        // s가 유효
    }
    // scope end 



    // 문자열 리터럴 - 스택에 하드 코딩

    let mut s = String::from("hello"); // 문자열 객체 String 타입
    s.push_str(", world!");
    println!("{}",s); // hello,world를 출력하게 된다
    // Rust는 변수가 밖으로 벗어나면 drop함수를 호출한다

    let s1 = String::from("hello!");
    let s2 = s1;
    // s1이 s2로 이동했다 . s1이 더이상 유효하지 않음 
    // 이 시스템을 통해서 (중복 해제)를 방지합니다 
    // Rust는 절대로 깊은 복사를 수행하지 않는다

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    // s1, s2 깊은 복사를 수행한다 


    let mut s = String::from("hello");  // s가 스코프 안으로 들어옵니다

    takes_ownership(s);   // s의 값이 함수의 파라미터로 옮겨감
    

}

fn takes_ownership(mut some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("{}", some_string);
    some_string.push_str("x"); //대신, println!은 인자로 전달된 값을 참조로 받아서 사용합니다.
    println!("{}",some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 drop 호출? 

// 반환값과 스코프 