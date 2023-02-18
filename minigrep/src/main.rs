use std::env; //bring it to scope so we can use args()
use std::process; //will be used to exit the program if there's an error

use minigrep::Config; // bring the Config struct from the library crate (minigrep - lib.rs) into the binary crate's scope

fn main() {
    let args: Vec<String> = env::args().collect(); //turn the iterator into a collection of values, in this case a Vector of strings
    // dbg!(args);
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1) // signaling that the the program has exited with an error (non-zero exit code)
    });
    
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);
    
    //As the success return value of run is (), there is nothing to unwrap so we're only interested in handling the error with if let Err(e)
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
