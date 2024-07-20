fn main() {
    println!("Hello, world!");

    let your_coin = Coin::First(String::from("this is your coin!"));

    let bank_account = prize(your_coin);

    println!("your received is : {}", bank_account);
    

    let num : Option<i32> = Some(bank_account.into());

    println!("{:?}", 
    match num {
        None => None,
        Some(_i)=> Some(_i+1),
        //match는 타입이 동일해야한다. 타입 매치가 되어야하므로 some!
    })


}
fn prize(coin: Coin)->u8{
    match coin {
        Coin::First(s) => {
            println!("{}",s);
            100 
        },
        Coin::Second(_i) => 50,
        Coin::Third(_i) => 10,
        Coin::Fourth(_i) => 0,

    }
}
enum Coin {
    First(String),
    Second(u8),
    Third(u8),
    Fourth(u8),
}