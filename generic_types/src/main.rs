fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list {
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
    
    let result = largest(&number_list);
    println!("[with function] the largest number is {}", result);
    
    let another_numbers_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&another_numbers_list);
    println!("[with function] the largest number in another_numbers_list is {}", result);
    //we abstracted code to make it reusable
    
    
    
    
}
