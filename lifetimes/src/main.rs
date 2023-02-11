use std::fmt::Display;


//We need to use generic lifetimes parameters, because neither us nor the borrow checker
//know in advance how long the references will be valid for.
// we're basically saying: the function takes two references to string slices and the
//the returning reference will be valid as long as the two parameters are valid, so it will
//have the lifetime of the shortest of the two parameters.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//we annotate the lifetime of the struct so we can use it in the body of the struct
struct ImportantExcerpt<'a> {
    part: &'a str, //this field part will have a reference to a string slice, and an instance of this struct will leave as long as the reference does, and no longer than that.
}

//although we annotate the impl and its use after the struct name,  we don't need to annotate the lifetime of self reference because of the first rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    //in this case, we have multiple inputs, so the lifetime of &self is passed to the return type
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//this wouldn't compile in previous versions, it needed explicit annotations of lifetimes like this:
//fn first_word<'a>(s: &'a str) -> &'a str {}
//this would result in entering the same lifetime annotations reppeatidely, so we can use the elisions rules
//to make the code more concise as the borrow checker can infer the lifetimes under these rules. 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//putting it all together: Generic Type Parameters, Trait Bounds and Lifetimes;
//this function will print the announcement and return the longest string slice from either x or y
//<'a, T> - lifetime is a kind of generic, so we pass it within angle brackets after the function name
fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T,) -> &'a str
where
    T: Display, //the where clause specifies that the generic type `T` must implement the `Display` trait...
    {
        println!("Announcement! {}", ann); //...because it will be needed here to print the announcement within the {}
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

fn main() {
    let r; //it's ok to declare a variable without initializing, so it leaves in this outer scope                                             ---+---- 'a
    // println!("r: {}", r); // but this will throw an error at compile time, because we're trying to use something that hasn't been defined     
    // ^ `r` used here but it is possibly-uninitialized
    
    {
        let x = 5; //the scope of x is limited to this block
        r = &x;
        println!("inner `r` is valid here: {}", r);
    } // `x` dropped here while still borrowed
    
    // println!("r: {}", r); //   ^^ borrowed value does not live long enough
    
    let r = 2;
    println!("r: {}", r); //r: 2
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("the longest string is {}", result);
    } //end of scope of string4
    
    let resulting_result;
    {
        let string4 = String::from("xyz");
        resulting_result = longest(string3.as_str(), string4.as_str()); // borrowed value does not live long enough
        println!("The longest string is valid here in the inner scope {}", resulting_result); 
    }
    // println!("The longest string WOULD NOT BE VALID HERE {}", resulting_result); 
    // would not compile because resulting_result goes out of scope 
    //as it's bound to the result of the longest.
    
    
    let novel = String::from("Call me Bond, James Bond. Once upon a time...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    

    //this i is an instance of ImportantExcerpt,
    let i = ImportantExcerpt {
        part: first_sentence, 
    };
    println!("novel: {}", novel);
    println!("i.part: {}", i.part);
    
    
    let first_wrd = first_word(&novel);
    println!("first word: {}", first_wrd);
    
    println!("i.level(): {}", i.level());
    println!("i.announce_and_return_part(): {}", i.announce_and_return_part("This is a really important announcement, so pay attention"));
    
    //this will live as long as the program lives.
    let some_string_literal: &'static str = "I have a static lifetime";
    println!("some_string_literal: {}", some_string_literal);
    
    //WRAP IT UP
    let wrap_it_up = longest_with_an_announcement("This is the X string slice", "This is the Y string slice - let's make it longer", "This is an announcement in the form of a string slice, which implements the Display trait and therefore can be printed in curly braces!");
    println!("wrap_it_up should be the longest string slice from the previous call: {}", wrap_it_up);

} //end of scope of string3
