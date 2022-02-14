#![allow(dead_code)]

use std::{error::Error, io::BufRead, os::unix::prelude::PermissionsExt};

use rusty_fat16::directory_tree::{Node, NodeT};

const MKDIR_MODE: u32 = 0o777;

/* *
 * Performs a binary search to find the entry with the given name in a directory.
 * This assumes that the directory's entries are sorted by name.
 * Returns None if no matching entry is found.
 */
pub fn get_child<'a>(directory: &'a Node, name: &[u8]) -> Option<&'a Node> {
    match &directory.node {
        NodeT::Directory { children } => {
            let mut left: usize = 0;
            let mut right = children.len();
            while left < right {
                let mid = (left + right) / 2;
                let mid_child = &children[mid];
                match name.cmp(mid_child.name.as_slice()) {
                    std::cmp::Ordering::Less => {
                        right = mid;
                    }
                    std::cmp::Ordering::Equal => {
                        return Some(mid_child);
                    }
                    std::cmp::Ordering::Greater => {
                        left = mid + 1;
                    }
                }
            }
        }
        NodeT::File { .. } => {
            eprintln!("Tried to get the child of a file node: {}", directory);
        }
    }
    None
}
/* *
 * Adds a file with the given path and contents to the directory tree.
 * Builds any missing intermediate directories.
 */
pub fn add_file<'a>(directory: &'a mut Node, path: &[u8], contents: &[u8]) {
    let mut remaining_path = path.to_vec();
    let mut new_child: Box<Node>;
    let mut temp_directory: Box<Node> = Box::new(directory.clone());
    loop {
        // Identify the next file/directory name in the path
        let slash = remaining_path.iter().position(|&x| x == b'/');
        if let Some(slash_found) = slash {
            remaining_path[slash_found] = b'\x00';
        }
        let child = get_child(&temp_directory, remaining_path.as_slice());

        if slash.is_none() {
            // This is the last part of the path, so it represents a file
            assert!(
                child.is_some(),
                "File '{}' already exists\n",
                std::str::from_utf8(path).unwrap()
            );

            Node::add_child_directory_tree(
                &mut temp_directory,
                Node::init_file_node(&remaining_path, contents),
            );
            break;
        }

        new_child = Box::new(Node::init_directory_node(&remaining_path));
        // This is an intermediate directory
        if child.is_none() {
            Node::add_child_directory_tree(&mut temp_directory, *new_child.clone());
        } else {
            // If the child already exists, it should be a directory
            assert!(
                child.unwrap().is_directory(),
                "The child node wasn't a directory\n"
            );
        }

        temp_directory = new_child.clone();
        remaining_path = remaining_path.split_at(slash.unwrap()).1.to_vec();
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Program should be invoked as "bin/test_tree test-input-file.txt output-files"
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() >= 3,
        "Program should be invoked as \"bin/test_tree test-input-file.txt output-files\""
    );
    let test_input: std::fs::File = std::fs::File::open(&args[1])?;
    let mut root = Node::init_directory_node(&[]);
    let line_reader = std::io::BufReader::new(test_input);
    for line in line_reader.lines() {
        /* Separate each line of the input file into path:contents
         * and add the given file to the directory tree */
        let r_line = line.unwrap();
        let line_split: usize = r_line.find(':').unwrap();

        add_file(
            &mut root,
            r_line[..line_split].as_bytes(),
            r_line[line_split + 1..].as_bytes(),
        );
    }

    println!("{}", root);
    std::fs::create_dir(&args[2])?;
    let metadata = std::fs::metadata(&args[2])?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(MKDIR_MODE);
    std::env::set_current_dir(&args[2])?;
    Node::create_directory_tree(&root)?;
    Ok(())
}
