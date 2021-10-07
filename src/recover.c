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

void recurse_follow(FILE *disk, directory_entry_t entry, directory_node_t *node,
                    bios_parameter_block_t *bpb) {
    if (!is_hidden(entry)) {
        // only follow up if the entry isn't hidden.
        char *file_name = get_file_name(entry);
        fseek(disk, get_offset_from_cluster(entry.first_cluster, *bpb), SEEK_SET);

        if (is_directory(entry)) {
            // if the entry is a directory follow this branch.
            node->children =
                realloc(node->children, sizeof(node_t *) * (node->num_children + 1));

            directory_entry_t *disk_dir_entry = malloc(sizeof(directory_entry_t));
            fread(disk_dir_entry, sizeof(directory_entry_t), 1, disk);

            if (strcmp(file_name, "\0") != 0) {
                // if the directory name isn't a null terminator, then we can use its
                // actual name
                node->children[node->num_children] =
                    (node_t *) init_directory_node(file_name);
                // now read this init 'd node' s child directories from the file and
                // recurse

                recurse_follow(disk, *disk_dir_entry,
                               (directory_node_t *) node->children[node->num_children],
                               bpb);
            }
            else {
                char *temp_name = malloc(sizeof(char) * 16);
                sprintf(temp_name, "%d", rand());
                node->children[node->num_children] =
                    (node_t *) init_directory_node(temp_name);
                recurse_follow(disk, *disk_dir_entry,
                               (directory_node_t *) node->children[node->num_children],
                               bpb);
            }
            node->num_children++;
            free(disk_dir_entry);
        }
        else {
            // if its a file follow this branch

            // allocate more space for another node
            node->children =
                realloc(node->children, sizeof(node_t *) * (node->num_children + 1));

            // Allocate a buffer for this file's contents
            uint8_t *file_contents = malloc(entry.file_size);

            // Read the contents of that file into the buffer.
            fread(file_contents, sizeof(uint8_t), entry.file_size, disk);
            if (strcmp(file_name, "\0") != 0) {
                node->children[node->num_children] =
                    (node_t *) init_file_node(file_name, entry.file_size, file_contents);
            }
            else {
                char *temp_name = malloc(sizeof(char) * 16);
                sprintf(temp_name, "%d.pdf", rand());
                node->children[node->num_children] =
                    (node_t *) init_file_node(file_name, entry.file_size, file_contents);
                // free(temp_name);
            }
            node->num_children++;
        }
    }
    else {
        // if the directory_entry_t passed is hidden, return.
        return;
    }
}

void follow(FILE *disk, directory_node_t *node, bios_parameter_block_t bpb) {
    // directory_entry_t *cast_entry = (directory_entry_t *) disk;
    fseek(disk, get_root_directory_location(bpb), SEEK_SET);
    directory_entry_t *cast_entry = malloc(sizeof(directory_entry_t) * 512);
    size_t read_disk = fread(cast_entry, sizeof(directory_entry_t), 512, disk);
    assert(read_disk == 512);
    for (size_t i = 0; i < 512; i++) {
        recurse_follow(disk, cast_entry[i], node, &bpb);
    }

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
