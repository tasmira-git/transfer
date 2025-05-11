use std::{
    fs, net::{TcpStream, ToSocketAddrs}, path::{Path, PathBuf}, time::Instant,
};

use crate::protocol::{SendFilePath, SendProtocol};

pub struct Sender {
    stream: TcpStream,
    send_path: PathBuf,
}

impl Sender {
    pub fn new(addr: impl ToSocketAddrs, send_path: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        println!("连接到服务器：{}", stream.peer_addr().unwrap());

        let send_path = PathBuf::from(send_path);

        Self { stream, send_path }
    } 

    pub fn send(&mut self) {
        let start_time = Instant::now();

        if !self.send_path.exists() {
            println!("文件不存在：{}", self.send_path.display());
            return;
        }

        if self.send_path.is_file() {
            self.send_file(&self.send_path.clone());
        } else {
            let mut paths = Vec::new();
            traverse_dir(&self.send_path, &mut paths);

            for p in paths {
                if p.is_file() {
                    println!("发送文件：{}", p.display());
                    self.send_file(&p);
                } else {
                    println!("发送文件夹：{}", p.display());
                    self.send_dir(&p);
                }
            }
        }

        let duration = start_time.elapsed();
        println!("传输完成，耗时：{:?}",duration);
    }

    fn send_file(&mut self, path: &Path) {
        let mut send_protocol = SendProtocol::new(&mut self.stream);
        send_protocol.send_file_type(true);

        let parent_path = self.send_path.parent().unwrap();
        let send_file_path = SendFilePath::new(parent_path, path);
        send_protocol.send_file_path(send_file_path);

        send_protocol.send_file_size(path);
        send_protocol.send_file_content(path);
    }

    fn send_dir(&mut self, path: &Path) {
        let mut send_protocol = SendProtocol::new(&mut self.stream);
        send_protocol.send_file_type(false);

        let parent_path = self.send_path.parent().unwrap();
        let send_file_path = SendFilePath::new(parent_path, path);
        send_protocol.send_file_path(send_file_path);
    }
}

fn traverse_dir(path: &Path, paths: &mut Vec<PathBuf>) {
        let entrys = fs::read_dir(path).unwrap();
        for entry in entrys {
            let entry = entry.unwrap().path();
            if entry.is_dir() {
                traverse_dir(&entry, paths);
            }
            paths.push(entry);
        } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_dir() {
        let path = Path::new("test");
        let mut paths = Vec::new();
        traverse_dir(path, &mut paths);
        for p in paths {
            println!("{}", p.display());
        }
    }

    #[test]
    fn test() {
        let file_size = 4.6*1024.*1024.*1024.;

        let file_size = file_size as u64;
        let file_size = file_size.to_be_bytes();
        println!("{}", file_size.len());
        let file_size = u64::from_be_bytes(file_size);
        println!("{}", file_size);
    }

}
