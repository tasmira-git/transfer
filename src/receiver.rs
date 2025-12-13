use crate::transfer_protocol::receive_protocol::ReceiveProtocol;
use std::{
    fs::create_dir_all,
    net::{TcpListener, ToSocketAddrs},
    path::PathBuf,
};

pub fn handle_receive(addr: impl ToSocketAddrs, output_path: &str) {
    let output_path = PathBuf::from(output_path);
    create_dir_all(&output_path).unwrap();
    tracing::debug!("保存文件的目录：{}", output_path.display());

    let stream = TcpListener::bind(addr).unwrap();
    tracing::debug!("服务器启动，监听{}", stream.local_addr().unwrap());

    loop {
        let output_path = output_path.clone();

        let (stream, a) = stream.accept().unwrap();
        tracing::debug!("新连接：{}", a);

        std::thread::spawn(move || {
            let mut stream = ReceiveProtocol::new(stream);
            stream.receive_file_or_dir(&output_path);
            tracing::debug!("接收任务完成");
        });
    }
}
