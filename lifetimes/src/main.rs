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

fn main() {
    let r; //it's ok to declare a variable without initializing, so it leaves in this outer scope                                             ---+---- 'a
    // println!("r: {}", r); // but this will throw an error at compile time, because we're trying to use something that hasn't been defined     
    // ^ `r` used here but it is possibly-uninitialized
    
    {
        let x = 5; //the scope of x is limited to this block
        r = &x;
    } // `x` dropped here while still borrowed
    
    // println!("r: {}", r); //   ^^ borrowed value does not live long enough
    // 
    
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
    }
    // println!("The longest string is {}", resulting_result); 
    // would not compile because resulting_result goes out of scope 
    //as it's bound to the result of the longest.
    
} //end of scope of string3
