// #![allow(
//     dead_code,
//     mutable_transmutes,
//     non_camel_case_types,
//     non_snake_case,
//     non_upper_case_globals,
//     unused_assignments,
//     unused_mut
// )]
// use rusty_fat16::directory_tree::{Node, NodeT};
// use rusty_fat16::fat16::{BiosParameterBlock, DirectoryEntry, FatAttribute};

// type size_t = u64;

// #[no_mangle]
// pub static mut MASTER_BOOT_RECORD_SIZE: size_t = 0x20b_i32 as size_t;
// #[no_mangle]
// pub static mut FILE_ALLOCATION_TABLES_SIZE: size_t = (78_i32 * 512_i32) as size_t;
// #[no_mangle]
// pub static mut DIRECTORY_COUNT: size_t = 512_i32 as size_t;
// #[no_mangle]
// pub unsafe extern "C" fn follow_file_branch(
//     mut disk: *mut FILE,
//     mut entry: directory_entry_t,
//     mut node: *mut directory_node_t,
//     mut bpb: *mut bios_parameter_block_t,
// ) {
//     let mut file_name: *mut libc::c_char = get_file_name(entry);
//     // if its a file follow this branch
//     // Allocate a buffer for this file's contents
//     let mut file_contents: *mut uint8_t =
//         calloc(1_i32 as libc::c_ulong, entry.file_size as libc::c_ulong) as *mut uint8_t;
//     let mut seek_result: libc::c_int = fseek(
//         disk,
//         get_offset_from_cluster(entry.first_cluster as size_t, *bpb) as libc::c_long,
//         0_i32,
//     );
//     if seek_result == 0_i32 {
//     } else {
//         __assert_fail(b"seek_result == 0\x00" as *const u8 as
//                           *const libc::c_char,
//                       b"src/recover.c\x00" as *const u8 as
//                           *const libc::c_char,
//                       29_i32 as libc::c_uint,
//                       (*::std::mem::transmute::<&[u8; 97],
//                                                 &[libc::c_char; 97]>(b"void follow_file_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//     }
//     // Read the contents of that file into the buffer.
//     // printf("%s", file_name);
//     let mut read_result: size_t = fread(
//         file_contents as *mut libc::c_void,
//         ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
//         entry.file_size as libc::c_ulong,
//         disk,
//     );
//     if read_result == entry.file_size as libc::c_ulong {
//     } else {
//         __assert_fail(b"read_result == entry.file_size\x00" as *const u8 as
//                           *const libc::c_char,
//                       b"src/recover.c\x00" as *const u8 as
//                           *const libc::c_char,
//                       33_i32 as libc::c_uint,
//                       (*::std::mem::transmute::<&[u8; 97],
//                                                 &[libc::c_char; 97]>(b"void follow_file_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//     }
//     if strcmp(file_name, b"\x00\x00" as *const u8 as *const libc::c_char) != 0_i32 {
//         add_child_directory_tree(
//             node,
//             init_file_node(file_name, entry.file_size as size_t, file_contents) as *mut node_t,
//         );
//     };
// }
// #[no_mangle]
// pub unsafe extern "C" fn follow_directory_branch(
//     mut disk: *mut FILE,
//     mut entry: directory_entry_t,
//     mut node: *mut directory_node_t,
//     mut bpb: *mut bios_parameter_block_t,
// ) {
//     let mut file_name: *mut libc::c_char = get_file_name(entry);
//     let mut seek_result: libc::c_int = fseek(
//         disk,
//         get_offset_from_cluster(entry.first_cluster as size_t, *bpb) as libc::c_long,
//         0_i32,
//     );
//     if seek_result == 0_i32 {
//     } else {
//         __assert_fail(b"seek_result == 0\x00" as *const u8 as
//                           *const libc::c_char,
//                       b"src/recover.c\x00" as *const u8 as
//                           *const libc::c_char,
//                       46_i32 as libc::c_uint,
//                       (*::std::mem::transmute::<&[u8; 102],
//                                                 &[libc::c_char; 102]>(b"void follow_directory_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//     }
//     // if the entry is a directory follow this branch.
//     let mut disk_dir_entry: *mut directory_entry_t = calloc(
//         1_i32 as libc::c_ulong,
//         ::std::mem::size_of::<directory_entry_t>() as libc::c_ulong,
//     ) as *mut directory_entry_t;
//     let mut read_result: size_t = fread(
//         disk_dir_entry as *mut libc::c_void,
//         ::std::mem::size_of::<directory_entry_t>() as libc::c_ulong,
//         1_i32 as libc::c_ulong,
//         disk,
//     );
//     if read_result == 1_i32 as libc::c_ulong {
//     } else {
//         __assert_fail(b"read_result == 1\x00" as *const u8 as
//                           *const libc::c_char,
//                       b"src/recover.c\x00" as *const u8 as
//                           *const libc::c_char,
//                       51_i32 as libc::c_uint,
//                       (*::std::mem::transmute::<&[u8; 102],
//                                                 &[libc::c_char; 102]>(b"void follow_directory_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//     }
//     if strcmp(file_name, b"\x00\x00" as *const u8 as *const libc::c_char) != 0_i32 {
//         // if the directory name isn't a null terminator, then we can use its
//         // actual name
//         add_child_directory_tree(node, init_directory_node(file_name) as *mut node_t);
//         // now read this inited nodes child directories from the file and
//         // recurse

//         let mut is_null: bool = false;
//         let mut sibling_count: size_t = 0;

//         while is_null as libc::c_int == 0_i32 {
//             recurse_follow(
//                 disk,
//                 *disk_dir_entry,
//                 *(*node)
//                     .children
//                     .offset((*node).num_children.wrapping_sub(1_i32 as libc::c_ulong) as isize)
//                     as *mut directory_node_t,
//                 bpb,
//             );
//             seek_result = fseek(
//                 disk,
//                 get_offset_from_cluster(entry.first_cluster as size_t, *bpb).wrapping_add(
//                     (::std::mem::size_of::<directory_entry_t>() as libc::c_ulong)
//                         .wrapping_mul(sibling_count),
//                 ) as libc::c_long,
//                 0_i32,
//             );
//             if seek_result == 0_i32 {
//             } else {
//                 __assert_fail(b"seek_result == 0\x00" as *const u8 as
//                                   *const libc::c_char,
//                               b"src/recover.c\x00" as *const u8 as
//                                   *const libc::c_char,
//                               71_i32 as libc::c_uint,
//                               (*::std::mem::transmute::<&[u8; 102],
//                                                         &[libc::c_char; 102]>(b"void follow_directory_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//             }
//             let mut read_result_0: size_t = fread(
//                 disk_dir_entry as *mut libc::c_void,
//                 ::std::mem::size_of::<directory_entry_t>() as libc::c_ulong,
//                 1_i32 as libc::c_ulong,
//                 disk,
//             );
//             if read_result_0 == 1_i32 as libc::c_ulong {
//             } else {
//                 __assert_fail(b"read_result == 1\x00" as *const u8 as
//                                   *const libc::c_char,
//                               b"src/recover.c\x00" as *const u8 as
//                                   *const libc::c_char,
//                               74_i32 as libc::c_uint,
//                               (*::std::mem::transmute::<&[u8; 102],
//                                                         &[libc::c_char; 102]>(b"void follow_directory_branch(FILE *, directory_entry_t, directory_node_t *, bios_parameter_block_t *)\x00")).as_ptr());
//             }
//             let mut temp_name: *mut libc::c_char = get_file_name(*disk_dir_entry);
//             // printf("%s", temp_name);
//             if strcmp(temp_name, b"\x00\x00" as *const u8 as *const libc::c_char) == 0_i32 {
//                 is_null = 1_i32 != 0
//             }
//             sibling_count = sibling_count.wrapping_add(1);
//             free(temp_name as *mut libc::c_void);
//         }
//     }
//     free(disk_dir_entry as *mut libc::c_void);
// }
// #[no_mangle]
// pub unsafe extern "C" fn recurse_follow(
//     mut disk: *mut FILE,
//     mut entry: directory_entry_t,
//     mut node: *mut directory_node_t,
//     mut bpb: *mut bios_parameter_block_t,
// ) {
//     if !is_hidden(entry) {
//         // only follow up if the entry isn't hidden.
//         if is_directory(entry) {
//             follow_directory_branch(disk, entry, node, bpb);
//         } else {
//             follow_file_branch(disk, entry, node, bpb);
//         }
//     };
// }

// pub fn follow(mut disk: std::fs::File, mut node: &Node, mut bpb: BiosParameterBlock) {
//     // directory_entry_t *cast_entry = (directory_entry_t *) disk;
//     let mut seek_result: libc::c_int = fseek(
//         disk,
//         get_root_directory_location(bpb) as libc::c_long,
//         0_i32,
//     );
//     assert!(seek_result == 0);
//     let mut cast_entry: *mut directory_entry_t = calloc(
//         DIRECTORY_COUNT,
//         ::std::mem::size_of::<directory_entry_t>() as libc::c_ulong,
//     ) as *mut directory_entry_t;
//     let mut read_disk: size_t = fread(
//         cast_entry as *mut libc::c_void,
//         ::std::mem::size_of::<directory_entry_t>() as libc::c_ulong,
//         DIRECTORY_COUNT,
//         disk,
//     );

//     assert!(read_disk == DIRECTORY_COUNT);
//     let mut i: size_t = 0_i32 as size_t;
//     while i < DIRECTORY_COUNT {
//         recurse_follow(disk, *cast_entry.offset(i as isize), node, &mut bpb);
//         i = i.wrapping_add(1)
//     }
//     free(cast_entry as *mut libc::c_void);
// }
// unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
//     if argc != 2_i32 {
//         fprintf(
//             stderr,
//             b"USAGE: %s <image filename>\n\x00" as *const u8 as *const libc::c_char,
//             *argv.offset(0_i32 as isize),
//         );
//         return 1_i32;
//     }
//     let mut disk: *mut FILE = fopen(
//         *argv.offset(1_i32 as isize),
//         b"r\x00" as *const u8 as *const libc::c_char,
//     );
//     if disk.is_null() {
//         fprintf(
//             stderr,
//             b"No such image file: %s\n\x00" as *const u8 as *const libc::c_char,
//             *argv.offset(1_i32 as isize),
//         );
//         return 1_i32;
//     }
//     let mut bpb: bios_parameter_block_t = bios_parameter_block_t {
//         bytes_per_sector: 0,
//         sectors_per_cluster: 0,
//         reserved_sectors: 0,
//         num_fats: 0,
//         max_root_entries: 0,
//         logical_sectors: 0,
//         media_descriptor: 0,
//         sectors_per_fat: 0,
//         padding: [0; 19],
//         volume_name: [0; 11],
//         type_0: [0; 8],
//     };
//     /* TODO: Write your code here. */
//     let mut seek_result: libc::c_int = fseek(disk, MASTER_BOOT_RECORD_SIZE as libc::c_long, 0_i32);
//     if seek_result == 0_i32 {
//     } else {
//         __assert_fail(
//             b"seek_result == 0\x00" as *const u8 as *const libc::c_char,
//             b"src/recover.c\x00" as *const u8 as *const libc::c_char,
//             132_i32 as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                 b"int main(int, char **)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     let mut read_result: size_t = fread(
//         &mut bpb as *mut bios_parameter_block_t as *mut libc::c_void,
//         1_i32 as libc::c_ulong,
//         ::std::mem::size_of::<bios_parameter_block_t>() as libc::c_ulong,
//         disk,
//     );
//     if read_result == ::std::mem::size_of::<bios_parameter_block_t>() as libc::c_ulong {
//     } else {
//         __assert_fail(
//             b"read_result == sizeof(bios_parameter_block_t)\x00" as *const u8
//                 as *const libc::c_char,
//             b"src/recover.c\x00" as *const u8 as *const libc::c_char,
//             134_i32 as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                 b"int main(int, char **)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     seek_result = fseek(
//         disk,
//         get_root_directory_location(bpb) as libc::c_long,
//         0_i32,
//     );
//     if seek_result == 0_i32 {
//     } else {
//         __assert_fail(
//             b"seek_result == 0\x00" as *const u8 as *const libc::c_char,
//             b"src/recover.c\x00" as *const u8 as *const libc::c_char,
//             137_i32 as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                 b"int main(int, char **)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     let mut root: *mut directory_node_t = init_directory_node(std::ptr::null_mut::<libc::c_char>());
//     follow(disk, root, bpb);
//     print_directory_tree(root as *mut node_t);
//     create_directory_tree(root as *mut node_t);
//     free_directory_tree(root as *mut node_t);
//     let mut result: libc::c_int = fclose(disk);
//     if result == 0_i32 {
//     } else {
//         __assert_fail(
//             b"result == 0\x00" as *const u8 as *const libc::c_char,
//             b"src/recover.c\x00" as *const u8 as *const libc::c_char,
//             146_i32 as libc::c_uint,
//             (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
//                 b"int main(int, char **)\x00",
//             ))
//             .as_ptr(),
//         );
//     }
//     0
// }

pub fn main() {
    // let mut args: Vec<*mut libc::c_char> = Vec::new();
    // for arg in ::std::env::args() {
    //     args.push(
    //         ::std::ffi::CString::new(arg)
    //             .expect("Failed to convert argument into CString.")
    //             .into_raw(),
    //     );
    // }
    // args.push(::std::ptr::null_mut());
    // unsafe {
    //     ::std::process::exit(main_0(
    //         (args.len() - 1) as libc::c_int,
    //         args.as_mut_ptr() as *mut *mut libc::c_char,
    //     ) as i32)
    // }
}
