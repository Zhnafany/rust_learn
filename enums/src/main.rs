enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,                              //Quit has no data associated with it at all.
    Move { x: i32, y: i32 },           //Move includes an anonymous struct inside it.
    Write(String),                     //Write includes a single String.
    ChangeColor(i32, i32, i32),        //ChangeColor includes three i32 values.
}

/////////////////////////////
//std::option, T some type, enum that can have value or not // like null at C but protected
enum Option<T> {
    Some(T),
    None,
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    ///////////////////////////////////
    //enum at struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //////////////////////
    //enum with data
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    /////////////////////////
    //enum v4 with 4 bates
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}

