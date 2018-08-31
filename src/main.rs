enum IpAddrKind {
    v4(String),
    v6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: "127.0.0.1",
    // };

    let home = IpAddrKind::v4(String::from("127.0.0.1"));
    // println!("The Origin is {:?}", home);
}
