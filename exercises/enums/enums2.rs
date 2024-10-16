// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.
#[derive(Debug)]  
enum Message {  
    // 表示移动的消息，带有x和y坐标  
    Move { x: i32, y: i32 },  
    // 表示回显的消息，带有字符串数据  
    Echo(String),  
    // 表示更改颜色的消息，带有RGB值  
    ChangeColor(i32, i32, i32),  
    // 表示退出的消息，不带有任何数据  
    Quit,  
}  
  
impl Message {  
    fn call(&self) {  
        println!("{:?}", self);  
    }  
}  
  
fn main() {  
    let messages = [  
        Message::Move { x: 10, y: 30 },  
        Message::Echo(String::from("hello world")),  
        Message::ChangeColor(200, 255, 255),  
        Message::Quit,  
    ];  
  
    for message in &messages {  
        message.call();  
    }  
}
