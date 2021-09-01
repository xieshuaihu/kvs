use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::error::Error;
use protobuf::Message;

use crate::command::{Command, Set, Get, Remove, Type};

pub struct KvsClient {
    stream: TcpStream,
}

impl KvsClient {
    pub fn new(socket_addr: &SocketAddr) -> Result<KvsClient, Box<dyn Error>>{
        Ok(KvsClient{stream: TcpStream::connect(socket_addr)?})
    }

    pub fn exec_set(&mut self, key: String, value: String) -> Result<(), Box<dyn Error>>{
        let mut command = Command::new();
        command.set_field_type(Type::set);
        command.set_action(vec![1, 2, 3, 4, 54, 6]);
        let bytes = command.write_to_bytes()?;

        self.stream.write(&bytes)?;
        Ok(())
    }
}

pub struct KvsServer {
    listener: TcpListener,
}

impl KvsServer {
    pub fn new(socket_addr: &SocketAddr) -> Result<KvsServer, Box<dyn Error>>{
        Ok(KvsServer {listener: TcpListener::bind(socket_addr)?})
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream)?;
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 2048];
        let size = stream.read(&mut buf)?;
        let mut command = Command::new();
        command.merge_from_bytes(&buf[..size])?;
        println!("{:?}", command.get_field_type());
        Ok(())
    }
}