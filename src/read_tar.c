#include <assert.h>
#include <stdio.h>

#include "directory_tree.h"

typedef struct __attribute__((__packed__)) {
    char name[100];
    char mode[8];
    char uid[8];
    char gid[8];
    char size[12];
    char mtime[12];
    char checksum[8];
    char typeflag[1];
    char linkname[100];
    char magic[6];
    char version[2];
    char uname[32];
    char gname[32];
    char devmajor[8];
    char devminor[8];
    char prefix[155];
    char pad[12];
} tar_header_t;

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "USAGE: %s <filename>\n", argv[0]);
        return 1;
    }

    node_t *root = init_directory_node("");

    FILE *f = fopen(argv[1], "r");
    tar_header_t header;
    (void) header;

    // TODO: Implement me!

    fclose(f);

    print_directory_tree(root);
    free_directory_tree(root);
}
