pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    
    
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
        
        assert_ne!(rect1, rect2);
    }
    
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
    
}
