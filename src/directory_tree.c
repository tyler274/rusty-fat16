#include "directory_tree.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

void init_node(node_t *node, char *name, node_type_t type) {
    if (name == NULL) {
        name = strdup("ROOT");
        assert(name != NULL);
    }
    node->name = name;
    node->type = type;
}

file_node_t *init_file_node(char *name, size_t size, uint8_t *contents) {
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

int compare_inodes(const void *v1, const void *v2) {
    const node_t *p1 = (node_t *) v1;
    const node_t *p2 = (node_t *) v2;

    return strcmp(p1->name, p2->name);
}

void add_child_directory_tree(directory_node_t *dnode, node_t *child) {
    dnode->num_children += 1;
    dnode->children = realloc(dnode->children, dnode->num_children * sizeof(node_t *));
    dnode->children[dnode->num_children - 1] = child;
    qsort(dnode->children, dnode->num_children - 1, sizeof(node_t *), compare_inodes);
    // printf("added inode: %s", dnode->children[dnode->num_children - 1]->name);
    (void) dnode;
    (void) child;
}

void print_directory_tree(node_t *node) {
    printf("ROOT\n");
    size_t index = 0;
    if (node != NULL) {
        if (node->type == DIRECTORY_TYPE) {
            directory_node_t *cast_node = (directory_node_t *) node;
            // printf("%zu", cast_node->num_children);
            for (index; index < cast_node->num_children; index++) {
                printf("    %s\n", (cast_node->children)[index]->name);
            }
        }
        else {
            assert(node->type == FILE_TYPE);
            printf("    %s\n", node->name);
        }
    }
    (void) node;
}

void create_directory_tree(node_t *node) {
    (void) node;
}

void free_directory_tree(node_t *node) {
    if (node->type == FILE_TYPE) {
        file_node_t *fnode = (file_node_t *) node;
        free(fnode->contents);
    }
    else {
        assert(node->type == DIRECTORY_TYPE);
        directory_node_t *dnode = (directory_node_t *) node;
        for (size_t i = 0; i < dnode->num_children; i++) {
            free_directory_tree(dnode->children[i]);
        }
        free(dnode->children);
    }
    free(node->name);
    free(node);
}
