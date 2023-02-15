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
6. We can add custom failure messages to document the meaning of an assertion

By introducing reliable tests, if we introduce bugs, it will be easier to spot them.

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

## `assert_eq!`and `assert_ne!`

Two very convenient methods of testing for equality or inequality.
the cool thing is that if it fails, it will show the mismatched results in the logs, which gives us a good starting point when debugging.

```rust
 #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(3));
    }
```

```bash
  ---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
left: `4`,
right: `5`', src/lib.rs:78:9

```

***Important: When implementing structs, we need to extend the directive to include PartialEq, if we are testing for equality or inequality***

```rust
#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//
#[test]
    fn test_ne_rect() {
        let rect1 = Rectangle {
            width: 9,
            height:5,
        };
        
        let rect2 = Rectangle {
            width:8,
            height:4,
        };
        
        assert_ne!(rect1, rect2); //the test succeeds if the params are not equal
    }
```

## Custom Failure Messages

```rust
#[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
        // assert!(result.contains("Caroll"));
//---- tests::greeting_contains_name stdout ----
//thread 'tests::greeting_contains_name' panicked at 'assertion failed: result.contains(\"Caroll\")', src/lib.rs:104:9
//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }
    
    #[test]
    fn greeting_contains_name_custom() {
        let result = greeting("Katia");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        )
    }  
//   ---- tests::greeting_contains_name_custom stdout ----
//thread 'tests::greeting_contains_name_custom' panicked at 'Greeting did not contain name, value was 'Hello, Katia'', src/lib.rs:114:9    
```

## `should_panic`

Check the full implemented example under Guessing Game

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1, got {},", value);
        };
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value)
        }
        Guess { value }
    }
    
    //getter method because the value field of Guess is private
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn make_a_guess(input: &str) -> Guess {
    let guess: i32 = input.trim().parse().expect("Please, type a number!");
    Guess::new(guess)
} 
#[cfg(test)]
mod tests {
    use super::*;  
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be greater or equal to 1")]
    fn smaller_than_1() {
        Guess::new(0);
    }
}
```

## `Result<T, E>`

```rust
    #[test]
    fn it_works_complete() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four! Blasphemy!"))
        }
    }

//test tests::it_works_complete ... ok


  #[test]
    fn it_works_complete() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four! Blasphemy!"))
        }
    }

//test tests::it_works_complete ... FAILED
//---- tests::it_works_complete stdout ----
//Error: "two plus two does not equal four! Blasphemy!"

```

When we use `Result<T, E>`, we cannot use should_panic. We can use the `assert!(value.is_err())`

## Controlling How Tests Run

`cargo run`compiles the code and runs the binary
`cargo test`compiles the code in test mode and runs that test binary
By default it happens in parallel

To override the default behavior, we can separate the command with `--`

`cargo test -- --help` to the left of the separator are commands to pass to cargo test, and to the right the ones to pass to the resulting test binary.

`cargo test -- --test-threads=1` will make the tests run in sequence, instead of in parallel. It's a good idea to make the tests totally independent from each other, though.

`cargo test -- --show-output` logs the values

### Running single tests and Filtering to Run Multiple Tests

Never been easier: `cargo test add_two_and_two`
Just add the function name, the remaining tests in the module are filtered out.

```bash
running 1 test
test tests::add_two_and_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s
```

Although we can't specify multiple tests to `cargo test` we can trick it like this:
`cargo test add` will run all the tests that contain `add` in the test function name:

```bash
    Finished test [unoptimized + debuginfo] target(s) in 0.85s
     Running unittests src/lib.rs (target/debug/deps/adder-d0b5831847bf2ea7)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::it_adds_two ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s
```

### Ignoring some tests

Just annotate the test with `#[ignore]`and it will be skipped

```rust
 #[test]
 #[ignore] 
 fn add_three_and_two() {
    assert_eq!(5, add_two(3));
}
```

```bash
   Finished test [unoptimized + debuginfo] target(s) in 1.28s
     Running unittests src/lib.rs (target/debug/deps/adder-d0b5831847bf2ea7)

running 3 tests
test tests::add_three_and_two ... ignored
test tests::it_adds_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 11 filtered out; finished in 0.00s
```

Use `cargo test -- --ignored` to run only the tests flagged with #[ignore]

To run all, `cargo test -- --include-ignored`

## Test Organization

### Unit Tests

Create a module `tests` annotated with #[cfg(test)] for each file in the **src** directory to tell the Rust compiler to run it only when you use `cargo test`and not `cargo build`

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

//private function 
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; //to take it into scope

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

```

### Integration Tests


1. Create a folder `tests` This directory becomes special and you don't need to annotate it with #[cfg(test)].

```rust
use adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
```

`$ cargo test` will run the cascade of test sections:

1. Unit tests
2. Integration tests
3. Doc tests

If any fails, the next doesn't run!!!

```bash
Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests src/lib.rs (target/debug/deps/adder-d0b5831847bf2ea7)

running 11 tests
test tests::add_three_and_two ... ignored
test tests::add_two_and_two ... ok
test tests::exploration ... ok
test tests::greeting_contains_name ... ok
test tests::greeting_contains_name_custom ... ok
test tests::it_adds_two ... ok
test tests::it_works ... ok
test tests::larger_can_hold_smaller ... ok
test tests::one_hundred_and_two ... ok
test tests::smaller_cannot_hold_larger ... ok
test tests::test_ne_rect ... ok

test result: ok. 10 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-92aefea4feb54615)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

To run just the integration test, run `cargo test --test integration_test` or whatever the name of your file will be.

Go to `cd adder` for a full implementation.

## Additional Readings

- [Benchmark Testing - (available only in Nightly Rust)](https://doc.rust-lang.org/unstable-book/library-features/test.html)
- [Documentation Comments as Tests `Doc-tests`](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)
