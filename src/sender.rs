use crate::transfer_protocol::send_protocol::SendProtocol;
use std::{path::PathBuf, time::Instant};
use tokio::net::{TcpStream, ToSocketAddrs};
use walkdir::WalkDir;

pub async fn handle_send(addr: impl ToSocketAddrs, send_path: &str) {
    let start_time = Instant::now();

    let send_path = PathBuf::from(send_path);
    if !send_path.exists() {
        panic!("需要发送的路径{}不存在", send_path.display());
    }

    let stream = TcpStream::connect(addr).await.unwrap();

    let mut stream = SendProtocol::new(stream);

    let root_dir = send_path.parent().unwrap();

    let paths = WalkDir::new(&send_path);

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        stream.send_file_or_dir(path, root_dir).await;
    }
    stream.flush().await;
    tracing::debug!("发送任务完成，耗时: {:?}", start_time.elapsed());
}
