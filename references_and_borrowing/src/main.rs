//Both pointers and references provide addresses to access stored data.
//But while pointers can be moved, a reference is bound to a given value for its lifetime.
//i,e, like variables are immutable by default, so are references.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    //change(&s1); //`some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutab
    
    //MUTABLE REFERENCES
    let mut s2 = String::from("hello");
    println!("BEFORE: {}", s2);
    change(&mut s2);
    println!("AFTER: {}", s2);
    
    //DANGLING REFERENCES
    // let reference_to_nothing = dangle();
    let will_own_it = no_dangle();
    println!("will_own_it grabbing ownership of the return value of calling no_dangle: {}", will_own_it);
    
}

//here we borrow the value without taking ownership of it - s will be of type &String instead of String
//s is a reference to a String, which is a reference to stored value of s1.
fn calculate_length(s: &String) -> usize {
    s.len()
}

//This doesn't work because we can't change something we borrowed, but to do not own.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//MUTABLE REFERENCES
fn change(some_string: &mut String) {
    // the original string has to be mutable
    // the function signature indicates that will accept a mutable reference to a String &mut
    //the change function will now change the value it borrows
    some_string.push_str(", world");
}
// &s --> immutable reference of type &String
// &mut s --> mutable reference of type &mut String
// & cannot be used to change the value of the reference, because it's again just borrowing something it doesn't own.

//NOTE: You can only have one mutable reference to a variable:
//example: let r1 = &mut s; let r2 = &mut s; //error

//You cannot mix and match mutable and immutable references.


//DANGLING REFERENCES
//this doesn't work because the function returns a reference to a String, but  the scope of s is dropped by the end of the function and it's dropped.
// a returning reference would be pointing to an invalid String
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
//this works, ownership is moved out of the function and returned to the caller
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}





//PS: the opposite of referencing (&) is dereferencing(*) - more of it later on Chapter 8 and 15