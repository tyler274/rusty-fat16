#![allow(dead_code)]

use rusty_fat16::directory_tree::{Node, NodeT};

const MKDIR_MODE: libc::c_uint = 0o777 as libc::c_int as libc::c_uint;

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
                if name == mid_child.name.as_slice() {
                    return Some(mid_child);
                } else if name < mid_child.name.as_slice() {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }
        NodeT::File { .. } => {
            eprintln!("Tried to get the child of a file node: {}", directory);
        }
    }
    return None;
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
        let child = get_child(&temp_directory, &remaining_path.as_slice());

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

unsafe fn main_0(mut _argc: libc::c_int, mut _argv: *mut *mut libc::c_char) -> libc::c_int {
    // If the child already exists, it should be a directory
    // Program should be invoked as "bin/test_tree test-input-file.txt output-files"
    // if argc == 3 as libc::c_int {
    // } else {
    //     __assert_fail(
    //         b"argc == 3\x00" as *const u8 as *const libc::c_char,
    //         b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //         88 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //             b"int main(int, char **)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    // let mut test_input: *mut FILE = fopen(
    //     *argv.offset(1 as libc::c_int as isize),
    //     b"r\x00" as *const u8 as *const libc::c_char,
    // );
    // if !test_input.is_null() {
    // } else {
    //     __assert_fail(
    //         b"test_input != NULL\x00" as *const u8 as *const libc::c_char,
    //         b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //         90 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //             b"int main(int, char **)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    // let mut root: *mut directory_node_t = init_directory_node(0 as *mut libc::c_char);
    // let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    // let mut line_capacity: size_t = 0 as libc::c_int as size_t;
    // while getline(&mut line, &mut line_capacity, test_input) > 0 as libc::c_int as libc::c_long {
    //     /* Separate each line of the input file into path:contents
    //      * and add the given file to the directory tree */
    //     let mut colon: *mut libc::c_char = strchr(line, ':' as i32);
    //     if !colon.is_null() {
    //     } else {
    //         __assert_fail(
    //             b"colon != NULL\x00" as *const u8 as *const libc::c_char,
    //             b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //             99 as libc::c_int as libc::c_uint,
    //             (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //                 b"int main(int, char **)\x00",
    //             ))
    //             .as_ptr(),
    //         );
    //     }
    //     *colon = '\u{0}' as i32 as libc::c_char;
    //     add_file(root, line, colon.offset(1 as libc::c_int as isize));
    // }
    // free(line as *mut libc::c_void);
    // let mut result: libc::c_int = fclose(test_input);
    // if result == 0 as libc::c_int {
    // } else {
    //     __assert_fail(
    //         b"result == 0\x00" as *const u8 as *const libc::c_char,
    //         b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //         105 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //             b"int main(int, char **)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    // print_directory_tree(root as *mut node_t);
    // result = mkdir(*argv.offset(2 as libc::c_int as isize), MKDIR_MODE);
    // if result == 0 as libc::c_int {
    // } else {
    //     __assert_fail(
    //         b"result == 0\x00" as *const u8 as *const libc::c_char,
    //         b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //         109 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //             b"int main(int, char **)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    // result = chdir(*argv.offset(2 as libc::c_int as isize));
    // if result == 0 as libc::c_int {
    // } else {
    //     __assert_fail(
    //         b"result == 0\x00" as *const u8 as *const libc::c_char,
    //         b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
    //         111 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
    //             b"int main(int, char **)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    // create_directory_tree(root as *mut node_t);
    // free_directory_tree(root as *mut node_t);
    return 0;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
