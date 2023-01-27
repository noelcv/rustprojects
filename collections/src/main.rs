use std::collections::HashMap;

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
    
    //--------HashMap<K, V>---------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Yellow");
    
    //the `get` method returns an `Option<&V>`
    // if no option is found, it returns None
    // `copied` will get an Option<i32> instead of a Option<&i32>
    //the unwrap will set the score to 0 if a entry for the matching key doesn't exist
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score of {}: {}", team_name, score);
    //Yellow: 50
    //Blue: 10
    //Red: 0
    
    
    //iterating over HashMaps
    // we take a reference of the hashmap &scores
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    //Hashmaps and ownership
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //the ownership of field_name and field_value is moved to the hashmap
    //so cannot use them here anymore
    println!("{:?}", map); //prints: {"Favorite color": "Blue"}
    // println!("{}", field_name) //error: value moved when inserting to the HashMap
    // if we'd insert references to values into the hashmap, the values wouldn't be moved
    // but their lifetime should remain valid at least as long as the hashmap is valid
    
    //Updating a HashMap
    //each key can only have one value at a time (but not the other way around)
    
    //1. Overwriting a value;
    
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue"), 50);
    println!("overwriting Blue score at scores: {:?}", scores);
    //prints: overwriting Blue score at scores: {"Blue": 50, "Yellow": 50}

    //2. Adding a Key anv Value only If a key is not present
    //using the 'entry' API -> returns a enum called Entry
    
    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Green")).or_insert(50);
    println!("scores after adding green: {:?}", scores);
    //prints: score after adding green: {"Blue": 50, "Yellow": 50, "Green": 50}
    //because yellow already exists, the value passed into the `.or_insert()` is not added
    
    //3. Updating a value based on the old value
    
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    //we iterate over the words and for each,
    // we check if the word already exists and increment its count
    // or we insert the word with 0 and increment it to 1
    //"the `split_whitespace`method returns an iterator over the sub-slices separated by whitespaces"
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
        //we have to dereference the value of the mutable reference count
        //but its Safe to do it because
        //at the end of the loop, all mutable references will go out of scope
        //and only the map will remain
    }
    
    println!("{:?}", map); 
    //the order is arbitrary when iterating over a hashmap
    //prints: {"hello": 1, "world": 2, "wonderful": 1}
    //or: {"world": 2, "wonderful": 1, "hello": 1}
    //etc...
    
    //use the mut to be able to perform changes
    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    println!("h BEFORE: {:?}", h);
    //prints: h BEFORE: {}
    for (i, c) in "hello!".chars().enumerate() {
        //println!("i: {}, c: {}", i, c);
        //prints: 
        //i: 0, c: h
        //i: 1, c: e
        //i: 2, c: l
        //i: 3, c: l
        //i: 4, c: o
        //i: 5, c: !
        h.entry(c).or_insert(Vec::new()).push(i); 
    }
    println!("h AFTER: {:?}", h);
    //prints: h AFTER: {'o': [4], 'e': [1], 'l': [2, 3], 'h': [0], '!': [5]}
    
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("sum: {}", sum); //prints: sum: 5
    
}
