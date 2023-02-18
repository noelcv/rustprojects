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


[Refactoring to Improve Modularity and Error Handling](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html)

- main.rs - will call the CLI logic with args / setup configuration / call a run function in lib rs / error handling
- lib.rs - will handle the logic
