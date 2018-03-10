#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrEnumTuple {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Ipv4Addr(u8, u8, u8, u8);

#[derive(Debug)]
struct Ipv6Addr(String);

#[derive(Debug)]
enum IpAddrEnumStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddrEnumStruct { // Enums can also use impl
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home is: {:?}", home);
    println!("loopback is: {:?}", loopback);
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("home is: {:?}", home);
    println!("loopback is: {:?}", loopback);
    let home = IpAddrEnumTuple::V4(127, 0, 0, 1);
    let loopback = IpAddrEnumTuple::V6(String::from("::1"));
    println!("home is: {:?}", home);
    println!("loopback is: {:?}", loopback);
    let home = IpAddrEnumStruct::V4(Ipv4Addr(127, 0, 0, 1));
    let loopback = IpAddrEnumStruct::V6(Ipv6Addr(String::from("::1")));
    println!("home is: {:?}", home);
    println!("loopback is: {:?}", loopback);
    home.print();

    // Use Option<T> enum to prevent null.
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    println!("Is some_number none? {}.", some_number.is_none());
    println!("Is some_string none? {}.", some_string.is_none());
    println!("Is absent_number none? {}.", absent_number.is_none());

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // Won't work
}

fn route(ip_type: IpAddrKind) {
    println!("ip type is: {:?}", ip_type);
}
