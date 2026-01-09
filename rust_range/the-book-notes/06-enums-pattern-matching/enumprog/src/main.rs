#![allow(dead_code)]
#![allow(unused)]

// Basic IP address enum
// Doesn't associate data & classification like a variant
enum IpAddrKindNaive {
    V4,
    V6,
}

// You might think of using it like so
// This is bad though
struct IpAddrNaive {
    kind: IpAddrKindNaive,
    address: String,
}

// Could (for example) use it in a function like so
fn route(ip_kind: IpAddrKindNaive) {
    () // Do something
}

// A better way to do it would be this
// Embeds data directly in each enum variant
enum IpAddr {
    V4(String),
    V6(String),
}

// The standard library has a type for it though :)
// use std::net::IpAddr;

fn main() {
    let four = IpAddrKindNaive::V4;
    route(four);
}
