fn main() {
    print_labeled_measurement(5, 'h');


    let y = 6; // 우변에는 expression이 와야하는 것

    let y = {
        let x= 3;
        x+1
    };

    println!("{y}");

    let z= 6;
    let z =  plus_one(z);

    println!("{z}");

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x:i32) -> i32{
    x + 1
}