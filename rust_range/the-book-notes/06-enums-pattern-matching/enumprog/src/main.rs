#![allow(dead_code)]
#![allow(unused)]

// Basic IP address enum
// Doesn't associate data & classification like a variant
enum IpAddrKind {
    V4,
    V6,
}

// You might think of using it like so
// This is bad though
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// Could (for example) use it in a function like so
fn route(ip: IpAddr1) {
    // Do something
}

// A better way to do it would be this
// Embeds data directly in each enum variant
    // So variants are directly related to the data
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Enums are better than structs here as well, as...
// You can have different data (amounts, types) associated with each variant
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// The standard library has a type for it though :)
// use std::net::IpAddr;

fn main() {
    let four = IpAddrKind::V4;
    let four = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // Example use
    route(four);

    // Each enum variant's name becomes a function
    // to construct an instance of that enum variant
    let six = IpAddr2::V6(
        String::from("::1")
    );

}
