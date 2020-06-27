#ifndef FAT16_H
#define FAT16_H

typedef struct __attribute__((__packed__)) {
    uint16_t bytes_per_sector;
    uint8_t sectors_per_cluster;
    uint16_t reserved_sectors;
    uint8_t num_fats;
    uint16_t max_root_entries;
    uint16_t logical_sectors;
    uint8_t media_descriptor;
    uint16_t sectors_per_fat;
    uint8_t padding[19];
    char volume_name[11];
    char type[8];
} bios_parameter_block_t;

typedef struct __attribute__((__packed__)) {
    char filename[8];
    char extension[3];
    uint8_t attribute;
    char reserved[8];
    uint16_t high_size;
    char time[2];
    char date[2];
    uint16_t first_cluster;
    uint32_t file_size;
} directory_entry_t;

size_t get_root_directory_location(bios_parameter_block_t bpb);
size_t get_offset_from_cluster(uint16_t cluster, bios_parameter_block_t bpb);
bool is_directory(directory_entry_t d);
bool is_hidden(directory_entry_t d);
char *get_file_name(char *filename, char *extension);

#endif /* FAT16_H */
