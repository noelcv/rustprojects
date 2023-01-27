# Error Handling

[Reference Guide](https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html)

## PANIC

Panic Behavior:

- print a failure message
- unwind
- clean up the stack
- quit
- BONUS: display the call stack (via env variable) `$ RUST_BACKTRACE=1 cargo run`

To prevent having to clean up the stack you can also set in the Cargo.toml to switch from unwindling to aborting

```toml
[profile.release]
panic = 'abort'
```

When a panic occurs, we can backtrace where it occurred (`src/main.rs:5:5`) or where the code was called

```bash
  error_handling git:(main) ✗  RUST_BACKTRACE=1 cargo run
   Compiling error_handling v0.1.0 (/Users/noel/rustprojects/error_handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/error_handling`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:65:14
   2: core::panicking::panic_bounds_check
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:151:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/slice/index.rs:259:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/vec/mod.rs:2736:9
   6: error_handling::main
             at ./src/main.rs:5:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/ops/function.rs:251:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

>*" A panic should not be used to communicate failure within the program. The default assumption is that caller functions will not try to catch panics."*

## Recoverable Errors

```rust
enum Result<T,E> {
  Ok(T),
  Err(E),
}
```

***Example***: Trying to read a file that doesn't exist

`T` -> the type of value to return in case of success
`E` -> type of Error in case of failure

Result is brought to the scope in the prelude, so no need to import it.

```rust
let greeting_file_result = File::open("hello.txt"); //returns a Result<T,E>
    
    
    //Basic error Handling 
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    //thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:16:23
    
    
    //Multiple errors handling
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    
    //with closures and unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    
    //SHORTCUTS: unwrap() 
    let greeting_file = File::open("hello.txt").unwrap(); //if ok the file is returned, if error the program calls a panic! macro
    
    //SHORTCUTS: expect() - a little bit like a try/catch in JS that helps convey intent and track down the source of the panic
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project, but couldn't be found");
        // prints: thread 'main' panicked at 'hello.txt should be included in this project, but couldn't be found: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:57:10
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        
```

***Expect should be preferred over unwrap in production code as it gives better custom error messages for context***

>"we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual."
>***"When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value. "***
>***"The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode"***

In a nutshell, use `panic!` when you want to decide on behalf of the caller that an error in unrecoverable.
This situations might include violations of a function contract, or when exposure to repeated inputs may be a threat (e.g, DoS attacks) and use `Result<T,E>` when you want to let the caller decide what to do in case of failure. 