/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 */
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
fn main() {
    println!("{}", IpAddr::V4(Ipv4Addr::new(127,0,0,1)));
    println!("{}", IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)));
}

