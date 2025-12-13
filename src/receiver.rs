use crate::transfer_protocol::receive_protocol::ReceiveProtocol;
use std::path::PathBuf;
use tokio::{
    fs::create_dir_all,
    net::{TcpListener, ToSocketAddrs},
};

pub async fn handle_receive(addr: impl ToSocketAddrs, output_path: &str) {
    let output_path = PathBuf::from(output_path);
    create_dir_all(&output_path).await.unwrap();
    tracing::debug!("保存文件的目录：{}", output_path.display());

    let stream = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("服务器启动，监听{}", stream.local_addr().unwrap());

    loop {
        let output_path = output_path.clone();

        let (stream, a) = stream.accept().await.unwrap();
        tracing::debug!("新连接：{}", a);

        tokio::spawn(async move {
            let mut stream = ReceiveProtocol::new(stream);
            stream.receive_file_or_dir(&output_path).await;
            tracing::debug!("接收任务完成");
        });
    }
}
