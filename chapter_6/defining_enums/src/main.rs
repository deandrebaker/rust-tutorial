enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;
    route(ip_v4);
    route(ip_v6);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_2 = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}
