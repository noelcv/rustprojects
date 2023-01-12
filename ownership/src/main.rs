fn main() { //st is not valid here, because it was not yet declared
    
    // st value is hardcoded and immutable
    // string literals are known at compile time
    let st = "hello";
    println!("{}", st); //prints: `hello`
    
    // this kind of string is mutable
    //String Type - is great when we don't know the size in advance. Manages data allocated on the heap
    //we allocate a slot an compile time, then it will be requested by the memory allocator at runtime
    //we need a way to return its memory to the allocator, so we use the String::from function
    let mut s = String::from("hello");
    
    s.push_str(", world"); // push_str() appends a literal to a String
    println!("{}", s); //prints: `hello, world`
    
    let s1 = String::from("Hola");
    let s2 = s1;
    
    //this would not work because s2 now owns the value of s1, and s1 is out of scope
    //in other words, we try to borrow a value that has moved
    // println!("{}, mundo!", s1); 
    println!("s2 = {}", s2); //prints: `Hola`
    
    //wwe can also clone to explicity copy or duplicate the value
    //clone is expensive, because the copy is being duplicated on the heap
    let s3 = String::from("Clone string");
    let s4 = s3.clone();
    
    println!("s3 = {}, s4 = {}",s3, s4);
    
    //given integers have a known size at compile time, they are stored on the stack and easily accessible
    //if for instance we would annotate a variable with the Copy trait, the value wouldn't be moved, but copied 
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y); //any group of scalars can implement the Copy trait - including integers, floats, boolean, char and tuples of these types only
    
    //OWNERSHIP AND FUNCTIONS
    let my_string = String::from("hello"); //my_string comes into scope
    
    takes_ownership(my_string); //my_string's values move into the function are no longer available herein after
    
    let xpto = 5; //xpto comes into scope aka becomes available
    
    makes_copy(xpto); //because 5 is an integer (i32), it can implement Copy, so in this case, it's being copied into the function but remains available
    println!("xpto just to prove the point that {} remains available after being copied to the function makes_copy = {}", xpto, xpto); //prints 5; also {} {} require two arguments
    
    let receiver = gives_ownership(); //gives_ownership moves its return value into receiver
    
    let some_other_string = String::from("I will jump around");
    
    let wild_receiver = takes_and_gives_back(some_other_string);
    
    println!("receiver = {}, wild_receiver = {}", receiver, wild_receiver); 
    //prints: `receiver = I will be passed to the caller, wild_receiver = I will jump around`
    
    let yet_another_string = String::from("let's find out the length of this string");
    
    let (that_yet_string, length_of_yet_another_string ) = calculate_length(yet_another_string);
    
    println!("that_yet_string = {}, has a length of {}", that_yet_string, length_of_yet_another_string);
    //prints: `that_yet_string = let's find out the length of this string, has a length of 40`
    
    
    // let otpx = if true {1} else {"hello"}; //not allowed, because the types are different
    // assert_eq!(optx + 1,2)
    
    // this will also not compile, because the compiler assumes, s6 was moved when calling move_two, even though it was not used
    // let (s5, s6) = (String::from("a"), String::from("b"));
    // let s7 = move_two(s5, s6);
    
    // println!("{}, {}", s6, s7) //it won't compile
    
} //neither is s valid here, because it's out of scope. The curly brace calls the drop() function

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);   
} //some_string goes out of scope and drop() is called. The memory is freed

fn makes_copy(some_integer: i32) { 
    println!("a copy of {}", some_integer);
}

//write a function that gives ownership
fn gives_ownership() -> String {
    let some_string = String::from("I will be passed to the caller");
    some_string   //note, no ; 
}

//let's go crazy
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

//returning multiple values
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn move_two(s1: String, s2: String) -> String {
    s1
}