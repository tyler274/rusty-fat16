#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#include "directory_tree.h"

void init_node(node_t *node, char *name, node_type_t type) {
    if (!strlen(name)) {
        name = strdup("ROOT");
    }
    node->name = name;
    assert(node->name != NULL);
    node->type = type;
}

file_node_t *init_file_node(char *name, uint64_t size, char *contents) {
    file_node_t *node = malloc(sizeof(file_node_t));
    assert(node != NULL);
    init_node((node_t *) node, name, FILE_TYPE);
    node->size = size;
    node->contents = contents;
    return node;
}

directory_node_t *init_directory_node(char *name) {
    directory_node_t *node = malloc(sizeof(directory_node_t));
    assert(node != NULL);
    init_node((node_t *) node, name, DIRECTORY_TYPE);
    node->num_children = 0;
    node->children = NULL;
    return node;
}

void add_child_directory_tree(directory_node_t *dnode, node_t *child) {
    (void)dnode;
    (void)child;
}

void print_directory_tree(node_t *node) {
    (void)node;
}


void create_directory_tree(node_t *node) {
    (void)node;
}

void free_directory_tree(node_t *node) {
    if (node->type == FILE_TYPE) {
        file_node_t *fnode = (file_node_t *)node;
        free(node->name);
        free(fnode->contents);
        free(node);
    }
    else {
        directory_node_t *dnode = (directory_node_t *)node;
        for (size_t i = 0; i < dnode->num_children; i++) {
            free_directory_tree(dnode->children[i]);
        }
        free(node->name);
        free(dnode->children);
        free(node);
    }
}
