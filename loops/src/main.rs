fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
        //the following line will only be printed once the loop finishes.
        //it's a silly loop, but it illustrates that we can return a value after the break
        println!("The result is {result}");

    //disambiguate loops with 'labels:
    //for loops within loops the break and continue keywords apply to the innermost loop at that
    //point.
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {count}");

    //while loop
    let mut number = 7;

    while number != 0 {
        println!("Number in while loop {number}!");
        number -= 1;
    }
    println!("We've finished! time to fly!");

   let a = [10, 20, 30, 40, 50];
   let mut index = 0;
   while index < 5 {
       println!("the value is: {}", a[index]);
       index += 1;
   }
   

   //for in -- adds increased safety - doesn't rely on hardcoded values
   //safety and conciseness
   let ab = [10, 20, 30, 40, 50];
   for element in ab {
       println!("the for...in value is {element}");
   }

   //Now with Range
   for n in (1..4).rev() {
       println!("range {n} reversing the range!");
   }
   println!("Yey");
}
