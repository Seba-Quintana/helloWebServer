pub trait Summary {
    // both declarations work, but you cant define the same method twice 
    // so I commented the first one
    // fn summarize(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    // this method defines a default behaviour, 
    // and can be overwritten by any type that 
    // implements it
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// here NewsArticle is implementing Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
/*
    * implementing trait method (or overwriting default one)
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2    
}
