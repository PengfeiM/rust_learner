use self::enums::IpAddrVersion;

mod enums;

#[derive(Debug)]
struct IpAddr {
    version: IpAddrVersion,
    address: String,
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
}
