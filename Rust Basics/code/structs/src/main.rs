fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("nyj"), 
    String::from("luke"));

    println!("{}", user2.username);

    let rect1 = Rectangle{length: 50, width: 30};

    println!("rect1 is {}", rect1.length * rect1. width);

    println!("rect1 is {:#?}", rect1);

    println!("the area is {}", rect1.area());
    // Rust에서의 . 연산자는 자동으로 추론함. 읽는지, 소비하는지, 변경하는지 ...  



}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    } // also expression
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
        // 소유권을 가져오고 싶지 않음 
    }
}