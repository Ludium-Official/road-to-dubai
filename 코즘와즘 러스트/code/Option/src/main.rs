fn main() {
    println!("Hello, world!");
    //let number = Option<String>::Some(String::from("String"));
    //사용자 정의 열거형 사용할 때 이렇게 쓰고,

    let number: Option<String> = Some(String::from("hello"));

    let x: i8 = 7;
    // 컴파일 타임에 0이 아님을 확신하고 자신있게 쓸 수 있음
    let y: Option<i8> = Some(5);
    // Option인 경우에는 값이 있는지 없는지 검사가 필요함

    // 따라서 둘 사이에서는 사칙연산을 수행할 수 없다
    //println!("{}", x+y);



    println!("{:?}",number);

}
//enum Option<T> {
//    Some(T),
//    None,
//} Option의 정의 