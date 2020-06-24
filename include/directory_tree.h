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
} file_node_t;

typedef struct {
    node_t base;
    size_t num_children;
    node_t **children;
} directory_node_t;


node_t *init_file_node(const char *name, uint64_t size);
node_t *init_directory_node(const char *name);

void add_file(node_t *node, char *path, uint64_t size);
void print_directory_tree(node_t *node);
void free_directory_tree(node_t *node);

#endif /* DIRECTORY_TREE_H */
