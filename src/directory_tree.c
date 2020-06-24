#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "directory_tree.h"

static node_t *init_node(node_t *node, char *name, node_type_t type) {
    node->name = malloc(strlen(name) + 1);
    strcpy(node->name, name);
    node->type = type;
    return node;
}

node_t *init_file_node(char *name, uint64_t size) {
    file_node_t *fnode = (file_node_t *)init_node(malloc(sizeof(file_node_t)), name, FILE_TYPE);
    fnode->size = size;
    return (node_t *)fnode;
}

node_t *init_directory_node(char *name) {
    directory_node_t *dnode = (directory_node_t *)init_node(malloc(sizeof(directory_node_t)), name, DIRECTORY_TYPE);
    dnode->num_children = 0;
    dnode->children = NULL;
    return (node_t *)dnode;
}

void add_child_node(directory_node_t *dnode, node_t *child) {
    // TODO: Implement me!
    (void)dnode;
    (void)child;
}

void add_file(node_t *node, char *path, uint64_t size) {
    // TODO: Implement me!
    (void)node;
    (void)path;
    (void)size;
}


void print_directory_tree(node_t *node) {
    // TODO: Implement me!
    (void)node;
}

void free_directory_tree(node_t *node) {
    // TODO: Implement me!
    (void)node;
}
