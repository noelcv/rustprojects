# Structs - Methods

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 5.3 Method Syntax](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html) of The Book.

Some quick notes:

- Methods always have a `&self` reference
- Methods are scoped within the `impl` block
- Associated functions return a new instance of the struct:
  - Example: `String::from("hello world")`

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    //ASSOCIATED FUNCTIONS
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
