//!
//! # jsonfile
//!
//! Json file reader / writer.
//!

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn read<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let file = std::fs::File::open(path)?;
    let data = serde_json::from_reader(file)?;
    Ok(data)
}

pub fn write<T: Serialize>(path: &Path, data: &T) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer(file, data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct Node {
        id: u32,
        name: String,
        children: Vec<Node>,
    }

    impl Node {
        pub fn new() -> Self {
            let mut node = Node {
                id: 1,
                name: String::from("parent"),
                children: Vec::new(),
            };

            for i in 1..3 {
                let idx = i + 1;
                node.children.push(Node {
                    id: idx,
                    name: format!("child{idx}").to_string(),
                    children: Vec::new(),
                });
            }

            node
        }
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let filepath = temp.path().join("test.json");
        let node = Node::new();

        let result = write(&filepath, &node);
        assert!(result.is_ok());
        assert!(filepath.exists(), "test.json should be created.");

        let result: anyhow::Result<Node> = read(&filepath);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(node, result);
    }

    #[test]
    fn it_return_err_when_file_not_exists() {
        let result: anyhow::Result<Node> = read(Path::new("noexists.json"));
        assert!(result.is_err());
    }

    #[test]
    fn write_is_overwrites_existing_file() {
        let temp = mktemp::TempDir::new().unwrap();
        let filepath = temp.path().join("test.json");

        let node1 = Node {
            id: 1,
            name: "test".to_string(),
            children: Vec::new(),
        };
        let _ = write(&filepath, &node1);
        let result1: Node = read(&filepath).unwrap();

        let mut node2 = node1.clone();
        node2.id = node1.id + 1;
        let _ = write(&filepath, &node2);
        let result2: Node = read(&filepath).unwrap();

        assert_ne!(result1, result2);
    }
}
