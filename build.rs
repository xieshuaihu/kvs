use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use protoc_rust::Customize;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = &env::var("OUT_DIR")?;
    let out_dir = Path::new(out_dir);
    let mut buf = PathBuf::new();
    buf.push(out_dir);
    buf.push("command");
    let out_dir = buf.as_path();

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    protoc_rust::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(out_dir)
        .input("src/resources/command.proto")
        .run()?;

    Ok(())
}
