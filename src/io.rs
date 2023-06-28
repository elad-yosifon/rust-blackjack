use std::io::stdin;
use std::result::Result::{Err, Ok};
use std::string::String;

pub fn read_line(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(n) => {}
        Err(error) => unreachable!(),
    };
    input
}