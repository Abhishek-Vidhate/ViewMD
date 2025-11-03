// take file path input and read content of file in terminal
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("test.md")?;
    println!("content : {content}");
    Ok(())
}
