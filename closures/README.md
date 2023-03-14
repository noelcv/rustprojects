# Closures

Closures can capture values from its lexical scope - but somehow looks different than JS...

**Initial hypothesis** : in the sense that in Rust we can grab the context and reference it from elsewhere, and not only from the surrounding scope.

Explore the code annotations in `src/main.rs` to find out more about it.

## Closure Type Inference and Annotation

Unlike functions, that need to be strictly typed to establish a wide consensus on how the code is expected to run and how the compiler shall build it, closures have a narrower space of usage and therefore the compiler can infer its type, in the vast majority of the cases.

The verbose option is still...possible...as an option :)

```rust

let expensive_closure = |num: u32| -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
}

```

> Listing 13-2

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 } //function annotation
let add_one_v2 = |x: u32| -> u32 { x + 1 }; //fully typed closure annotation
let add_one_v3 = |x|             { x + 1 }; // annotation without the inferred types
let add_one_v4 = |x|               x + 1  ; // without the brackets as the closure's body as only one expression, in this case `x+1`

//add_one_v3 and v4 - its types will be inferred from its usage

let example_closure = |x| x; // the type of x is inferred from the type of the argument
    let s = example_closure(String::from("hello")); //At this point the compiler will infer that x is of type String...
    println!("s: {:?}", s); //s: "hello"
    
    // and this will then fail...
    // let n = example_closure(5); //expected struct `String`, found integer - mismatched types
    let n = example_closure(5.to_string()); //this will work
    println!("n: {:?}", n); //n: "5"
```

>**"Top-level functions can be part of a library's external interface, while closures cannot be directly exposed"**

## Three Traits

### FnOnce

- can be called only once
- able to move values after their body
- all closures implement at least this trait

### FnMut

- don't move captured values out of their bodies
- but might mutate them
- these closures can be used multiple times

### Fn

- don't move captured values
- don't mutate
