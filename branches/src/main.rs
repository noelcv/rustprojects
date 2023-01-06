fn main() {
    //IF STATEMENTS
    let number = 10;
    // the condition must be a bool
    // otherwise, having something like if number { /*do something*/ }
    // would throw mismatched types error at compile time.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Rust doesn't convert automatically non-Booleans to booleans
    //so, be EXPLICIT and always provide a boolean as a condition for an if statement
    //e.g.,

    if number != 0 {
        println!("this statement will be printed only if number is different than 0, which is the case for number {number})");
    }


    //we can even assign the result of evaluating an if statement to a (let) variable,
    //because if is in an expression
    let condition = true;
    let conditional_number = if condition { 5 } else { 6 };
    // the arms of the if else statement need to be of the same type,
    // because the compiler needs to know the one and only type of the variable.
    // e.g. if condition { 5 } else { "six" } would throw an error[E0308]: `if` and `else` have incompatible types
    println!("Because the condition is {condition}, the conditional_number will be {conditional_number}");
    
}
