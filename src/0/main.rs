/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 */
fn main() {
//    println!("{:?}", IpAddrKind); // error[E0423]: expected value, found enum `IpAddrKind`
//    println!("{}", IpAddrKind::V4); // error[E0277]: `IpAddrKind` doesn't implement `std::fmt::Display`
    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
