#![allow(dead_code)]

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct BiosParameterBlock {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub max_root_entries: u16,
    pub logical_sectors: u16,
    pub media_descriptor: u8,
    pub sectors_per_fat: u16,
    pub padding: [u8; 19],
    pub volume_name: [u8; 11],
    pub type_0: [u8; 8],
}

const FILENAME_SIZE: usize = 8;
const EXTENSION_SIZE: usize = 3;
const FILENAME_MEM_SIZE: usize = std::mem::size_of::<[u8; FILENAME_SIZE]>();
const EXTENSION_MEM_SIZE: usize = std::mem::size_of::<[u8; EXTENSION_SIZE]>();

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct DirectoryEntry {
    pub filename: [u8; FILENAME_SIZE],
    pub extension: [u8; EXTENSION_SIZE],
    pub attribute: u8,
    pub reserved: [u8; 8],
    pub high_size: u16,
    pub time: [u8; 2],
    pub date: [u8; 2],
    pub first_cluster: u16,
    pub file_size: u32,
}

#[repr(u8)]
pub enum FatAttribute {
    ReadOnly = 0x1,
    Hidden = 0x2,
    System = 0x4,
    VolumeLabel = 0x8,
    SubDirectory = 0x10,
    Archive = 0x20,
    Device = 0x40,
}

pub const DELETED: u8 = 0xe5;

/**
 * Computes the position of the start of the root directory on a FAT16 disk.
 *
 * @param bpb the BIOS Parameter Block of the FAT16 disk
 * @return the index of the byte on the disk where the root directory begins
 */
pub fn get_root_directory_location(bpb: BiosParameterBlock) -> u64 {
    let fat_sectors: u64 = (bpb.num_fats as u64).wrapping_mul(bpb.sectors_per_fat as u64);
    return (1 + bpb.reserved_sectors as u64)
        .wrapping_add(fat_sectors)
        .wrapping_mul(bpb.bytes_per_sector as u64);
}

/**
 * Converts a cluster number to a position on a FAT16 disk.
 *
 * @param cluster the cluster number to look up
 * @param bpb the BIOS Parameter Block of the FAT16 disk
 * @return the index of the byte on the disk where the cluster begins
 */
pub fn get_offset_from_cluster(cluster: u64, bpb: BiosParameterBlock) -> u64 {
    return get_root_directory_location(bpb)
        .wrapping_add(
            (bpb.max_root_entries as u64)
                .wrapping_mul(std::mem::size_of::<DirectoryEntry>() as u64),
        )
        .wrapping_add(
            cluster
                .wrapping_sub(2_u64)
                .wrapping_mul(bpb.sectors_per_cluster as u64)
                .wrapping_mul(bpb.bytes_per_sector as u64),
        );
}

/**
 * Computes whether a directory entry represents a subdirectory.
 *
 * @param entry the directory entry
 * @return `true` if the entry represents a subdirectory, `false` if it is a file
 */
pub fn is_directory(entry: DirectoryEntry) -> bool {
    return (entry.attribute & FatAttribute::SubDirectory as u8) != 0;
}

/**
 * Computes whether a directory entry represents a hidden entry.
 * Hidden entries should be ignored when building the directory tree.
 *
 * @param entry the directory entry
 * @return `true` iff the entry is hidden
 */

pub fn is_hidden(entry: DirectoryEntry) -> bool {
    entry.filename[0] == b'.'
        || entry.filename[1] == b'.'
        || entry.attribute == 0
        || entry.attribute as i32
            & (FatAttribute::Hidden as i32
                | FatAttribute::System as i32
                | FatAttribute::VolumeLabel as i32)
            != 0
}

/**
 * Compute's a directory entry's filename.
 * If the entry is marked deleted, this will approximate its previous filename.
 *
 * @param entry the directory entry
 * @return a heap-allocated string containing the entry's filename
 */
pub fn get_file_name(entry: DirectoryEntry) -> Vec<u8> {
    let mut full: Vec<u8> = Vec::with_capacity(FILENAME_SIZE + 1 + EXTENSION_SIZE + 1);
    let mut full_index: usize = 0;
    let mut filename_index: usize = 0;
    while filename_index < FILENAME_MEM_SIZE {
        let fresh0 = filename_index;
        filename_index += 1;
        let filename_char: u8 = entry.filename[fresh0 as usize];
        if filename_char == b' ' {
            break;
        }
        let fresh1 = full_index;
        full_index += 1;
        full[fresh1 as usize] = if filename_char == DELETED {
            b'?'
        } else {
            filename_char
        }
    }
    if entry.extension[0] != b' ' {
        let fresh2 = full_index;
        full_index += 1;
        full[fresh2 as usize] = b'.';

        let mut extension_index: usize = 0;
        while extension_index < EXTENSION_MEM_SIZE {
            let fresh3 = extension_index;
            extension_index += 1;
            let extension_char: u8 = entry.extension[fresh3 as usize];
            if extension_char == b' ' {
                break;
            }
            let fresh4 = full_index;
            full_index += 1;
            full[fresh4] = extension_char;
        }
    }
    full[full_index] = b'\x00';
    full
}
