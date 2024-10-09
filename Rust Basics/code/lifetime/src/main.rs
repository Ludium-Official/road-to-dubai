fn main() {
    println!("Hello, world!");

    let mut r ;
    let x= 5;
    r = &x;
    println!("{}" , r);

    let y = r + 3;

    println!("{}",y);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); //result가 x,y중 더 작은 y보다 길게 살아있으므로 .. 컷 


}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
        //즉 위 함수에서는 "리턴 값의 lifetime이 최소한 x와 y 중 lifetime이 작은 것만큼은 된다"라는 뜻이다. 
    } else {
        y
        //즉 위 함수에서는 "리턴 값의 lifetime이 최소한 x와 y 중 lifetime이 작은 것만큼은 된다"라는 뜻이다. 
    }
}