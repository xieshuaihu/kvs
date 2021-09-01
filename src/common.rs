use std::net::IpAddr;
use std::net::AddrParseError;
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::str::FromStr;

fn parse_ip(ip: &str) -> Result<IpAddr, AddrParseError> {
    IpAddr::from_str(ip)
}

fn parse_port(port: &str) -> Result<u16, ParseIntError> {
    u16::from_str(port)
}

fn parse_socket(socket: &str) -> Result<SocketAddr, AddrParseError>{
    SocketAddr::from_str(socket)
}
