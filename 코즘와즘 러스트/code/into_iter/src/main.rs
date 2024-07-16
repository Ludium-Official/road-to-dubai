extern crate into_iter;

use into_iter::*;
fn main(){
    let mut shoes = vec![
        Shoe { size: 10, name: String::from("sneaker") },
        Shoe { size: 13, name: String::from("sandal") },
        Shoe { size: 10, name: String::from("boot") },
    ];

    print_shoe(&mut shoes);

}