use std::ops::Deref;
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //assert_eq!(5, y);
    // 같은 타입이 아니라서 오류! ref와 i32는 엄연히 다름

    let y = MyBox::new(x);

    assert_eq!(5,x);
    //assert_eq!(5,*y);
    
    println!("{}",*y);
    //assert_eq!(&x,y);
}

struct MyBox<T> (T,i32);

impl <T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x,1)
    }
}

impl <T> Deref for MyBox<T> { // 이러는 것은 트레잇을 구현하기 위함임. for은 트레잇을 구현하기 위함! 
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
    
    
}