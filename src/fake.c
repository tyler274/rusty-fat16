#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "directory_tree.h"

directory_node_t *empty_directory(char *name) {
    char *name_copy = strdup(name);
    assert(name_copy != NULL);
    return init_directory_node(name_copy);
}

node_t *new_file(char *filename, char *contents) {
    char *filename_copy = strdup(filename);
    assert(filename_copy != NULL);
    size_t size = strlen(contents);
    uint8_t *contents_copy = malloc(size);
    assert(contents_copy != NULL);
    memcpy(contents_copy, contents, size);
    return (node_t *) init_file_node(filename_copy, size, contents_copy);
}

int main(void) {
    directory_node_t *root = init_directory_node(NULL);
    directory_node_t *a = empty_directory("a");
    directory_node_t *a_b = empty_directory("b");
    directory_node_t *a_b_c = empty_directory("c");
    directory_node_t *a_b_d = empty_directory("d");

    add_child_directory_tree(root, (node_t *) a);
    add_child_directory_tree(a, (node_t *) a_b);
    add_child_directory_tree(a_b, (node_t *) a_b_c);
    add_child_directory_tree(a_b, (node_t *) a_b_d);

    add_child_directory_tree(a_b_c, new_file("c", "contents of c\n"));
    add_child_directory_tree(a_b_c, new_file("a", "contents of a\n"));
    add_child_directory_tree(a_b_c, new_file("e", "contents of e\n"));
    add_child_directory_tree(a_b_c, new_file("b", "contents of b\n"));
    add_child_directory_tree(a_b_d, new_file("d", "contents of d\n"));
    add_child_directory_tree(a_b_d, new_file("a", "contents of a\n"));
    add_child_directory_tree(a_b_c, new_file("d", "contents of d\n"));
    add_child_directory_tree(a_b_d, new_file("b", "contents of b\n"));
    add_child_directory_tree(a_b_d, new_file("c", "contents of c\n"));

    print_directory_tree((node_t *) root);
    create_directory_tree((node_t *) root);
    free_directory_tree((node_t *) root);
}
