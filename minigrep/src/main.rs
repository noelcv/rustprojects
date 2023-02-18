use std::env; //bring it to scope so we can use args()
use std::fs; //to handle file system operations

fn main() {
    let args: Vec<String> = env::args().collect(); //turn the iterator into a collection of values, in this case a Vector of strings
    // dbg!(args);
    
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file"); //returns a std::io::Result<String>

    println!("With text: \n{contents}");
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path } //the Config will own the values of query and file_path
}