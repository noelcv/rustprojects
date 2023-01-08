# The Slice Type

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 4.3](https://rust-book.cs.brown.edu/ch04-03-slices.html) of The Book.

>***"Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership."*** - from the docs

To create a slice use a range within brackets: `[starting_index..ending_index]`. Note that the ending index shall be the last position that you want + 1.

Example from the docs:
```
fn main() {
    let s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s; // not a slice, for comparison
}

```

The slice holds the *starting_index* and its *length*, i.e., the ending_index minus the starting_index.

> *"So in the case of `let world = &s[6..11];`, `world` would be a ***slice*** that contains a ***pointer*** to the __byte at index 6__ of `s` with a length value of 5."*

![Slice](https://rust-book.cs.brown.edu/img/trpl04-06.svg) Figure 4-6 from Chapter 4.3


`&String` -> is a reference to string

`&str` -> a string slice


## String literals are slices!!

String literals are immutable, because their type is a `&str` which is a an immutable reference to a specific point of the binary.

To clarify, `char`, string literals and strings are different things.

```
//single quote, primitive data type, 4 bytes, represents a Unicode Scalar Value
let a_char: char = 'z';

let a_string_literal: &str = "hello world";
// a combination of chars, immutable by default

let owned__and_mutable_string: String = "hello world".to_string();

let string_value_from_string_literal: String::from("hello");

```

String literals are stored in the binary of the program, while strings are stored on the heap.


## Other Slices

We can also slice arrays of integers, just like in the example below.

```
    let a = [1, 2, 3, 4, 5];
    
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2,3]);
    println!("slice: {:?}", slice);
```

## Final considerations

Rust provides memory safety at compile time. This means less bugs, less debugging, faster and more reliable code.

Memory safety is largely achieved by these concepts of ownership, borrowing and slices.
