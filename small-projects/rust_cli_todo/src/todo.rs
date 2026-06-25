use std::fs;
use std::io::{self, ErrorKind};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, task: String) -> Self {
        Self {
            id,
            task,
            completed: false,
        }
    }
}
fn read() -> Result<Vec<Todo>, io::Error> {
    match fs::read_to_string("todos.json") {
        Ok(text) => {
            if text.is_empty() {
                return Ok(Vec::new());
            }
            let v: Vec<Todo> = serde_json::from_str(&text)?;
            Ok(v)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}
fn write(v: Vec<Todo>) -> Result<(), io::Error> {
    let json_string = serde_json::to_string_pretty(&v)?;
    fs::write("todos.tmp", json_string)?;
    fs::rename("todos.tmp", "todos.json")?;
    Ok(())
}
pub fn add_task(task: String) -> Result<(), io::Error> {
    let mut v = read()?;

    let max_id = v.iter().map(|item| item.id).max().unwrap_or(0);
    v.push(Todo::new(max_id + 1, task));
    write(v)
}

pub fn list() -> Result<(), io::Error> {
    let v = read()?;

    for item in v {
        println!(
            "[{}]:{}--{}",
            item.id,
            item.task,
            if item.completed { "X" } else { "" }
        );
    }

    Ok(())
}
pub fn complete(id: u32) -> Result<(), io::Error> {
    let mut v = read()?;

    for item in &mut v {
        if item.id == id {
            item.completed = true;
            break;
        }
    }
    write(v)
}
pub fn delete(id: u32) -> Result<(), io::Error> {
    let mut v = read()?;

    let l1 = v.len();
    v.retain(|item| item.id != id);
    let l2 = v.len();
    if l1 == l2 {
        println!("[{}]:task not found", id);
    }
    write(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete() {
        let result = delete(1);
        assert!(result.is_ok());
    }
    #[test]
    fn test_complete() {
        let result = complete(1);
        assert!(result.is_ok());
    }
    #[test]
    fn test_add() {
        let result = add_task(String::from("abcd"));
        assert!(result.is_ok());
    }
    #[test]
    fn test_list() {
        let result = list();
        assert!(result.is_ok());
    }
}
