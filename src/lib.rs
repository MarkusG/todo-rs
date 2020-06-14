use std::fs;
use std::{error::Error, fmt};
use std::cmp::Ordering;

#[derive(Debug)]
pub enum TodoError {
    InvalidCommand,
    NotEnoughArguments
}

impl Error for TodoError {}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::InvalidCommand => write!(f, "Invalid command"),
            TodoError::NotEnoughArguments => write!(f, "Not enough arguments")
        }
    }
}

pub struct Config {
    pub verb: String,
    pub noun: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Err(Box::new(TodoError::NotEnoughArguments));
        }  

        let verb = args[1].clone();
        let noun = args[2..].join(" ").clone();

        Ok(Config { noun, verb })
    }
}

#[derive(Eq)]
struct Todo {
    index: i32,
    content: String
}

impl Todo {
    fn new(line: &str) -> Result<Todo, Box<dyn Error>> {
        let words: Vec<&str> = line.split(" ").collect();
        let index = words[0].parse::<i32>()?;
        let content = words[1..].join(" ").clone();

        Ok(Todo { index, content })
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
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
    let mut todos = contents.lines().map(|l| Todo::new(l).unwrap()).collect::<Vec<_>>();
    todos.sort();

    for todo in todos {
        println!("{}. {}", todo.index, todo.content);
    }
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
