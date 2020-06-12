use std::fs;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum TodoError {
    InvalidCommand
}

impl Error for TodoError {}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid command")
    }
}

pub struct Config {
    pub verb: String,
    pub noun: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }  
        let verb = args[1].clone();
        let noun = args[2].clone();

        Ok(Config { noun, verb })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.verb.as_str() {
        "list" => list(),
        // note: Err and Error are NOT closely related
        // Err is a Result type and Error is a trait
        _ => Err(Box::new(TodoError::InvalidCommand))
    }
}

fn list() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("todo.txt")?;
    println!("{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let args: Vec<String> = vec!["argOne", "argTwo", "argThree"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let config = Config::new(&args[..]).unwrap();
        assert_eq!(config.verb, "argTwo");
        assert_eq!(config.noun, "argThree");
    }
}
