use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
enum RpnError {
    #[error("failed to read starting from {0}")]
    ReadError(String),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, RpnError> {
    let path = "number.txt";

    let num_str = fs::read_to_string(path).map_err(|_| RpnError::ReadError(path.into()))?;
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(RpnError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
