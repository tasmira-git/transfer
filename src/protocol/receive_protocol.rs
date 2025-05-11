use std::{fs::{create_dir_all, File}, io::{BufReader, BufWriter, Read, Write}, net::TcpStream, path::Path};

use crate::BUFFER_SIZE;

pub struct ReceiveProtocol<'a> {
    stream: BufReader<&'a mut TcpStream>,
}

impl<'a> ReceiveProtocol<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        let stream = BufReader::new(stream);

        Self { stream }
    }

    pub fn receive_file_type(&mut self) -> Result<bool, std::io::Error> {
        let mut buf = [255];
        self.stream.read_exact(&mut buf)?;

        Ok(buf[0] == 0)
    }
    
    pub fn receive_file_path(&mut self) -> String {
        let mut name_length_buf = [0; 2];
        self.stream.read_exact(&mut name_length_buf).unwrap();
        let name_length = u16::from_be_bytes(name_length_buf);

        let mut name_buf = vec![0; name_length as usize];
        self.stream.read_exact(&mut name_buf).unwrap(); 
        let file_name = String::from_utf8(name_buf).unwrap();
        file_name
    }

    pub fn receive_file_size(&mut self) -> u64 {
        let mut size_buf = [0; 8];
        self.stream.read_exact(&mut size_buf).unwrap();
        let size = u64::from_be_bytes(size_buf);

        size
    }

    pub fn receive_file_content(&mut self, path: &Path, file_size: u64) {
        create_dir_all(path.parent().unwrap()).unwrap();

        let file = File::create(path).unwrap();
        let mut file = BufWriter::new(file);

        let mut buf = [0; BUFFER_SIZE];
        let mut total_read = 0;

        while total_read < file_size {
            let bytes_to_read = std::cmp::min(buf.len() as u64, file_size - total_read) as usize;

            let n = self.stream.read(&mut buf[..bytes_to_read]).unwrap();

            if n==0 {
                break;
            }

            file.write_all(&buf[..n]).unwrap();
            total_read += n as u64;
            // println!("total_read: {}, file_size: {}", total_read, file_size);
        }
    }
}