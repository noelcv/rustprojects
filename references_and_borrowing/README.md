# Ownership - References and Borrowing

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 4.2](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html) of The Book.


>***"At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid."*** -- from the docs.


References, like variables, are immutable by default. `&` is different than `&mut` . Only with the latter, can we modify that default behavior.

`&` just reads. you can have multiple references.

`&mut` reads and writes.

# Only one borrower at once
This is to prevent data races. The compiler won't allow data races such as this:

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{},{}, r1, r2")
```

While that would not work, the example below would

```
let mut s = String::from("hello");
{
  let r1 = &mut s;
} //end of scope for r1

let r2 = &mut s;
```

Remember the 3 rules of Ownership? They still apply here:
```
1. Each value has a owner

2. Only one owner at a time

3. Once a owner goes out of scope, the value is dropped.

```

That's why you cannot combine mutable and immutable references in the same scope.

From the docs:
```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

```
while the above is not okay, the example below will compile,because once r1 and r2 have been invoked at that println! macro, they will go out of scope.
```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

```

`Non-Lexical Lifetimes` or NLL is the ability of the compiler to check whether a reference is still in use within a given scope.
