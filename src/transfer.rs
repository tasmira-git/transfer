
// pub struct Transfer {
//     sender: Sender,
//     receiver: Receiver,
// }

// impl Transfer {
//     pub fn new() -> Self {
//         Transfer {
//             sender: Sender::new(),
//             receiver: Receiver::new(),
//         }
//     } 

//     pub fn start_by_mode(&self, mode: &str) -> Result<(), String> {
//         match mode {
//             "send" => {
//                 if args.len() < 4 {
//                     println!("用法：{} {} ip 文件", args[0], mode);
//                     return format!("");
//                 }
//                 let addr = format!("{}:{}", args[2], PORT); 
//                 send(addr, args[3]);
//             }
//             "receive" => {
//                 if args.len() < 2 {
//                     println!("用法：{} {}", args[0], mode);
//                     return;
//                 }
//                 let output_dir = args.get(2).unwrap_or(&"");
//                 receive(output_dir);
//             }
//             _ => println!("无效模式：{}，请使用 send 或 receive", mode),
//         };
//     }
// }