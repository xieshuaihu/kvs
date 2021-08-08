use clap::{App, Arg};
use std::env;
use std::net::IpAddr;

fn main() -> kvs::Result<()> {
    // let dir = env::current_dir()?;
    // let mut kvs = kvs::KvStore::open(&dir)?;
    // IpAddr::

    let args = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("a kv store for learning")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("get")
                .about("get a value by key")
                .arg(Arg::new("key").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("remove a value by key")
                .arg(Arg::new("key").required(true)),
        )
        .subcommand(
            App::new("set")
                .about("set a value by key")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true)),
        )
        .arg(
            Arg::new("addr")
            .about("server host and port")
                .long("addr")
                .short('a')
                .required(false)
                .default_value("127.0.0.1:4000"),
        )
        .get_matches();

    if let Some(addr) = args.value_of("addr") {
        match args.subcommand() {
            Some(("get", args)) => {
                if let Some(key) = args.value_of("key") {
                    println!("{}, {}", key, addr);
                    return Ok(());
                    // command = Some(kvs::Command::Get(key.to_owned()));
                }
            }
            Some(("rm", args)) => {
                if let Some(key) = args.value_of("key") {
                    println!("{}, {}", key, addr);
                    return Ok(());
                    // command = Some(kvs::Command::Rm(key.to_owned()));
                }
            }
            Some(("set", args)) => {
                if let Some(key) = args.value_of("key") {
                    if let Some(value) = args.value_of("value") {
                        println!("{}, {}, {}", key, value, addr);
                        return Ok(());
                        // command = Some(kvs::Command::Set((key.to_owned(), value.to_owned())));
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    Ok(())
}
