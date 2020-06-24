#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "directory_tree.h"

void add_empty_file(node_t *root, const char *filename) {
    assert(root->type == DIRECTORY_TYPE);
    char *filename_copy = strdup(filename);
    assert(filename != NULL);
    add_file((directory_node_t *) root, filename_copy, 0);
    free(filename_copy);
}

int main(void) {
    node_t *root = init_directory_node("");
    add_empty_file(root, "a/b/c/c");
    add_empty_file(root, "a/b/c/a");
    add_empty_file(root, "a/b/c/e");
    add_empty_file(root, "a/b/c/b");
    add_empty_file(root, "a/b/c/d");
    add_empty_file(root, "a/b/d/d");
    add_empty_file(root, "a/b/d/a");
    add_empty_file(root, "a/b/d/b");
    add_empty_file(root, "a/b/d/c");
    print_directory_tree(root);
    free_directory_tree(root);
}
