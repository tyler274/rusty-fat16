#![allow(dead_code)]

use std::{fmt::Display, io::Write, os::unix::prelude::PermissionsExt};

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    name: Vec<u8>,
    node: NodeT,
}

fn recurse_print_helper(f: &mut std::fmt::Formatter, node: &Node, depth: u64) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "    ")?;
    }

    // TODO: Implement the right conversions for the "?" operator to bubble the utf errors up.
    write!(f, "{}", std::str::from_utf8(&node.name).unwrap())?;

    match &node.node {
        NodeT::Directory { children } => {
            for i in 0..children.len() {
                recurse_print_helper(f, &children[i], depth + 1)?;
            }
        }
        _ => (),
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
    File { size: u64, contents: Vec<u8> },
}

pub const D_MKDIR_MODE: u32 = 0o777;

// pub const D_PATH_INITIAL_SIZE: u32 = 3;

impl Node {
    pub fn init_file_node(mut name: &[u8], size: u64, contents: Vec<u8>) -> Node {
        if name.is_empty() {
            name = b"ROOT\x00";
        }

        Node {
            name: name.to_vec(),
            node: NodeT::File { size, contents },
        }
    }
    pub fn init_directory_node(mut name: &[u8]) -> Node {
        if name.is_empty() {
            name = b"ROOT\x00";
        }

        Node {
            name: name.to_vec(),
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

    fn recurse_create_tree(node: &Node, current_path: &[u8]) -> std::io::Result<()> {
        let mut new_path = current_path.to_vec();
        new_path.extend(&node.name);
        match &node.node {
            NodeT::Directory { children } => {
                new_path.extend(b"/\x00");

                // Convert the completed path into a format the standard lib likes.
                let new_path_str: &str = std::str::from_utf8(new_path.as_slice()).unwrap();

                // Create the directory
                std::fs::create_dir(new_path_str)?;
                let metadata = std::fs::metadata(new_path_str)?;
                let mut permissions = metadata.permissions();
                // Set the permissions of the directory
                permissions.set_mode(D_MKDIR_MODE);

                for child in children {
                    Self::recurse_create_tree(child, new_path.as_slice())?;
                }
            }
            NodeT::File { size: _, contents } => {
                let new_path_str: &str = std::str::from_utf8(new_path.as_slice()).unwrap();
                let mut current_file = std::fs::File::create(new_path_str)?;
                current_file.write_all(contents)?;
            }
        };
        Ok(())
    }

    pub fn create_directory_tree(node: &Node) -> std::io::Result<()> {
        let current_path = b"./\x00";
        Self::recurse_create_tree(node, current_path)
    }
}
