pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}