use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::vec;
pub fn run(config: &Config) -> Result<(), Box<dyn Error> > {

    let mut f = File::open(&config.filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())

}
// 그냥 config를 쓸 시 config의 소유권을 가지게 되며, config.filename과 config.query 필드의 소유권도 함께 가지게 됩니다. 
//따라서 config.filename을 사용할 때 별도의 참조자를 사용할 필요가 없습니다.

/*

struct Config<'a> {
    query: &'a String,
    filename: &'a String
}
- 이렇게 구현할 수도 있지만, 조금 더 깔끔하게 하기 위해서
참조가 아니라 아예 데이터를 복사하는 (깊은복사) clone을 쓴다 
*/
#[derive(Debug)]
pub struct Config{
    pub query: String,
    pub filename: String
}

impl Config{

    #[warn(dead_code)]
    pub fn parse_config(list: &[String]) -> Config{
    let query = list[1].clone();
    // &list[1].clone()을 하면 list[1]이 아니라
    // 참조를 복사하므로 안된다! 
    let filename = list[2].clone();

    let result = Config{
        query,filename,
    };
    result
    }

    pub fn new(list: &[String]) -> Result<Config,&str> {
    
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// 아래처럼 구현하면 result의 변경가능한 그런 타임을 막을 수 있음
// 이는 병렬 프로그래밍 등의 이점을 가질 수 있다 
pub fn new_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}