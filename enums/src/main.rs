//(*) instead of a enum inside a struct
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

//let home = IpAddr {
//   kind: IpAddrKind::V4,
//   address: String::from("127.0.0.1")
//}
//let loopback = IpAddr {
//   kind: IpAddrKind::V6,
//   address: String::from("::1")
//}


//enum is a custom data type
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),  
    V6(String), //we associate (String or other) values to each of the variant (*)
}


#[derive(Debug)]
enum Message {
    _Quit, //no data associated with this variant
    _Move { x: i32, y: i32}, //named fields (x) and (y) like a struct
    Write(String), //a String
    _ChangeColor(i32, i32, i32), //three i32 values
}

//from the standard library. no need to import it
// enum Option<T> {
//     Some(T),
//     None,
// }

impl Message {
    fn call(&self) {
        println!("{:#?}", self)
    }
}

fn main() {
    
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6; //the variants :: are namespaced :: under its identifier
    
    
    //each enum variant that we define also becomes 
    //a function that constructs an instance of the enum
    
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1")); 
    
    
    // route(IpAddrKind::V6);
    let home = IpAddrKind::V4(127, 0, 0, 1);
    route(home);
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    //Option Enum and its variants
    
    let _some_number = Some(5); //Option<i32>
    let _some_char = Some('a'); //Option<char> 
    let _absent_number: Option<i32> = None; //Option<i32> we annotate because Rust can't infer the type in the absence of a value
   
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; //error -> no implementation for `i8 + std::option::Option<i8>`
    
}


fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind)
}