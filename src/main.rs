// take file path input and read content of file in terminal
use std::env;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let command = env::args().nth(1).expect("no viewmd command provided");
    let filepath = env::args().nth(2).expect("incorrect filepath");
    // loop, when correct command provided
    if command == "vmd" {
        println!("command: {command} with filepath: {filepath}");

        let content = fs::read_to_string(filepath)?;
        println!("content : {content}");
    } else {
        println!("incorrect viewmd command provided");
    }

    Ok(())
}
