# Enums and Pattern Matching

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 6](https://rust-book.cs.brown.edu/ch06-00-enums.html) of The Book.

Quick notes

By attaching data to a variant of an enum directly, we avoid replicating structs' boilerplate.

E.g. instead of this...

```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
}
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1")
}
```

... we can have this...

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}
```

Another example of how we can use enums to combine different types:

```rust
enum Message {
    Quit, 
    Move { x: i32, y: i32 }, 
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

now let's look at the equivalent with structs... way more verbose 

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

An enum is some sort of super type aggregator.
We can use `impl` with it to define methods.


### Rust doesn't have NULL!!!!!
Because trying to use a null value as not-null value would throw an error,
Rust has a different approach for expressing the concept of a value that is currently invalid or absent

```rust
enum Option<T> {
    None,
    Some(T)
}
```

This Option is made available without need to import it.

`Option<T>` and `T` are different types.

```rust

let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;

//this would not compile.
```

In other words, it wouldn't work, because if we tell the compiler we want a value of a type, Rust will make sure we only accept valid values.

Otherwise, we need to perform some sort of checks to use the `T` out of the `Option<T>`. -> to prevent the risk of assuming a not-null value incorrectly

Basically, 

- If there is a possibility of having a null value, use `Option<T>` and explicitly handle it
- otherwise, you can assume that the value will not be null.

To use the value `T` we'll need to define some code for each variant and use `match` expression as a control flow tool to check the variants.
