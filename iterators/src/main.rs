fn main() {
    let v1 = vec![1,2,3];
    
    //iterators are lazy. here we're only storing it to variable
    let v1_iter = v1.iter();
    
    println!("vi_iter: {:?}",  v1_iter); //vi_iter: Iter([1, 2, 3])
    
    for value in v1_iter {
        println!("Got: {}", value);
    }
    
}
