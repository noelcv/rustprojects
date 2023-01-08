fn main() {
    let mut st = String::from("hello world");
    
    let word = first_word_without_slice(&st);
    
    println!("The variable word is : {}", word); //5
    
    st.clear(); //empty the string;
    
    //word is still 5, but the string is empty;
    println!("The variable word is : {}, but st is {}", word, st); //5
    
    //STRING SLICES in action
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; //slice
    let world: &str = &s[6..11]; //slice
    let not_a_slice: &String = &s; //string
    println!("hello: {}, world: {}, not_a_slice: {}", hello, world, not_a_slice);
    
    let len = s.len();
    let another_hello: &str = &s[..5];
    let another_world: &str = &s[6..len];
    println!("hello: {}, world: {}", another_hello, another_world);
    
    let slice = &s[0..len];
    let another_slice = &s[..];
    println!("Slices of the entire string - slice: {}, another_slice: {}", slice, another_slice);
    
    let test_string = String::from("hello world");
    let first_word = first_word_with_slice(&test_string);
    let second_word = second_word_with_slice(&test_string);
    println!("The first word is: {}", first_word);
    println!("The second word is: {}", second_word);
    
    //
    let test_word = first_word_with_slice(&test_string); //if we have an immutable reference to a string...
    println!("The test word is: {}", test_word);
    // test_string.clear(); // ...we cannot use a mutable reference to the same string. //REMEMBER THE RULES
    
    let sample_string = String::from("my hello world");
    
    let the_first_word = get_first_word(&sample_string[..]);
    println!("testing that get_first_word works on a slice - the_first_word: {}", the_first_word); //prints: my
    
    let the_first_word = get_first_word(&sample_string[..6]);
    println!("testing that get_first_word works on a slice - the_first_word: {}", the_first_word); //prints: my
    
    let the_first_word = get_first_word(&sample_string[3..]);
    println!("testing that get_first_word works on a slice - the_first_word: {}", the_first_word);//prints: hello
    
    let the_first_word = get_first_word(&sample_string);
    println!("testing that get_first_word works on a Reference to a String - the_first_word: {}", the_first_word); //prints: my
    
    let hardcoded_string_literal = "hello literal world";
    let word = get_first_word(&hardcoded_string_literal);
    println!("testing that get_first_word works on a hardcoded string literal because a string literal is already a slice!!! - word: {}", word); //prints hello
    let word = get_first_word(&hardcoded_string_literal[..]);
    println!("testing that get_first_word works on a whole slice of a string literal - word: {}", word); //prints: hello
    let word = get_first_word(&hardcoded_string_literal[6..]);
    println!("testing that get_first_word works on a partial slice of a string literal - word: {}", word); //prints: literal
    
    
    //OTHER SLICES
    let a = [1, 2, 3, 4, 5];
    
    let slice: &[i32] = &a[1..3];
    println!("slice: {:?}", slice);
    println!("The result of the assertion is {:?}", assert_eq!(slice, &[2,3]));
}

//write a function that retunrs the first word in a string
//without SLICE
fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes(); //convert String to an array of bytes
    
    //iter() returns each element in a collection
    //enumerate() is a wrapper to the result of iter() as a tuple
    // the for (i, &item) is a destructuring pattern of the tuple 
    // that returns the index and a reference to the item (byte)
    for (i, &item) in bytes.iter().enumerate() {
        //b is a byte literal, in this case to find a space
        if item == b' ' {
            return i; //return the position
        }
    }
    s.len() //if no space is found, we return the length of the string
}

//WITH SLICE

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }
    &s[..]
}

//WITH SLICE and &str signature we can use the function on both
// a reference to a string (&String) and a slice from a string value(&str)
//Stay tuned for deref coercions (chapter 15)
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s[..]   
}
