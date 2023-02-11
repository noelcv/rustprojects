# Testing

[Reference Guide for Testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

>*"Correctness in our programs is the extent to which our code does what we intend it to do. "* - The Book

In this chapter we cover unit tests and integration tests.

Basic principles as in any other language:

"1. Set up any needed data or state;
2. Run the code you want to test;
3. Assert the results are what you expect."

>*Whenever we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.*
`cargo new adder --lib`

## Things to note

1. `#[test]` annotates a function as a test;
2. `assert!` is a testing macro to test for truthiness
3. `cargo test` will run the tests
4. We can run a subset of the tests by filtering
5. `Doc-tests` is for helping having documentation of the results

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }
    
    #[test]
    fn it_fails_for_sure() {
        assert_eq!(2 + 2, 5)
    }
    
    #[test]
    fn it_fails() {
        panic!("Make this test fail")
    }
}

```

>*"Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed."*

```bash
 cargo test
   Compiling adder v0.1.0 (/Users/noel/rustprojects/testing/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.80s
     Running unittests src/lib.rs (target/debug/deps/adder-d0b5831847bf2ea7)

running 4 tests
test tests::exploration ... ok
test tests::it_works ... ok
test tests::it_fails ... FAILED
test tests::it_fails_for_sure ... FAILED

failures:

---- tests::it_fails stdout ----
thread 'tests::it_fails' panicked at 'Make this test fail', src/lib.rs:28:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::it_fails_for_sure stdout ----
thread 'tests::it_fails_for_sure' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:22:9

failures:
    tests::it_fails
    tests::it_fails_for_sure

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

## `assert!`

See the full example `cd adder/src/lib.rs`. 

```rust
 #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        //we expect it to be false, so we use the bang operator to negate the value inside the assertion
        assert!(!smaller.can_hold(&larger));
    }
```

## Additional Readings

- [Benchmark Testing - (available only in Nightly Rust)](https://doc.rust-lang.org/unstable-book/library-features/test.html)
- [Documentation Comments as Tests `Doc-tests`](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)
