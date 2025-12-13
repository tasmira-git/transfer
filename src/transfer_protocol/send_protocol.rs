use crate::{
    display::size_display,
    transfer_protocol::{TYPE_DIR, TYPE_FILE},
};
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    path::Path,
};

pub struct SendProtocol {
    writer: BufWriter<TcpStream>,
}

impl SendProtocol {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            writer: BufWriter::new(stream),
        }
    }
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    pub fn send_file_or_dir(&mut self, path: &Path, root_dir: &Path) {
        self.send_file_type(path);

        let relative_path = path.strip_prefix(root_dir).unwrap();
        self.send_path_name(&relative_path);

        if path.is_file() {
            self.send_file(path);
        }
    }

    fn send_file_type(&mut self, path: &Path) {
        if path.is_file() {
            tracing::debug!("发送文件：");
            self.writer.write_all(&[TYPE_FILE]).unwrap();
        } else {
            tracing::debug!("发送目录：");
            self.writer.write_all(&[TYPE_DIR]).unwrap();
        }
    }

    fn send_path_name(&mut self, path_name: &Path) {
        tracing::debug!("   {}", path_name.display());

        let path_name = path_name.to_str().unwrap().as_bytes();
        let path_name_len: [u8; 2] = (path_name.len() as u16).to_be_bytes();

        self.writer.write_all(&path_name_len).unwrap();
        self.writer.write_all(path_name).unwrap();
    }

    fn send_file(&mut self, file_path: &Path) {
        let metadata = file_path.metadata().unwrap();
        let file_size: [u8; 8] = metadata.len().to_be_bytes();

        tracing::debug!("   发送文件中，文件大小：{}", size_display(metadata.len()));

        self.writer.write_all(&file_size).unwrap();

        let mut file = std::fs::File::open(file_path).unwrap();
        std::io::copy(&mut file, &mut self.writer).unwrap();
    }
}
