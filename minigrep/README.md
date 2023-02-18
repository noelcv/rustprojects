# MinigrepIO - A project based on Chapter 12

`cargo run -- needle haystack`

```bash
[src/main.rs:5] args = [
    "target/debug/minigrep", //is the name of our binary - &args[0]
    "needle",
    "haystack",
]
```

```bash
➜  minigrep git:(main) ✗ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/minigrep needle haystack`
Searching for needle
In file haystack
```

We added the `poem.txt` at the root of the project,

```bash
➜  minigrep git:(main) ✗ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text: 
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

## Separation of concerns

Let's split the code to make it modular and easier to test, instead of having main parsing arguments and reading files. 
Group configuration variables: tuple -> struct 

- we're using the clone method right now, but it is slightly inneficient because we're allocating an entire copy of that data in memory, instead of borrowing a reference
- we gain simplicity and clarity because we don't have to manage the lifetimes of the data, but it has a runtime cost - (here is not significant though)
- by adding query and file path to struct, we convey meaning how both elements relate

- Convert `parse_config` into an associated function `new` of the Config struct - idiomatic approach. Note that an associated function doesn't have access to the internal state of instances of a type, but are rather called **on the type itself**.

## Error Handling

Create meaningful error handling

```bash
➜  minigrep git:(main) cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:26:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Having the code panicking if the len of args is lower than 3 is better than nothing, and provides context to developers, but we can do better...

```bash
➜  minigrep git:(main) ✗ cargo run
   Compiling minigrep v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/minigrep`
thread 'main' panicked at 'Not enough arguments', src/main.rs:27:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Returning a Result instead of panic!

- Return a Result with a Config in case of success, and an error description in case of failure
- Change new -> build ()
- Refactored main to handle the Result with the unwrap_or_else method from the std library to define custom error message.
  In case it's Ok, it will return the Result
  In case it's Err, the method calls the code in the closure (`|err|` - vertical pipes), in this case will exit the program and use the err value at runtime.

WAY BETTER!!!

```bash
➜  minigrep git:(main) ✗ cargo run
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/minigrep`
Problem parsing arguments: Not enough arguments
```

### Back to Refactoring

- Extract the logic in main to a function run to hold the logic for configuration and error handling
- Refactor error handling at fn run -> bring `std::error::Error` into scope and use the trait object `Box<dyn Error>` to updated the return type to `Result<(), Box<dyn Error>>` - the function will return a type that implements the Error trait, dyn stands for dynamic, should we need to return error values of different types for different errors
- replace `expect` for `?` to return the error in case of error instead of panicking
- In case of success, Ok(()) is the idiomatic way of indicating we're calling the function for its side effects (e.g. printing to the console) without having a returning value.
- use `if let Err(e)`to handle the error, if there is an error. Again, the success of run() has no return value, so here the priority is to handle the error.

### Code Splitting

- lib.rs - will handle the logic
- main.rs - will call the CLI logic with args / setup configuration / call a run function in lib rs / error handling
  
Note: remember to make the API public, and bring it to the binary's crate scope (main.rs) from lib.rs

With such a modular code, it will be easier to test it.

## Test-Driven Development

>"1. Write a test that fails and run it to make sure it fails for the reason you expect.
> 2.Write or modify just enough code to make the new test pass.
> 3.Refactor the code you just added or changed and make sure the tests continue to pass.
> 4.Repeat from step 1!"


## Environment variables

See the code implementation for details.

```bash
➜  minigrep git:(main) ✗ IGNORE_CASE=1 cargo run -- to poem.txt
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.42s
     Running `target/debug/minigrep to poem.txt`
Insensitive search
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
➜  minigrep git:(main) ✗ cargo run -- to  poem.txt             
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Sensitive search
Are you nobody, too?
How dreary to be somebody!
```

[Refactoring to Improve Modularity and Error Handling](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html)
[Test-Driven Development](https://rust-book.cs.brown.edu/ch12-04-testing-the-librarys-functionality.html)