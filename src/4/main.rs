/*
 * RustのEnum（定義）。
 * CreatedAt: 2019-06-15
 */
fn main() {
    let m = Message::Move{x:0,y:0};
    m.call();
    Message::Move{x:1,y:1}.call();
    Message::Quit.call();
    Message::Write(String::from("hello")).call();
    Message::ChangeColor(255,0,0).call();
}
// メッセージをenumで定義する
#[derive(PartialOrd, PartialEq, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enumにメソッドを実装する
impl Message {
    fn call(&self) {
        println!("call {:?}", self);
    }
}

