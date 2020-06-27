#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <stdbool.h>
#include <assert.h>
#include <ctype.h>

#include "fat16.h"
#include "directory_tree.h"

const size_t MASTER_BOOT_RECORD_SIZE = 0x20B;

void follow(FILE *disk, directory_node_t *node, bios_parameter_block_t bpb) {
    (void)disk;
    (void)node;
    (void)bpb;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "USAGE: %s <filename>\n", argv[0]);
        exit(1);
    }

    FILE *disk = fopen(argv[1], "rb");

    /* TODO: Initialize me  and read from the disk. */
    bios_parameter_block_t bpb = {0};

    directory_node_t *root = init_directory_node(""); 
    follow(disk, root, bpb);
    print_directory_tree((node_t *)root);
    create_directory_tree((node_t *)root);
    free_directory_tree((node_t *)root);
}
