#[derive(Debug,Clone)]
pub struct Shoe{
    pub size: u32 ,
    pub name: String,
}
pub fn shoes_in_my_size(shoes:Vec<Shoe>, shoe_size:u32) -> Vec<Shoe>{
    shoes.into_iter()
        .filter(|s| s.size == shoe_size) 
        .collect()        
}

pub fn print_shoe(args: &mut [Shoe]) {
    let x: Vec<Shoe> = args.into_iter()
    .map(|s| { 
        s.size = s.size+1;
        s.name = String::from("good"); //접근은 가능한데 소유권은 없음 참조로 받아서 
        s.clone()
    })
    .collect();

    print!("{:?}", x);
}

pub fn gogo(str: String) {
    print!("{}",str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let shoes = vec![
            Shoe { size: 10, name: String::from("sneaker") },
            Shoe { size: 13, name: String::from("sandal") },
            Shoe { size: 10, name: String::from("boot") },
        ];

    }
}
