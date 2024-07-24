fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    //let r3 = &mut s; 
    
    println!("{}",r1); // immutable한 상태에서만 borrow가 일어날 수 있다!

    change(&mut s);// 얘가 지금 여기 있어서 동작하지, 앞에 있으면 mutable한 위험이 있어서 동작 안함.

    println!("{}",s);

}

fn change(s:&mut String) {
    s.push_str(", world!");
}
