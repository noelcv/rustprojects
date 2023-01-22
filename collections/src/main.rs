fn main() {
    let _v: Vec<i32> = Vec::new();
    
    //the macro vec! allows to create a vector with values and let it infer the type
    let mut v = vec![1,2,3];
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v); //prints: [1, 2, 3, 5, 6, 7, 8]
    
    //Reading elements
    
    let third: &i32 = &v[2]; //gives a reference to the element at the specified index
    println!("The third element is {}", third); //prints: The third element is 3
    
    //this use case is suitable when handling a vector with an unknown number of elements, because it won't panic if it does't find it
    //also for error handling and provide UI/UX feedback
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element inside the match is {}", third),
        None => println!("No third element found"),
    }
    
    // let does_not_exist = &v[100];
    // println!("The 100th element is {}", does_not_exist); //panics: thread 'main' panicked at 'index out of bounds: the len is 7 but the index is 100',

    let does_not_exist = v.get(100);
    println!("The 100th element is {:?}", does_not_exist); //prints without panicking: The 100th element is None
    
    let mut v1 = vec![1,2,3,4,5];
    let _first = &v1[0];
    v1.push(6);
    // println!("The first element is: {}", first); // the rules of ownership and borrowing still apply
    //so this line would not compile, as we kept an immutable reference to the first element
    //and adding a new element would require allocating new memory and copy old elements around.
    //there would be a risk of pointing at deallocated memory.
    
    //ITERATING over the values in a vector
    let v3 = vec![100, 32, 57];
    for n_ref in &v3 {
        let n_plus_one = *n_ref + 1; //dereferencing to get the value - see chapter 15
        println!("{}", n_plus_one);
    }
    println!("------------");
    
    //now iterating over mutable references
    //we use the * to grab the value in the reference and then we update it
    let mut v4 = vec![100, 32, 57];
    for n_ref in &mut v4 {
        //n_ref has type &mut i32
        *n_ref += 50;
        println!("{}", n_ref);
    }

    //using Enums to store multiple types
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    //this is needed because the compiler needs to know the types at compile time to plan the memory allocation on the heap for each element
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("bananas")), SpreadsheetCell::Float(21.01)];

    //STORING UTF-8 ENCODED TEXT WITH STRINGS
    let mut s = String::new();
    println!("s content is: {}", s); //prints: s content is:
    let data = "initial contents";
    
    //1.
    let s = data.to_string();
    println!("s content is: {}", s); //prints: s content is: initial contents
    //2.this would also work
    let s = "another way to create a string".to_string();
    println!("s content is: {}", s); //prints: s content is: another way to create a string
    
    //3. using String::from
    let s = String::from("yet another way to create a string from a string literal");
    println!("s content is: {}", s); //prints: s content is: yet another way to create a string from a string literal
    
    
    //Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s content is: {}", s); //prints: s content is: foobar
    
    let mut s1 = String::from("hello");
    let s2 = " world";
    s1.push_str(s2);
    //push_str takes a string slice as an argument, and note that the ownership is not being taken
    //so s2 will still be valid after this operation
    println!("s1 content is: {}", s1); //prints: s1 content is: hello world
    println!("s2 content is: {}", s2); //prints: s2 content is:  world
    
    let mut s3 = String::from("lo");
    //push takes a single character as a parameter in SINGLE QUOTES
    s3.push('l');
    println!("s3 content is: {}", s3); //prints: s3 content is: lol
    
    //CONCATENATION with the + operator
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //string concatenation requires an owned `String` on the left + one or more references to a String on the right
    
    let s3 = s1 + &s2; //so s1 is moved here and can no longer be used;
    println!("s3 content is: {}", s3);
    //under the hood, the + operator calls the add method with such a signature:
    // fn add(self, s: &str) -> String
    //through coercion s2 of type &String becomes &str
    //i,e, &s2 is, by deref coercion, &s2[..]
    
    //CONCATENATION with the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s content is: {}", s); //prints s content is: tic-tac-toe;
    
    //INDEXING
    
    let s1 = String::from("hello");
    // let h = s1[0]; //^^^^^ `String` cannot be indexed by `{integer}`
    //Rust strings do not support indexing
    
    //A `String` wraps over a `Vec<u8>`
    let hello = String::from("Hola"); //len is 4 bytes - each letter takes 1 byte (UTF-8)
    let hello = String::from("Здравствуйте"); //len is 24 bytes - each Unicode scalar value takes 2 bytes (UTF-8)
    //therefor indexing would be be misleading and hence is not supported
    // let answer = &hello[0]; //this doesn't work, but if it would, it would return the byte value, not the character (208) in the second case, and (104) in the first
    
    
    //SLICING STRINGS
    
    let hello = "Здравствуйте";
    // if you really need to slice a string, you have to specify a range of bytes
    let s = &hello[0..4];
    println!("s content is: {}", s); //prints: s content is: Зд - because again, each char here takes 2 bytes
    // let s = &hello[0..1];
    // println!("s content is: {}", s); panics: thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
    
    //METHODS FOR ITERATING OVER STRINGS
    
    for c in "Зд".chars() {
        println!("{}", c);
        //prints: 
        //3 
        //д
        
    }
    
    for b in "Зд".bytes() {
        println!("{}", b);
        //prints:
        //208
        //151
        //208
        //180
    }
}
