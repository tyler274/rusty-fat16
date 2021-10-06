#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "directory_tree.h"
#include "fat16.h"

const size_t MASTER_BOOT_RECORD_SIZE = 0x20B;
const size_t FILE_ALLOCATION_TABLES_SIZE = 78 * 512;

void recurse_follow(FILE *disk, directory_entry_t *entry, directory_node_t *node,
                    bios_parameter_block_t *bpb, size_t entry_index) {
    if (!is_hidden(entry[entry_index])) {
        // only follow up if the entry isn't hidden.
        if (is_directory(entry[entry_index])) {
            // if the entry is a directory follow this branch.
            node->children =
                realloc(node->children, sizeof(node_t *) * (node->num_children + 1));

            directory_entry_t *disk_entries = malloc(entry[entry_index].file_size);
            fseek(disk, get_offset_from_cluster(entry[entry_index].first_cluster, *bpb),
                  SEEK_SET);
            fread(disk_entries, sizeof(uint8_t), entry[entry_index].file_size, disk);

            if (strcmp(get_file_name(entry[entry_index]), "\0") != 0) {
                node->children[node->num_children] =
                    (node_t *) init_directory_node(get_file_name(entry[entry_index]));
                // now read this init'd node's child directories from the file
            }
            else {
                char *temp_name = malloc(sizeof(char) * 8);
                sprintf(temp_name, "%zu", entry_index);
                // directory_node_t *temp_dir = init_directory_node(temp_name);
                node->children[node->num_children] =
                    (node_t *) init_directory_node(temp_name);
                free(temp_name);

                recurse_follow(disk, entry,
                               (directory_node_t *) node->children[node->num_children],
                               bpb, entry_index + 1);
            }
            node->num_children++;
        }
        else {
            // if its a file
            node->children =
                realloc(node->children, sizeof(node_t *) * (node->num_children + 1));
            uint8_t *file_contents = malloc(entry[entry_index].file_size);
            fseek(disk, get_offset_from_cluster(entry[entry_index].first_cluster, *bpb),
                  SEEK_SET);
            fread(file_contents, sizeof(uint8_t), entry[entry_index].file_size, disk);
            if (strcmp(get_file_name(entry[entry_index]), "\0") != 0) {
                // node->children[node->num_children] = (node_t *) init_file_node(
                //     get_file_name(entry[entry_index]), entry[entry_index].file_size,
                //     entry[entry_index].first_cluster);
            }
            else {
                char *temp_name = malloc(sizeof(char) * 8);
                sprintf(temp_name, "%zu.pdf", entry_index);
                // file_node_t temp_file = init_file_node(temp_name);
                // node->children[node->num_children] = (node_t *) temp_file;
                free(temp_name);
            }
            node->num_children++;
        }
    }
    else {
        return;
    }
}

void follow(FILE *disk, directory_node_t *node, bios_parameter_block_t bpb) {
    // directory_entry_t *cast_entry = (directory_entry_t *) disk;
    directory_entry_t *cast_entry = malloc(sizeof(directory_entry_t) * 512);
    size_t read_disk = fread(cast_entry, sizeof(directory_entry_t), 512, disk);
    assert(read_disk == 512);
    recurse_follow(disk, cast_entry, node, &bpb, 0);
    free(cast_entry);
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
