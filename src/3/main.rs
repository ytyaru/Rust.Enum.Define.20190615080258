/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 */
fn main() {
    // enum型だと種類を識別できる(structだと不可?)
    println!("{}", Message::Move{x:0,y:0} == Message::Move{x:0,y:0});
    println!("{}", Message::Move{x:0,y:0} == Message::Move{x:1,y:1});
    println!("{}", Message::Move{x:0,y:0} == Message::Quit);
    use std::mem;
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Move{x:0,y:0}));
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Move{x:1,y:1}));
    println!("{}", mem::discriminant(&Message::Move{x:0,y:0}) == mem::discriminant(&Message::Quit));
}
// メッセージをenumで定義する
#[derive(PartialOrd, PartialEq, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// メッセージをstructで定義する
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct Write(String); // タプル構造体
struct ChangeColor(i32, i32, i32); // タプル構造体

