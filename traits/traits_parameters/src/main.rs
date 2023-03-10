use std::fmt::{Display, Formatter, Result, Debug};

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

//decluttering multiple trait signatures with `where` clauses
//instead of this: 
pub fn another_function_without_where_clauses<T: Display, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    33
}

pub fn another_function< T, U>(t: &T, u:&U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug {
    println!("t is: {}", t);
    println!("u is: {:?}", u);
    33    
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

//We can also return types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("yoda"),
        content: String::from("To be or not to be, question is."),
        reply: true,
        retweet: false,
    }
}

// Implemnent methods conditionaly on a generic type that depends on a trait bound
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y)
        }
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
    
    let x = 3;
    let y = "hello";
    let result = another_function(&x, &y);
    println!("result: {}", result);
    
    let summarized = returns_summarizable();
    println!("summarized: {}", summarized.summarize());
    
    //tessting that we can convert integers to string because integers implement the Display Trait from the standard library
    //impl <T: Display> ToString for T {...} - blanket implementation
    let s = 3.to_string();
    println!("{}", s);
}
