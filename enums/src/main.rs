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


fn main() {
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6; //the variants :: are namespaced :: under its identifier
    // println!("{:#?}", four);
    // println!("{:?}", six);
    
    
    //each enum variant that we define also becomes 
    //a function that constructs an instance of the enum
    
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1")); 
    
    
    // route(IpAddrKind::V6);
    let home = IpAddrKind::V4(127, 0, 0, 1);
}


fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind)
}