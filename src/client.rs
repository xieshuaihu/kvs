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
        let mut set = Set::new();
        set.set_key(key);
        set.set_value(value);

        let mut command = Command::new();
        command.set_field_type(Type::set);
        command.set_action(set.write_to_bytes()?);

        let bytes = command.write_to_bytes()?;
        self.stream.write(&bytes)?;

        Ok(())
    }

    pub fn exec_remove(&mut self, key: String) -> Result<(), Box<dyn Error>> {
        let mut remove = Remove::new();
        remove.set_key(key);

        let mut command = Command::new();
        command.set_field_type(Type::remove);
        command.set_action(remove.write_to_bytes()?);

        let bytes = command.write_to_bytes()?;
        self.stream.write(&bytes)?;

        Ok(())
    }

    pub fn exec_get(&mut self, key: String) -> Result<(), Box<dyn Error>> {
        let mut get = Get::new();
        get.set_key(key);

        let mut command = Command::new();
        command.set_field_type(Type::get);
        command.set_action(get.write_to_bytes()?);

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
        println!("Waiting for new connection......");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("A new connection from: \"{}\"", stream.peer_addr()?);
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
        let command = Command::parse_from_bytes(&buf[..size])?;

        match command.get_field_type() {
            Type::get => {
                let get = Get::parse_from_bytes(command.get_action())?;
                println!("Get: {}", get.get_key());
            },
            Type::remove => {
                let remove = Remove::parse_from_bytes(command.get_action())?;
                println!("Remove: {}", remove.get_key());
            },
            Type::set => {
                let set = Set::parse_from_bytes(command.get_action())?;
                println!("Set: {} {}", set.get_key(), set.get_value());
            },
            Type::response => unreachable!(),
        }

        Ok(())
    }
}