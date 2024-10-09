fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let ip_of_me = Ip{
        V4 : (127,0,0,1),
        V8: String::from("::1"),
    };

    let four = IpAddrKind::V4; // 단순한 열거형 variant
    let six = IpAddrKind::V6;  // 단순한 열거형 variant

    let four = IpAddrKind::V4(127,0,0,1); //열거형은 이런 식으로 하나만 빼서 정의가 가능

    let six = IpAddrKind::V6(String::from(":::1"));

    route(four);
    route(six);

    println!("{:?}",ip_of_me.V4);

    let m = Message::Write(String::from("hello"));
    m.call();

    

}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

struct Ip {
    V4: (u8,u8,u8,u8),
    V8: String
}
enum Message {
    Quit,
    Move {x: i32, y:i32}, // 익명 구조체 
    Write(String), // String
    ChangeColor(i32, i32, i32), // 3개의 i32
}
impl Message {
    fn call (&self) {

    }
}

fn route(ip_type: IpAddrKind){
    println!("{:?}",ip_type);
}
