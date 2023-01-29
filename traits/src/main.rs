pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
    pub source: String,
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.source)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn main() {
    let tweet_one = Tweet {
        username: String::from("luke_skywalker"),
        content: String::from("Who is my true father?"),
        reply: false,
        retweet: false,
    };
    
    let tweet_two = Tweet {
        username: String::from("darth_vader"),
        content: String::from("I am your father."),
        reply: true,
        retweet: false,
    };
    
    println!("[new tweet] {}", tweet_one.summarize());
    println!("[new tweet] {}", tweet_two.summarize());
    
    let article_one = NewsArticle {
        headline: String::from("Shocking revelation for Star Wars fans!"),
        location: String::from("Planet Earth"),
        author: String::from("Herr Pulitzer"),
        source: String::from("Not The Real Times"),
        content: String::from("Luke Skywalker found out this morning that Darth Vader is his father, in a shocking revelation on Twitter.")
    };
    
    println!("[BREAKING NEWS] {}", article_one.summarize());
}
