#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <stdbool.h>
#include <assert.h>
#include <ctype.h>

#include "fat16.h"

size_t get_root_directory_location(bios_parameter_block_t bpb) {
    return (1 + bpb.reserved_sectors + (bpb.num_fats *  bpb.sectors_per_fat))*bpb.bytes_per_sector;
}

size_t get_offset_from_cluster(uint16_t cluster, bios_parameter_block_t bpb) {
    size_t initial_offset = get_root_directory_location(bpb) + bpb.max_root_entries * sizeof(directory_entry_t);
    return initial_offset + (cluster- 2) * bpb.sectors_per_cluster * bpb.bytes_per_sector;
}

bool is_directory(directory_entry_t d) {
    return d.attribute & 0x10;
}

bool is_hidden(directory_entry_t d) {
    return d.filename[0] == '.' || d.filename[1] == '.' || d.attribute & 0x2 || d.attribute & 0x4 || d.attribute & 0x8 || d.attribute == 0x0; 
}

char *get_file_name(char *filename, char *extension) {
    char *full = malloc(8 + 1 + 3 + 1);
    int x = 0;
    int i = 0;
    while (filename[i] != ' ' && i < 8) {
        full[x++] = (uint8_t)filename[i++] == 0xe5 ? '?' : filename[i-1];
    }
    char *space = strchr(extension, ' ');
    if (space && space > extension) {
        full[x++] = '.';
        i = 0;
        while (space > extension + i && i < 3) {
            full[x++] = extension[i++];
        }
    }
    full[x] = '\0';
    return full;
}

