use std::{fs::File, io::{BufReader, BufWriter, Read, Write}, net::TcpStream, path::Path};

use crate::BUFFER_SIZE;

pub struct SendFilePath<'a>(&'a Path);
impl<'a> SendFilePath<'a> {
    pub fn new(parent_path: &'a Path, full_path: &'a Path) -> Self {
        let path = full_path.strip_prefix(parent_path).unwrap();

        Self(path)
    }
}

pub struct SendProtocol<'a> {
    stream: BufWriter<&'a mut TcpStream>,
}

impl<'a> SendProtocol<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        let stream = BufWriter::new(stream);

        Self { stream }
    } 

    pub fn send_file_type(&mut self, is_file: bool) {
        if is_file {
            self.stream.write_all(&[0]).unwrap();
        } else {
            self.stream.write_all(&[1]).unwrap();
        }
    }

    pub fn send_file_path(&mut self, path: SendFilePath) {
        let path_name = path.0.to_str().unwrap().as_bytes();
        let path_name_length = (path_name.len() as u16).to_be_bytes();

        self.stream.write_all(&path_name_length).unwrap();
        self.stream.write_all(path_name).unwrap();
    }

    pub fn send_file_size(&mut self, path: &Path) {
        let metadata = path.metadata().unwrap();
        let file_size = metadata.len().to_be_bytes();

        self.stream.write_all(&file_size).unwrap();
    }

    pub fn send_file_content(&mut self, path: &Path) {
        let file = File::open(path).unwrap();
        let mut file = BufReader::new(file);

        let mut buf = [0; BUFFER_SIZE];
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            
            self.stream.write_all(&buf[..n]).unwrap();
        }
    }
}

