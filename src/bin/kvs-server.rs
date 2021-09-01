use std::{error::Error, net::SocketAddr, str::FromStr};

use kvs::KvsServer;

fn main() -> Result<(), Box<dyn Error>>{
    let socket = SocketAddr::from_str("127.0.0.1:4000")?;
    let server = KvsServer::new(&socket)?;
    server.run()?;

    Ok(())
}
