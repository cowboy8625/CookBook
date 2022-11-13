use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = match File::open("text.txt") {
        Ok(f) => f,
        Err(e) => panic!("WHHHHHHHAAATTTTT: {}", e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {}
    }
    println!("File: {}", contents);
    let my_string = include!("text.txt");
    println!("{}", my_string);
}
