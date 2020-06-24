#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "directory_tree.h"

void init_node(node_t *node, const char *name, node_type_t type) {
    node->name = strdup(name);
    assert(node->name != NULL);
    node->type = type;
}

node_t *init_file_node(const char *name, uint64_t size) {
    file_node_t *node = malloc(sizeof(file_node_t));
    assert(node != NULL);
    init_node((node_t *) node, name, FILE_TYPE);
    node->size = size;
    return (node_t *) node;
}

node_t *init_directory_node(const char *name) {
    directory_node_t *node = malloc(sizeof(directory_node_t));
    assert(node != NULL);
    init_node((node_t *) node, name, DIRECTORY_TYPE);
    node->num_children = 0;
    node->children = NULL;
    return (node_t *) node;
}

void add_child_node(directory_node_t *dnode, node_t *child) {
    // TODO: Implement me!
    (void) dnode;
    (void) child;
}

void add_file(node_t *node, char *path, uint64_t size) {
    // TODO: Implement me!
    (void) node;
    (void) path;
    (void) size;
}

void print_directory_tree(node_t *node) {
    // TODO: Implement me!
    (void) node;
}

void free_directory_tree(node_t *node) {
    // TODO: Implement me!
    (void) node;
}
