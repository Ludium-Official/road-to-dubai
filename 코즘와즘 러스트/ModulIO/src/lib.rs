pub mod logic;

#[cfg(test)]
mod test {
    use super::*;//현재 모듈(lib.rs)의 항목의 모든 것을 가져온다 
    use logic::*;
    // use logic::sub_module::*; 이래야 하위모듈도 가져온다 

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            logic::search(query, contents)
        );
    }
}

