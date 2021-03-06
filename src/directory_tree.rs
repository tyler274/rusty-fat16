#![allow(dead_code)]

use std::{fmt::Display, io::Write, os::unix::prelude::PermissionsExt};

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub node: NodeT,
}

fn recurse_print_helper(f: &mut std::fmt::Formatter, node: &Node, depth: u64) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "    ")?;
    }

    // TODO: Implement the right conversions for the "?" operator to bubble the utf errors up.
    write!(f, "{}", &node.name)?;

    if let NodeT::Directory { children } = &node.node {
        for child in children {
            recurse_print_helper(f, child, depth + 1)?;
        }
    };

    Ok(())
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let depth: u64 = 0;

        recurse_print_helper(f, self, depth)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NodeT {
    Directory { children: Vec<Node> },
    File { contents: Vec<u8> },
}

pub const D_MKDIR_MODE: u32 = 0o777;

// pub const D_PATH_INITIAL_SIZE: u32 = 3;

impl Node {
    pub fn is_directory(&self) -> bool {
        match self.node {
            NodeT::Directory { .. } => true,
            NodeT::File { .. } => false,
        }
    }

    pub fn init_file_node(mut name: &str, contents: &[u8]) -> Node {
        if name.is_empty() {
            name = "ROOT";
        }

        Node {
            name: name.to_string(),
            node: NodeT::File {
                contents: contents.to_vec(),
            },
        }
    }
    pub fn init_directory_node(mut name: &str) -> Node {
        if name.is_empty() {
            name = "ROOT";
        }

        Node {
            name: name.to_string(),
            node: NodeT::Directory {
                children: Vec::new(),
            },
        }
    }

    pub fn add_child_directory_tree(d_node: &mut Node, child: Node) {
        match &mut d_node.node {
            NodeT::Directory { children } => {
                children.push(child);
                children.sort_by(|a, b| a.name.cmp(&b.name));
            }
            NodeT::File { .. } => eprintln!("Cannot add a child to a file node."),
        }
    }

    fn recurse_create_tree(node: &Node, current_path: &str) -> std::io::Result<()> {
        let mut new_path = current_path.to_string();
        new_path += &node.name;
        match &node.node {
            NodeT::Directory { children } => {
                new_path += "/";

                // Create the directory
                std::fs::create_dir(&new_path)?;
                let metadata = std::fs::metadata(&new_path)?;
                let mut permissions = metadata.permissions();
                // Set the permissions of the directory
                permissions.set_mode(D_MKDIR_MODE);

                for child in children {
                    Self::recurse_create_tree(child, &new_path)?;
                }
            }
            NodeT::File { contents } => {
                let mut current_file = std::fs::File::create(&new_path)?;
                current_file.write_all(contents)?;
            }
        };
        Ok(())
    }

    pub fn create_directory_tree(node: &Node) -> std::io::Result<()> {
        let current_path = "./";
        Self::recurse_create_tree(node, current_path)
    }
}
