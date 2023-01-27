# Collections

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 8 - Common Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html) of The Book.

We're about to dive into:

## Vectors `Vec<T>`

- a kind of array that needs to know the types in advance, so that the compiler knows how much memory will need to allocate for each element
- Additional readings: [The Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html),
- [The Vec API docs](https://doc.rust-lang.org/std/vec/struct.Vec.html) - **push**, **pop**, etc...
- Their values are dropped once they go out of scope

## Strings

- Strings are a wrapper around a vector of bytes

- using `"foo.to_string()"` is equivalent to `String::from("foo")`
- Strings are UTF-8 encoded. so we can have things like:
  
  ```rust
  let hello = String::from("السلام عليكم");
  let hello = String::from("Здравствуйте");
  ```
  
- We can update a string with `.push_str("whatever")`
- the `+` operator implements the `add` method, which takes ownership of the first variable, takes a reference of the second, third...strings and copies it and then returns back ownership of the first moved variable (so it's not exactly a copy)

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
//string concatenation requires an owned `String` on the left + one or more references to a String on the right
    
let s3 = s1 + &s2; //so s1 is moved here and can no longer be used;
println!("{}", s3) //prints: Hello world!
```

- Use the `format!` macro for improved readability of more complex string concatenation. `format!` returns a String with its content, instead of printing it to the screen (`println!`)

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

//use
let s = format("{}-{}-{}", s1, s2, s3)
println!("{}",s) //prints: tic-tac-toe

//instead of
let s = s1 + "-" + s2 + "-" + s3 
```

- Indexing is not supported for strings in Rust, because UTF-8 scalar values may take a variable amount of bytes, so the results would be misleading, and at most would return the byte value, not the character.

```rust
let hello = String::from("hello")
let hello = String::from("Здравствуйте")
let h = &hello[0] //not valid
```

- UTF-8 from Rust's perspective: - example: ` “नमस्ते” `
  1. **bytes**

    ```rust
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //18 bytes
    ```

  2. **scalar values** - (Rust's `char` type)

  ```rust
  ['न', 'म', 'स', '्', 'त', 'े']
  //6 char values, but the 4th and 6th are not letters
  ```
  
  3. **grapheme clusters** (the closest thing to letters)

  ```rust
  ["न", "म", "स्", "ते"]
  // the 4 "letters"
  ```
  
Finally, indexing is expected to be a constant time O(1), and in such setup is just not possible to guarantee such performance without traversing each of the characters to determine its size upfront.

***CAUTION: If you are going to make a slice out of a string, you need to specify a range of bytes***

```rust
let hello = String::from("Здравствуйте")
let mySlice = &hello[0..4] // "Зд" - each of these Unicode scalar values takes 2 bytes
let hello = String::from("hello")
let mySlice = &hello[0..4] //"hello"

//The best way to iterate over strings is to specify whether you want characters or bytes
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
```

The standard library includes methods like `contains` and `replace`, that can help out, though.

>"&str is a promise that the byte sequence it points to will always be valid UTF-8. Therefore a programmer who wants to e.g. print out an &str never needs to check if it is valid, or worry about accidentally interpreting an invalid string."

## HashMaps `HashMap<K, V>`

- If type implement Copy trait (e.g, `i32`, `f64`, `bool`) -> the values are copied into the HashMap
- else, for owned values like String, the values are moved and the hash map will take ownership of those values
- Each key in a HashMap can only have one value at the time, but you can overwrite it
- Using the `entry` API, you can also check if a key is present in a Hashmap, insert it if not present, or update the values accordingly
- You can also update based on an old value, but when iterating over the hashmap, make sure to dereference the value using the `*` to update a scoped variable, the values will be dropped once the loop is done, and only the updated HashMap will remain
- PS: Attention to use the `mut` keyword when instantiating an HashMap. Otherwise, you will not be able to change it.
- Side note: HashMaps in Rust have by default implemented a Hashing Function called SipHash(provide resistance against DoS attacks on hash tables), but you can specify another hasher using the `BuildHasher` trait.
