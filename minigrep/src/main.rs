use std::env; //bring it to scope so we can use args()
use std::fs; //to handle file system operations
use std::process; //will be used to exit the program if there's an error

fn main() {
    let args: Vec<String> = env::args().collect(); //turn the iterator into a collection of values, in this case a Vector of strings
    // dbg!(args);
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1) // signaling that the the program has exited with an error (non-zero exit code)
    });
    
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

impl Config {
    //by having 'static as the lifetime, we're saying that error message will live as long as the program
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        //wrap the values in a Config struct in the Ok variant
        Ok(Config { query, file_path }) //the Config will own the values of query and file_path
    }
}