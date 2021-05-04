use std::fmt;

enum RpnError {
    Io(std::io::Error),
    NumParse(std::num::ParseIntError),
}

impl fmt::Display for RpnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpnError::Io(cause) => write!(f, "I/O Error: {}", cause),
            RpnError::NumParse(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

impl From<std::io::Error> for RpnError {
    fn from(cause: std::io::Error) -> Self {
        Self::Io(cause)
    }
}

impl From<std::num::ParseIntError> for RpnError {
    fn from(cause: std::num::ParseIntError) -> Self {
        Self::NumParse(cause)
    }
}

fn get_int_from_file() -> Result<i32, RpnError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(RpnError::from)?;
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(RpnError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            RpnError::Io(cause) => println!("I/O Error: {}", cause),
            RpnError::NumParse(cause) => println!("Parse Error: {}", cause),
        },
    }
}
