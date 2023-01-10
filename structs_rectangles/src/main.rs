//we have to explicitly opt in to make the Debug functionality available to the struct
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle WITH TUPLE is {} square pixels", area(width1, height1));
    
    //REFACTORING WITH TUPLES
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels", area_with_tuple(rect1));
    
    //REFACTORING WITH STRUCTS
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    //by passing a reference `&rect2`, main function maintain ownership and can continue to use rect2
    println!("The area of the rectangle WITH STRUCT is {} square pixels", area_with_structs(&rect2));
    
    println!("rect is {:?}", &rect2);
    println!("pretty rect is {:#?}", &rect2);
   //the dbg! macro, which takes ownership of an expression 
   //(as opposed to println! that takes a reference)
   //dbg! prints to the stderr
   //println! prints to the stdout (chapter 12)
   
   // we pass a reference to rect2, because we don't want the dbg! macro to take ownership
   //of the value
   dbg!(&rect2); //prints the file name, line number, value
}



fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//the type of the parameter is an immutable borrow of a struct Rectangle instance
// a win for clarity - the fields are related in a semantic way
fn area_with_structs(rectangle: &Rectangle) -> u32 {
    //"accessing fields of a borrowed struct instance does not move the field values"
    rectangle.width * rectangle.height
}

