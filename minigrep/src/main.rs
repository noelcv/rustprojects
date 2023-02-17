use std::env; //bring it to scope so we can use args()
use std::fs; //to handle file system operations

fn main() {
    let args: Vec<String> = env::args().collect(); //turn the iterator into a collection of values, in this case a Vector of strings
    // dbg!(args);
    
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file"); //returns a std::io::Result<String>

    println!("With text: \n{contents}");
}

