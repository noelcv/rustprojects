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

fn main() {
    let r; //it's ok to declare a variable without initializing, so it leaves in this outer scope                                             ---+---- 'a
    // println!("r: {}", r); // but this will throw an error at compile time, because we're trying to use something that hasn't been defined     
    // ^ `r` used here but it is possibly-uninitialized
    
    {
        let x = 5; //the scope of x is limited to this block
        r = &x;
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
    
} //end of scope of string3
