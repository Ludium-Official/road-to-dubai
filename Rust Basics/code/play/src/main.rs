fn main() {
    println!("Hello, world!");

    
}

#[derive(Clone, Copy)]
struct Shoe {
    size: u32,
    name: &'static str,
}

impl  Shoe {
    fn x ( list: &[Shoe]) {
        let x= list[1];

    }
}
struct Config<'a> {
    query: &'a String,
    filename: &'a String
}
// 이렇게 구현할 수도 있지만, 조금 더 깔끔하게 하기 위해서
// 참조가 아니라 아예 데이터를 복사하는 (깊은복사) clone을 쓴다 

impl<'a> Config<'a>{

    #[warn(dead_code)]
    fn parse_config(list: &[String]) -> Config{
    let query = &list[1];
    // &list[1].clone()을 하면 list[1]이 아니라
    // 참조를 복사하므로 안된다! 
    let filename = &list[2];

    let result = Config{
        query: query,
        filename: filename,
    };
    
    result
    }

    fn new(list: &[String]) -> Result<Config,&str> {
    
    if list.len()<3
    {
        return Err("not enough args!");
        // 문자열 리터럴은 'static이다! 
    }

    let query = list[1].clone();
    // &list[1].clone()을 하면 list[1]이 아니라
    // 참조를 복사하므로 안된다! 
    let filename = list[2].clone();

    Ok(Config{query,filename})
    }
}


