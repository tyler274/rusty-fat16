#include <assert.h>
#include <stdio.h>

#include "directory_tree.h"
#include "fat16.h"

const size_t MASTER_BOOT_RECORD_SIZE = 0x20B;
const size_t FILE_ALLOCATION_TABLES_SIZE = 78 * 512;

void recurse_follow(directory_entry_t *entry, directory_node_t *node,
                    bios_parameter_block_t *bpb, size_t entry_index) {
    if (!is_hidden(entry[entry_index])) {
        if (is_directory(entry[entry_index])) {
            node->

                init_directory_node();
        }
        else {
        }
    }
    else {
        return;
    }
}

void follow(FILE *disk, directory_node_t *node, bios_parameter_block_t bpb) {
    directory_entry_t *cast_entry = (directory_entry_t *) disk;
    recurse_follow(cast_entry, node, &bpb, 0);
    (void) disk;
    (void) node;
    (void) bpb;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "USAGE: %s <image filename>\n", argv[0]);
        return 1;
    }

    FILE *disk = fopen(argv[1], "r");
    if (disk == NULL) {
        fprintf(stderr, "No such image file: %s\n", argv[1]);
        return 1;
    }

    bios_parameter_block_t bpb;

    /* TODO: Write your code here. */
    fseek(disk, MASTER_BOOT_RECORD_SIZE, SEEK_SET);
    fread(&bpb, 1, sizeof(bios_parameter_block_t), disk);
    fseek(disk, get_root_directory_location(bpb), SEEK_SET);

    directory_node_t *root = init_directory_node(NULL);
    follow(disk, root, bpb);
    print_directory_tree((node_t *) root);
    create_directory_tree((node_t *) root);
    free_directory_tree((node_t *) root);

    int result = fclose(disk);
    assert(result == 0);
}
