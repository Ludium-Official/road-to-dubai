fn main() {
    let data = "initial contents";
    // 스트링 리터럴 , 스트링 슬라이스 

    let s = data.to_string();
    //String 타입

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    // 두 코드의 동작은 동일하다 


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    //s2이라는 불변(immutable) 문자열 슬라이스(&str) 변수를 선언하고, "bar"라는 문자열로 초기화합니다.
    println!("s2 is {}", s2);

    mystr(s2);
    println!("s2 is {}", s2);
    // s2는 소유권이랑 무관한 객체 
}

fn mystr(param: &str) {
    let a = param;
}
