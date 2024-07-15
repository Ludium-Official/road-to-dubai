use std::alloc::dealloc;

fn main() {
    let person = Person{
        name: "hello".to_string(),
        birth: 213,
    };

    let mut dict: Vec<Person> = Vec::new();

    dict.push(person);

    let name = dict[0].name.clone();


}

struct Person{
    name:String,
    birth: i32,
}
