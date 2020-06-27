#ifndef DIRECTORY_TREE_H
#define DIRECTORY_TREE_H

#include <inttypes.h>
#include <stddef.h>

typedef enum {
    FILE_TYPE,
    DIRECTORY_TYPE
} node_type_t;

typedef struct {
    node_type_t type;
    char *name;
} node_t;

typedef struct {
    node_t base;
    uint64_t size;
    char *contents;
} file_node_t;

typedef struct {
    node_t base;
    size_t num_children;
    node_t **children;
} directory_node_t;


file_node_t *init_file_node(char *name, uint64_t size, char *contents);
directory_node_t *init_directory_node(char *name);

void add_child_directory_tree(directory_node_t *dnode, node_t *child);

void print_directory_tree(node_t *node);
void create_directory_tree(node_t *node);
void free_directory_tree(node_t *node);

#endif /* DIRECTORY_TREE_H */
