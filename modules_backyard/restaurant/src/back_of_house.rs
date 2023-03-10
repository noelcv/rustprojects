pub mod back_of_house {
  pub fn fix_incorrect_order() {
      cook_order();

  }
  
  pub fn cook_order() {
      println!("Cooking order...");
  }
  
  pub struct Breakfast {
      pub toast: String,
      seasonal_fruit: String,
  }
  
  impl Breakfast {
      //because Breakfast includes a private field, 
      //we need to provide a public associate function to construct an instance
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seasonal_fruit: String::from("peaches"),
          }
      }
  }
  
  pub enum Appetizer {
      Soup,
      Salad,
  }
}