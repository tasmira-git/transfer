use crate::transfer_protocol::send_protocol::SendProtocol;
use std::{
    net::{TcpStream, ToSocketAddrs},
    path::PathBuf,
    time::Instant,
};
use walkdir::WalkDir;

pub fn handle_send(addr: impl ToSocketAddrs, send_path: &str) {
    let send_path = PathBuf::from(send_path);
    if !send_path.exists() {
        panic!("需要发送的路径{}不存在", send_path.display());
    }

    let stream = TcpStream::connect(addr).unwrap();
    tracing::info!("连接到服务器成功");

    let mut stream = SendProtocol::new(stream);

    let root_dir = send_path.parent().unwrap();

    let paths = WalkDir::new(&send_path);

    tracing::info!("发送文件中...");
    let start_time = Instant::now();
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        stream.send_file_or_dir(path, root_dir);
    }

    stream.flush();
    tracing::info!("发送任务完成，耗时: {:?}", start_time.elapsed());
}
