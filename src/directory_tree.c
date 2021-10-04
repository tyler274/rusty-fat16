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

void fill_buffer_by_depth(char *buffer, size_t *index, size_t *depth) {
    for (size_t i = 0; i < *depth; i += 1) {
        // buffer[i] = 0b00100000;
        strncpy(&buffer[*index + i], "    ", 4 * sizeof(char));
        index += 4;
        // buffer[i] = ' ';
    }
}

size_t get_output_buffer_size(size_t *depth) {
    return ((4 * (*depth) + 256) * sizeof(char));
}

void print_directory_tree(node_t *node) {
    // printf("ROOT\n");
    size_t index = 0;
    size_t depth = 0;
    depth += 1;
    // Hurray 256 character path limit in Fat32/NTFS
    size_t output_buffer_size = get_output_buffer_size(&depth);
    char *output_buffer = malloc(output_buffer_size);
    // magic numbner 5 for the root label length minus the null terminator.
    strncpy(&output_buffer[index], "ROOT\n", 5 * sizeof(char));
    index += 5;

    if (node != NULL) {
        if (node->type == DIRECTORY_TYPE) {
            directory_node_t *cast_node = (directory_node_t *) node;
            fill_buffer_by_depth(output_buffer, &index, &depth);
            // printf("%zu", cast_node->num_children);
            // output_buffer[];
            // for (index = 0; index < cast_node->num_children; index++) {
            //     printf("    %s\n", (cast_node->children)[index]->name);
            // }
            strncpy(&output_buffer[index], node->name, node->name_size);

            depth += 1;
            output_buffer = realloc(output_buffer, get_output_buffer_size(&depth));
        }
        else {
            assert(node->type == FILE_TYPE);
            fill_buffer_by_depth(output_buffer, &index, &depth);
            // output_buffer[]
        }
    }
    output_buffer[index] = '\0';

    fprintf(stdout, "%s\n", output_buffer);

    free(output_buffer);
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
