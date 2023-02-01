use std::fmt::{Display, Formatter, Result};

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
        pub source: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}({})", self.headline, self.author, self.source)
        }
    }
    


impl Display for NewsArticle {
    fn fmt (&self, f: &mut Formatter) -> Result {
        write!(f, "{}, by {}  ({})", self.headline, self.author, self.source)
    }
}


//Using Traits as Parameters in functions that accept any type that implements a Trait
//using `impl trait` syntax aka syntax sugar for the more verbose syntax below
pub fn notify(item: &impl Summary) {
    println!("[NOTIFY: BREAKING NEWS], {}", item.summarize());
}


//using the trait bound syntax
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("[NOTIFY VERBOSE BREAKING NEWS], {}", item.summarize());
}

//While here the `impl trait` syntax allows for any type that implements the Summary trait to be passed as an argument to the function...
pub fn notify_multiple_parameters(item1: &impl Summary, item2: &impl Summary) {
    println!("[NOTIFY MULTIPLE PARAMETERS: BREAKING NEWS 1], {}", item1.summarize());
    println!("[NOTIFY MULTIPLE PARAMETERS: BREAKING NEWS 2], {}", item2.summarize());
}


//...here the generic type T used trait bound syntax requires / constrains that both arguments are of the same type and implement the Summary trait
pub fn notify_multiple_parameters_strict<T: Summary> (item1: &T, item2: &T) {
    println!("[NOTIFY MULTIPLE PARAMETERS STRICT: BREAKING NEWS 1], {}", item1.summarize());
    println!("[NOTIFY MULTIPLE PARAMETERS STRICT: BREAKING NEWS 2], {}", item2.summarize());
}

//WITH MULTIPLE TRAITS
pub fn notify_with_several_traits(item: &(impl Summary + Display)) {
    println!("[NOTIFY WITH SEVERAL TRAITS: BREAKING NEWS], {}", item.summarize());
}

//WITH MULTIPLE TRAITS AND `+` SYNTAX
pub fn notify_with_plus_syntax<T: Summary + Display>(item: &T) {
    println!("[NOTIFY WITH + SYNTAX] {}", item.summarize());
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
    
    //Using Traits as Parameters in functions that accept any type that implements a Trait
    notify(&article_one);
    notify_verbose(&article_one);
    notify(&tweet_one);
    notify_verbose(&tweet_two);
    
    notify_multiple_parameters(&article_one, &tweet_one);
    notify_multiple_parameters_strict(&article_one, &article_one);
    
    notify_with_several_traits(&article_one);
    notify_with_plus_syntax(&article_one);
}
