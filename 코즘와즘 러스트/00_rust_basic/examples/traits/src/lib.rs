pub mod summary; 
use crate::summary::Summarizable;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Kal {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
} 

impl Summarizable for Kal {}

#[cfg(test)]
mod tests {
    use super::*;

}
