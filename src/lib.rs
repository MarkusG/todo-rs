use std::{fs, fs::OpenOptions};
use std::io::{Read, Write};
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
    pub noun: Option<String>
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Err(Box::new(TodoError::NotEnoughArguments));
        }  

        let verb = args[1].clone();

        if args.len() > 2 {
            let noun = args[2..].join(" ").clone();
            return Ok(Config { noun: Some(noun), verb })
        }

        Ok(Config { noun: None, verb })
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
        "add" => {
            if let Some(noun) = config.noun {
                add(&noun)
            } else {
                Err(Box::new(TodoError::NotEnoughArguments))
            }
        }
        // note: Err and Error are NOT closely related
        // Err is a Result type and Error is a trait
        _ => Err(Box::new(TodoError::InvalidCommand))
    }
}

fn parse_todos(lines: &str) -> Result<Vec<Todo>, Box<dyn Error>> {
    Ok(lines.lines().map(|l| Todo::new(l).unwrap()).collect::<Vec<_>>())
}

fn list() -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string("todo.txt")?;
    let mut todos = parse_todos(&file_contents)?;
    todos.sort();

    for todo in todos {
        println!("{}. {}", todo.index, todo.content);
    }
    Ok(())
}

fn add(content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("todo.txt")?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let mut todos = parse_todos(&file_contents)?;
    todos.sort();

    let mut index: i32 = 1;
    for todo in todos {
        if todo.index != index {
            break;
        }
        index += 1;
    }
    
    writeln!(file, "{}", format!("{} {}", index, content))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let args: Vec<String> = vec!["argOne", "argTwo", "argThree", "argFour"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let config = Config::new(&args[..]).unwrap();
        assert_eq!(config.verb, "argTwo");
        if let Some(s) = config.noun {
            assert_eq!(s, "argThree argFour");
        } else {
            panic!("config.noun is None");
        }
    }

    #[test]
    fn parse() {
        let contents = "2 Something else\n1 Something\n4 Another thing\n";
        let todos = parse_todos(&contents).unwrap();
        assert_eq!(todos[0].index, 2);
        assert_eq!(todos[0].content, "Something else");
        assert_eq!(todos[1].index, 1);
        assert_eq!(todos[1].content, "Something");
        assert_eq!(todos[2].index, 4);
        assert_eq!(todos[2].content, "Another thing");
    }
}
