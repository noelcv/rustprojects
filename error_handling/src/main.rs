// use std::fs;
// use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;


//It's up to the code calling this function to decide what to do based on its Result value, ie, either a String or an Error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    //here is where the propagation error happens -- assuming username_file is Ok we'll pass it as parameter to read_to_string
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    //"We don’t have enough information on what the calling code is actually trying to do, 
    //so we propagate all the success or error information upward for it to handle appropriately."
}


//SAME FUNCTION but with ? Operator - Shortcut

// fn read_username_from_file_2() -> Result<string, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// //even shorter
// fn read_username_from_file3() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// //Even shorter!!! but without explaining the error
// fn read_username_from_file4() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt"); 
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


//the Box<dyn Error> type is a trait object -- "any kind of error"
fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");
    
    // let v = vec![1,2,3];
    // v[99];
    //thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:5
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //Unlike C, Rust doesn't allow for buffer overread, i.e reading past the end of a data structure
    
    // let greeting_file_result = File::open("hello.txt"); //returns a Result<T,E>
    
    
    //Basic error Handling 
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    //thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:16:23
    
    
    //Multiple errors handling
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };
    
    // //with closures and unwrap_or_else
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    
    //SHORTCUTS: unwrap() - return or show error
    
    // let greeting_file = File::open("hello.txt").unwrap(); //if ok the file is returned, if error the program calls a panic! macro
    
    //SHORTCUTS: expect() - a little bit like a try/catch in JS that helps convey intent and track down the source of the panic
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project, but couldn't be found");
        // prints: thread 'main' panicked at 'hello.txt should be included in this project, but couldn't be found: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:57:10
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // let greeting_file = File::open("hello.txt")?;
    //^ cannot use the `?` operator in a function that returns `()`
    
    let greeting_file = File::open("hello.txt")?;
    
    Ok(())

}


