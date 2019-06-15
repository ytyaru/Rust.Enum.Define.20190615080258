/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 */
fn main() {
//    println!("{:?}", IpAddrKind); // error[E0423]: expected value, found enum `IpAddrKind`
//    println!("{}", IpAddrKind::V4); // error[E0277]: `IpAddrKind` doesn't implement `std::fmt::Display`
//    println!("{:?}", IpAddrKind::V4);
//    println!("{:?}", IpAddrKind::V6);
//    println!("{}", IpAddr::V4(127, 0, 0, 1)); // error[E0277]: `IpAddr` doesn't implement `std::fmt::Display`
//    println!("{}", IpAddr::V6(String::from("::1"))); // error[E0277]: `IpAddr` doesn't implement `std::fmt::Display`
    println!("{:?}", IpAddr::V4(127, 0, 0, 1)); // error[E0277]: `IpAddr` doesn't implement `std::fmt::Display`
    println!("{:?}", IpAddr::V6(String::from("::1"))); // error[E0277]: `IpAddr` doesn't implement `std::fmt::Display`
}
#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}
