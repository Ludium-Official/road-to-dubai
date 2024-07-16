fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // refer of s1! 

    let len = calculate_length_1(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    //s.push_str("!"); // not work! -> 기본적으로 immutable
    s.len() // irrelevent to owneship
}

fn calculate_length_1(s:&mut String) ->usize{
    s.push_str("!!!");
    s.len()
}


