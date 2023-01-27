fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

//we need to specify not only the generic type, but its capabilities
//in this case, we want to compare values so we use PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    let numbers_list = vec![34, 50, 25, 100, 65];
    
    let mut largest_number = numbers_list[0];
    
    for number in numbers_list {
        if number > largest_number {
            largest_number = number;
        }
    }
    println!("[NAIVE APPROACH] The largest number is {}", largest_number);
    
    //imagine we would to the same for another list...boring! so we'll make function instead
    
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest_i32(&number_list);
    println!("[with fn largest_i32] the largest_i32 is {}", result);
    
    let another_numbers_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&another_numbers_list);
    println!("[with fn largest_i32] the largest_i32 in another_numbers_list is {}", result);
    //we abstracted code to make it reusable
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("[with fn largest_char] The largest char is {}", result);
    
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("[with fn largest] the largest number is {}", result);
    
    let char_list = vec![ 'y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("[with fn largest] The largest char is {}", result);
}
