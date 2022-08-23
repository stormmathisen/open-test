use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt};

use anyhow::{Result, Context};


fn main() -> Result<()> {
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);

    println!("Opening file");

    let mut dev_file = file_options.open("/dev/pcie_bar_s3")
        .context("Failed to open dev_file")?;
    
    let mut packet: [u8; 255] = [12; 255];

    println!("Reading file");

    dev_file.read_exact(&mut packet)
        .context("Failed to read into package")?;
    println!("{:?}", packet);
    
    Ok(())
}
