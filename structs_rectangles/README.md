# Structs - An Example Program Using Structs

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 5](https://rust-book.cs.brown.edu/ch05-02-example-structs.html) of The Book.

Some quick notes:

## Structs provide meaning

``` rust
[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}
```

It's way better to have a clear meaning in the way we relate our coding blocks.
In the example, it's very clear what it means: the function borrows an immutable reference to a Rectangle struct and returns the area calculated from its properties `width` and `height`

```rust
fn area_with_structs(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}
```

## Additional behavior using derived traits

Using the derive keyword we can extend the default behavior of our structs, giving them custom functionality. We'll see how later, how to implement our own traits.

### Readings

[Derived Traits](https://rust-book.cs.brown.edu/appendix-03-derivable-traits.html)

- `Debug` -  For Programmer Output;
- `PartialEq` and `Eq` - for Equality Comparisons (`==` and `!=`);
- `PartialOrd` and `Ord` - for Ordering Comparisons(`<`, `>`, `>=`);
- `Clone` and `Copy` - for Duplicating Values;
- `Hash` - for Mapping a Value to a Value of Fixed Size;
- `Default` - for Default Values.

## Methods
