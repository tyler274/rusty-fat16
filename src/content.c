#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <stdbool.h>
#include <assert.h>

const uint8_t JPEG_MAGIC_NUMBER[3] = {0xFF, 0xD8};
const size_t MASTER_BOOT_RECORD_SIZE = 0x20B;

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
    char reserved[10];
    char time[2];
    char date[2];
    uint16_t first_cluster;
    uint32_t file_size;
} directory_entry_t;


int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "USAGE: %s <filename>\n", argv[0]);
        exit(1);
    }

    FILE *disk = fopen(argv[1], "rb");

    fseek(disk, MASTER_BOOT_RECORD_SIZE, SEEK_CUR);

    bios_parameter_block_t bpb;

    fread(&bpb, sizeof(bpb), 1, disk);

    assert(strncmp(bpb.type, "FAT 16", strlen("FAT 16")));

    size_t root_location =  (1 + bpb.reserved_sectors + (bpb.num_fats *  bpb.sectors_per_fat))*bpb.bytes_per_sector;
    fseek(disk, root_location + bpb.max_root_entries * sizeof(directory_entry_t), SEEK_SET);

    while (!feof(disk)) {
        uint8_t block[512];
        fread(&block, sizeof(block), 1, disk);

        //printf("%02x %02x\n", block[0], block[1]);
        if (block[0] == JPEG_MAGIC_NUMBER[0] && block[1] == JPEG_MAGIC_NUMBER[1]) {
            printf("Found! %d\n", 256*block[3] + block[4]);
        }
    }

}
