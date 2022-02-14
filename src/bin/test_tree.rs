#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {

    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;

    static mut stderr: *mut FILE;

    fn fclose(__stream: *mut FILE) -> libc::c_int;

    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;

    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    fn free(__ptr: *mut libc::c_void);

    fn abort() -> !;

    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;

    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    fn strlen(_: *const libc::c_char) -> libc::c_ulong;

    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;

    fn chdir(__path: *const libc::c_char) -> libc::c_int;

    fn init_file_node(
        name: *mut libc::c_char,
        size: size_t,
        contents: *mut uint8_t,
    ) -> *mut file_node_t;

    fn init_directory_node(name: *mut libc::c_char) -> *mut directory_node_t;

    fn add_child_directory_tree(dnode: *mut directory_node_t, child: *mut node_t);

    fn print_directory_tree(node: *mut node_t);

    fn create_directory_tree(node: *mut node_t);

    fn free_directory_tree(node: *mut node_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;

pub type FILE = libc::FILE;
pub type uint8_t = __uint8_t;
pub type node_type_t = libc::c_uint;
pub const DIRECTORY_TYPE: node_type_t = 1;
pub const FILE_TYPE: node_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub type_0: node_type_t,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_node_t {
    pub base: node_t,
    pub size: size_t,
    pub contents: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory_node_t {
    pub base: node_t,
    pub num_children: size_t,
    pub children: *mut *mut node_t,
}
#[no_mangle]
pub static mut MKDIR_MODE: libc::c_uint = 0o777 as libc::c_int as libc::c_uint;
/* *
 * Performs a binary search to find the entry with the given name in a directory.
 * This assumes that the directory's entries are sorted by name.
 * Returns NULL if no matching entry is found.
 */
#[no_mangle]
pub unsafe extern "C" fn get_child(
    mut directory: *mut directory_node_t,
    mut name: *mut libc::c_char,
) -> *mut node_t {
    let mut left: size_t = 0 as libc::c_int as size_t;
    let mut right: size_t = (*directory).num_children;
    while left < right {
        let mut mid: size_t = left
            .wrapping_add(right)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut mid_child: *mut node_t = *(*directory).children.offset(mid as isize);
        let mut cmp: libc::c_int = strcmp(name, (*mid_child).name);
        if cmp == 0 as libc::c_int {
            return mid_child;
        }
        if cmp < 0 as libc::c_int {
            right = mid
        } else {
            /* cmp > 0 */
            left = mid.wrapping_add(1 as libc::c_int as libc::c_ulong)
        }
    }
    return 0 as *mut node_t;
}
/* *
 * Adds a file with the given path and contents to the directory tree.
 * Builds any missing intermediate directories.
 */
#[no_mangle]
pub unsafe extern "C" fn add_file(
    mut directory: *mut directory_node_t,
    mut path: *mut libc::c_char,
    mut contents: *mut libc::c_char,
) {
    let mut remaining_path: *mut libc::c_char = path;
    loop
    // Identify the next file/directory name in the path
    {
        let mut slash: *mut libc::c_char = strchr(remaining_path, '/' as i32);
        if !slash.is_null() {
            *slash = '\u{0}' as i32 as libc::c_char
        }
        let mut child: *mut node_t = get_child(directory, remaining_path);
        if slash.is_null() {
            // This is the last part of the path, so it represents a file
            if !child.is_null() {
                fprintf(
                    stderr,
                    b"File \'%s\' already exists\n\x00" as *const u8 as *const libc::c_char,
                    path,
                );
                abort();
            }
            let mut path_copy: *mut libc::c_char = strdup(remaining_path);
            if !path_copy.is_null() {
            } else {
                __assert_fail(
                    b"path_copy != NULL\x00" as *const u8 as *const libc::c_char,
                    b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
                    60 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void add_file(directory_node_t *, char *, char *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            let mut file_size: size_t = strlen(contents);
            let mut contents_copy: *mut libc::c_void = malloc(
                (::std::mem::size_of::<libc::c_char>() * file_size as usize) as libc::c_ulong,
            );
            if !contents_copy.is_null() {
            } else {
                __assert_fail(
                    b"contents_copy != NULL\x00" as *const u8 as *const libc::c_char,
                    b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
                    63 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void add_file(directory_node_t *, char *, char *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            memcpy(
                contents_copy,
                contents as *const libc::c_void,
                (::std::mem::size_of::<libc::c_char>() * file_size as usize) as libc::c_ulong,
            );
            child =
                init_file_node(path_copy, file_size, contents_copy as *mut uint8_t) as *mut node_t;
            add_child_directory_tree(directory, child);
            break;
        } else {
            // This is an intermediate directory
            if child.is_null() {
                let mut path_copy_0: *mut libc::c_char = strdup(remaining_path);
                if !path_copy_0.is_null() {
                } else {
                    __assert_fail(
                        b"path_copy != NULL\x00" as *const u8 as *const libc::c_char,
                        b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
                        73 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void add_file(directory_node_t *, char *, char *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                child = init_directory_node(path_copy_0) as *mut node_t;
                add_child_directory_tree(directory, child);
            } else if (*child).type_0 as libc::c_uint
                == DIRECTORY_TYPE as libc::c_int as libc::c_uint
            {
            } else {
                __assert_fail(
                    b"child->type == DIRECTORY_TYPE\x00" as *const u8 as *const libc::c_char,
                    b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
                    79 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void add_file(directory_node_t *, char *, char *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            directory = child as *mut directory_node_t;
            remaining_path = slash.offset(1 as libc::c_int as isize)
        }
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    // If the child already exists, it should be a directory
    // Program should be invoked as "bin/test_tree test-input-file.txt output-files"
    if argc == 3 as libc::c_int {
    } else {
        __assert_fail(
            b"argc == 3\x00" as *const u8 as *const libc::c_char,
            b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut test_input: *mut FILE = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\x00" as *const u8 as *const libc::c_char,
    );
    if !test_input.is_null() {
    } else {
        __assert_fail(
            b"test_input != NULL\x00" as *const u8 as *const libc::c_char,
            b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
            90 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut root: *mut directory_node_t = init_directory_node(0 as *mut libc::c_char);
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_capacity: size_t = 0 as libc::c_int as size_t;
    while getline(&mut line, &mut line_capacity, test_input) > 0 as libc::c_int as libc::c_long {
        /* Separate each line of the input file into path:contents
         * and add the given file to the directory tree */
        let mut colon: *mut libc::c_char = strchr(line, ':' as i32);
        if !colon.is_null() {
        } else {
            __assert_fail(
                b"colon != NULL\x00" as *const u8 as *const libc::c_char,
                b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
                99 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"int main(int, char **)\x00",
                ))
                .as_ptr(),
            );
        }
        *colon = '\u{0}' as i32 as libc::c_char;
        add_file(root, line, colon.offset(1 as libc::c_int as isize));
    }
    free(line as *mut libc::c_void);
    let mut result: libc::c_int = fclose(test_input);
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    print_directory_tree(root as *mut node_t);
    result = mkdir(*argv.offset(2 as libc::c_int as isize), MKDIR_MODE);
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    result = chdir(*argv.offset(2 as libc::c_int as isize));
    if result == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"result == 0\x00" as *const u8 as *const libc::c_char,
            b"src/test_tree.c\x00" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    create_directory_tree(root as *mut node_t);
    free_directory_tree(root as *mut node_t);
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
