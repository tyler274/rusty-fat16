#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "directory_tree.h"

node_t *new_file(char *filename, char contents[10]) {
    char *filename_copy = strdup(filename);
    size_t size = strlen(contents);
    char *contents_copy = strdup(contents);
    return (node_t *)init_file_node(filename_copy, size, contents_copy);
}

int main(void) {
    directory_node_t *root = init_directory_node("");
    directory_node_t *a = init_directory_node(strdup("a"));
    directory_node_t *a_b = init_directory_node(strdup("b"));
    directory_node_t *a_b_c = init_directory_node(strdup("c"));
    directory_node_t *a_b_d = init_directory_node(strdup("d"));
    
    add_child_directory_tree(root, (node_t *)a);
    add_child_directory_tree(a, (node_t *)a_b);
    add_child_directory_tree(a_b, (node_t *)a_b_c);
    add_child_directory_tree(a_b, (node_t *)a_b_d);

    add_child_directory_tree(a_b_c, new_file("c", "contents of c\n"));
    add_child_directory_tree(a_b_c, new_file("a", "contents of a\n"));
    add_child_directory_tree(a_b_c, new_file("e", "contents of e\n"));
    add_child_directory_tree(a_b_c, new_file("b", "contents of b\n"));
    add_child_directory_tree(a_b_d, new_file("d", "contents of d\n"));
    add_child_directory_tree(a_b_d, new_file("a", "contents of a\n"));
    add_child_directory_tree(a_b_c, new_file("d", "contents of d\n"));
    add_child_directory_tree(a_b_d, new_file("b", "contents of b\n"));
    add_child_directory_tree(a_b_d, new_file("c", "contents of c\n"));

    print_directory_tree((node_t *)root);
    create_directory_tree((node_t *)root);
    free_directory_tree((node_t *)root);
}

