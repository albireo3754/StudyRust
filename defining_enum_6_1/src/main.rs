enum IpAdddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


fn main() {
    let home = IpAdddr::V4(String::from("127."));
    // let message: Message = Message::Move { ..MoveMessage };

    let y: Option<i8> = Option::Some(3);

    println!("Hello, world!{:?}", y);
}
