/// IpAddrVersion
/// a enum allows us to define a set of possible values which are similar.
#[derive(Debug)]
pub enum IpAddrVersion {
    V4,
    V6,
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    pub fn print(&self) {
        println!("here is a message: {:?}", self);
    }
}
