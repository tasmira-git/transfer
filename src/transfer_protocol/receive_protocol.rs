use std::path::{Path, PathBuf};

use tokio::{
    fs::create_dir_all,
    io::{AsyncReadExt, BufReader},
    net::TcpStream,
};

use crate::{display::size_display, transfer_protocol::TYPE_FILE};

pub struct ReceiveProtocol {
    reader: BufReader<TcpStream>,
}
impl ReceiveProtocol {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream),
        }
    }

    pub async fn receive_file_or_dir(&mut self, save_path: &Path) {
        loop {
            let Some(is_file) = self.receive_file_type().await else {
                break;
            };

            if is_file {
                tracing::debug!("接收文件：");
            } else {
                tracing::debug!("接收目录：");
            }

            let receive_path = self.receive_file_path().await;
            tracing::debug!("   {}", receive_path.display());

            if is_file {
                let save_path = save_path.join(receive_path);
                self.receive_file(&save_path).await;
            } else {
                create_dir_all(save_path.join(receive_path)).await.unwrap();
            }
        }
    }

    async fn receive_file_type(&mut self) -> Option<bool> {
        let mut type_buf = [0];

        match self.reader.read(&mut type_buf).await {
            Ok(0) => {
                tracing::debug!("接收任务完成");
                None
            }
            Ok(1) => Some(type_buf[0] == TYPE_FILE),
            Ok(n) => {
                panic!("接收文件类型失败：读取了{}字节，预期1字节", n);
            }
            Err(e) => {
                panic!("接收文件类型失败: {}", e);
            }
        }
    }

    async fn receive_file_path(&mut self) -> PathBuf {
        let mut len_buf = [0; 2];
        self.reader.read_exact(&mut len_buf).await.unwrap();
        let len = u16::from_be_bytes(len_buf);

        let mut path_buf = vec![0; len as usize];
        self.reader.read_exact(&mut path_buf).await.unwrap();
        let path = String::from_utf8_lossy(&path_buf).into_owned();

        PathBuf::from(path)
    }

    async fn receive_file(&mut self, save_path: &Path) {
        let mut size_buf = [0; 8];
        self.reader.read_exact(&mut size_buf).await.unwrap();

        let size = u64::from_be_bytes(size_buf);

        create_dir_all(save_path.parent().unwrap()).await.unwrap();
        let mut file = tokio::fs::File::create(save_path).await.unwrap();

        let mut limited_reader = (&mut self.reader).take(size);

        tracing::debug!("   接收文件中，文件大小：{}", size_display(size));
        tokio::io::copy(&mut limited_reader, &mut file)
            .await
            .unwrap();
    }
}
