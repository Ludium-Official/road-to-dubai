extern crate traits;
use crate::traits::Tweet;
use crate::traits::Kal;
use crate::traits::summary::Summarizable;
fn main(){
    let s = Tweet{
        username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,};

    println!("1 new tweet! : {}\n",s.summary());

    let k = Kal{
        username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,};

    println!("New Kal! {}", k.summary());
}
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
} // 트레잇 바운드, Display와 Clone이 구현되어있는 타입만 통과시킨다. 아니면 컴파일이 불가능함
 