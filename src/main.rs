pub mod send;
pub mod receive;
pub mod display;
pub mod protocol;

use receive::Receiver;
use send::Sender;

use std::env::args;

const PORT: u16 = 8000;
const BUFFER_SIZE: usize = 4*1024;

fn main() {
    let args = args().collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>(); 

    let mode = args[1];

    match mode {
        "send" => {
            if args.len() < 4 {
                println!("用法：{} {} ip 文件", args[0], mode);
                return;
            }
            let addr = format!("{}:{}", args[2], PORT); 
            Sender::new(addr, args[3]).send();
        }
        "receive" => {
            if args.len() < 2 {
                println!("用法：{} {}", args[0], mode);
                return;
            }
            let output_dir = args.get(2).map(|arg| *arg);
            Receiver::new(output_dir).receive();
        }
        _ => println!("无效模式：{}，请使用 send 或 receive", mode),
    };
}
