//STRUCTS
fn main() {
    let mut user1 = User {
        email: String::from("yoda@example.com"),
        username: String::from("yoda123"),
        active: true, //
        sign_in_count: 1,
    };
    
    user1.email = String::from("master.yoda@example.com");
    
    //Create instance from another instance without the update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("happy.yoda@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    
    //a better way...
    //the fields not explicitly set wil be inherited from the other instance
    //the update syntax should come last
    let user2 = User {
        email: String::from("power.yoda@example.com"),
        ..user1
    };
    
    //at this point, user1 is no longer available in memory,as we moved its username to user2
    println!("{}", user2.email);
    

    //TUPLE STRUCTS
    //the struct becomes the type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
    
    
    //Unit-like Structs
    let _subject = AlwaysEqual;
    //note: we prefix unused variables with an underscore to silence the warning from the compiler
    
    let user3 = build_user(String::from("luke@example.com"), String::from("luke123"));
    println!("user3 email: {}", user3.email);
    println!("user3 username: {}", user3.username);
    println!("user3 active: {}", user3.active);
    println!("user3 sign_in_count: {}", user3.sign_in_count);
    
    
}

struct User {
    active: bool, //implement Copy Trait
    username: String, // each instance will ow its data for as long as the struct is valid*
    email: String, // each instance will ow its data for as long as the struct is valid*
    sign_in_count: u64, //implement Copy trait
}

//like in TS, we can use field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


//Unit-Like Structs without any fields
//behave like unit, i.e., represent an empty value
//"The tuple without any values has a special name, unit. 
//This value and its corresponding type are both written () and represent an empty value 
//or an empty return type. 
//Expressions implicitly return the unit value if they don’t return any other value."
//use case: 
//implement a trait on some type but don't have any data that you want to store in the type itself
//it's like a placeholder for future behavior implementation - see Chapter 10
struct AlwaysEqual;