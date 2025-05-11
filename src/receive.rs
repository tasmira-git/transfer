use std::{
    fs::create_dir_all,
    net::{TcpListener, TcpStream}, path::PathBuf
};

use crate::{display::size_display, protocol::ReceiveProtocol, PORT};

pub struct Receiver {
    stream: TcpStream,
    output_dir: PathBuf,
}

impl Receiver {
    pub fn new(output_dir: Option<&str>) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).unwrap();

        println!("服务器启动，监听{}", listener.local_addr().unwrap());
        println!("文件接收目录：{:?}", output_dir);

        let (stream, addr) = listener.accept().unwrap();
        println!("新连接：{}", addr);

        let output_dir = output_dir.map(|s| PathBuf::from(s)).unwrap_or_default();
        println!("文件接收目录：{}", output_dir.display());

        Self { stream, output_dir }
    }

    pub fn receive(&mut self) {
        create_dir_all(&self.output_dir).unwrap();

        let mut receive_protocol = ReceiveProtocol::new(&mut self.stream);

        loop {
            let is_file = match receive_protocol.receive_file_type() {
                Ok(b) => b,
                Err(_) => {
                    println!("文件接收完成！");
                    return;
                }
            };
            
            let file_name = receive_protocol.receive_file_path();
            println!("接收的文件名：{}", file_name);

            let path = self.output_dir.join(file_name);
            if is_file {
                let file_size = receive_protocol.receive_file_size();

                println!("接收的文件大小：{}", size_display(file_size));
                receive_protocol.receive_file_content(&path, file_size);
            } else {
                create_dir_all(path).unwrap();                 
            }
        }
    }
}
