use self::enums::{IpAddrVersion, Message};
use self::option_enum::some_example;

mod enums;
mod option_enum;

#[derive(Debug)]
struct IpAddr {
    version: IpAddrVersion,
    address: String,
}

// Rather than use enum in a struct, we can put data directly into each enum varaint.
#[derive(Debug)]
enum IpAddressString {
    V4(String),
    V6(String),
}

// also, we can define it with numbers.
#[derive(Debug)]
enum IpAddressNum {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}

impl IpAddr {
    fn new(version: IpAddrVersion, address: String) -> Self {
        Self { version, address }
    }
}

fn main() {
    let four = IpAddrVersion::V4;
    // let six = IpAddrVersion::V6;

    // here, we demonstrate a useful feature of enum, we us it to define a type with numberic
    // values, like ip address version: 4 or 6.
    let home = IpAddr::new(four, String::from("127.0.0.1"));
    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1"),
    };
    println!(
        "Voila, we define deux ip address with enum: {:?} {:?}",
        home, loopback
    );

    // IpAddressString
    let home = IpAddressString::V4(String::from("127.0.0.1"));
    let loopback = IpAddressString::V6(String::from("::1"));
    println!(
        "we define two ip address with enum: {:?} {:?}",
        home, loopback
    );

    // IpAddressNum
    let ip43 = IpAddressNum::V4(19, 58, 10, 09);
    let ip63 = IpAddressNum::V6(0xfe80, 0, 0, 0, 0, 0, 0, 0x1);
    println!(
        "we define two ip address with enum containing number: {:?} {:?}",
        ip43, ip63,
    );

    // try some fancy enum.
    let m = Message::Write(String::from("hello"));
    m.print();

    // The Option Enum.
    println!("==== Option Enum ====");
    some_example();
}
