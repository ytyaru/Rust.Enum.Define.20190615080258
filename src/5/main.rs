/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 * https://doc.rust-jp.rs/book/second-edition/ch06-01-defining-an-enum.html#option-enum%E3%81%A8null%E5%80%A4%E3%81%AB%E5%8B%9D%E3%82%8B%E5%88%A9%E7%82%B9
 * https://doc.rust-lang.org/stable/std/option/enum.Option.html
 * enum Option<T> { Some(T), None, }
 */
fn main() {
    let some1 = Some(5);
    let some2 = Some("hello");
    let opt: Option<i32> = None;
    println!("{:?}", some1);
    println!("{:?}", some2);
    println!("{:?}", opt);
    let x: Option<i8> = Some(5);
    println!("{:?}", x);
//    println!("{:?}", 3+x); // error[E0277]: cannot add `std::option::Option<i8>` to `{integer}`
}

