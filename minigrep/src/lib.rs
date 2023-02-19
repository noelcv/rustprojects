use std::error::Error; //will be used to return errors from the run function
use std::fs; //to handle file system operations
use std::env; //to read the environment variables with the var() function

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //by having 'static as the lifetime, we're saying that error message will live as long as the program
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); //we're just checking if an env var is set- The value is not relevant here
        
        //wrap the values in a Config struct in the Ok variant
        Ok(Config { query, file_path, ignore_case }) //the Config will own the values of query, file_path and ignore_case
    }
}

//contains the logic of the program from read the file, onwards
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?; // the ? will return the error value instead of panicking (in case it fails to read the file)
  
  let results = if config.ignore_case {
    println!("Insensitive search");
    search_case_insensitive(&config.query, &contents)
  } else {
    println!("Sensitive search");
    search(&config.query, &contents)
  };
  
  for line in results {
    println!("{line}");
  }
  
  Ok(()) //indicates success and the unit type () indicates that we don't have a value to return
}

//we connect the signature lifetime to the lifetime of the contents argument, as we're returning a slice of the contents
//containing the lines that match the query and not the other way around
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //requirements:
    let mut results = Vec::new();
    //Iterate through each line of the contents.
    for line in contents.lines() { //lines() returns an iterator over the lines of a string
      //Check whether the line contains our query string.
      if line.contains(query) {
        //If it does, add it to the list of values we’re returning.
        results.push(line);
      }
      //If it doesn’t, do nothing.
    }
    //Return the list of results that match
    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  //requirements:
  //parse the query to lowercase and repeat the same steps as in search()
  let query = query.to_lowercase(); // create a new String with the lowercase version of the query
  let mut results = Vec::new();
  
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) { 
      results.push(line); //we'll still be pushing the original line and not the lowercase version used only for comparison
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  
  //the test function describes the desired behavior: given a query and contents, return only lines that contain the query
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    
  }
  
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}