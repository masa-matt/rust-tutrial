fn main() {
    bad_example_of_enum();
    good_example_of_enum();
    enum_methods();
    option();
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn bad_example_of_enum() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddrG {
    V4(String),
    V6(String),
}

enum IpAddrDiff {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn good_example_of_enum() {
    let home = IpAddrG::V4(String::from("127.0.0.1"));
    let loopback = IpAddrG::V6(String::from("::1"));

    let home2 = IpAddrDiff::V4(127, 0, 0, 1);
    let loopback2 = IpAddrDiff::V6(String::from("::1"));
}

// standard library
// https://doc.rust-lang.org/std/net/enum.IpAddr.html

struct IpAddrV4 {
    // -- snip --
}

struct IpAddrV6 {
    // -- snip --
}

enum IpAddrStd {
    V4(IpAddrV4),
    V6(IpAddrV6),
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

fn enum_methods() {
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // error[E0277]: cannot add `Option<i8>` to `i8`
    // let sum = x + y;
}
