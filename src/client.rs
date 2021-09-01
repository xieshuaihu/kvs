use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::error::Error;
use protobuf::Message;

use crate::command::{Command, Set, Get, Remove, Type};

pub struct KvsClient {
    stream: TcpStream,
}

impl KvsClient {
    fn new(socket_addr: &SocketAddr) -> Result<KvsClient, Box<dyn Error>>{
        Ok(KvsClient{stream: TcpStream::connect(socket_addr)?})
    }

    fn exec_set(&self, key: String, value: String) {
        let mut command = Command::new();
        command.set_field_type(Type::set);
        command.set_action(vec![1, 2, 3, 4, 54, 6]);
        self.stream.write(command.write_to_bytes()?);
    }
}

pub struct KvsServer {
    listener: TcpListener,
}

impl KvsServer {
    fn new(socket_addr: &SocketAddr) -> Result<KvsServer, Box<dyn Error>>{
        Ok(KvsServer {listener: TcpListener::bind(socket_addr)?})
    }

    fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handleConnection(stream);
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    }

    fn handleConnection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 2048];
        let size = stream.read(&mut buf)?;
        let mut command = Command::new();
        command.merge_from_bytes(&buf[..size]);
        println!("{:?}", command.get_field_type());
        Ok(())
    }
}