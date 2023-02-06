fn main() {
    let r; //it's ok to declare a variable without initializing, so it leaves in this outer scope                                             ---+---- 'a
    // println!("r: {}", r); // but this will throw an error at compile time, because we're trying to use something that hasn't been defined     
    // ^ `r` used here but it is possibly-uninitialized
    
    {
        let x = 5; //the scope of x is limited to this block
        r = &x;
    } // `x` dropped here while still borrowed
    
    // println!("r: {}", r); //   ^^ borrowed value does not live long enough
    // 
    
    let r = 2;
    println!("r: {}", r) //r: 2
    
    
}
