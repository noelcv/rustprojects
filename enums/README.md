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
