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
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("bananas"), SpreadsheetCell::Float(21.01)),];
    
}
