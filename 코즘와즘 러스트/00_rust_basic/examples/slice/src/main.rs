fn main(){
    let mut s = String::from("hello world"); 
    //힙 데이터 - 소유권 관리 대상 (구조체- ptr,len,capacity)

    let z = &mut s;

    let word = first_word(z);

    // let z = &mut s; 이러면 mutable한 slice와 함께 존재
    // -> 그러면 mutable한 참조가 2개 존재하므로 데이터 경합 발생 - 에러

    //s.clear(); // Error!

    println!("the first word is: {}", word);


    let s = "Hello, world!"; // 문자열 리터럴은 &str 타입이다. (불변임 기본적으로)

    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
}


fn first_word(s: &mut String) -> &mut str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &mut s[0..i];
        }
    }

    &mut s[..]
}
