#include "directory_tree.h"

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>

const unsigned D_MKDIR_MODE = 0777;
const unsigned D_PATH_INITIAL_SIZE = 3;

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
    // qsort(dnode->children, dnode->num_children, sizeof(node_t *), compare_inodes);
    for (size_t i = dnode->num_children - 1; i > 0; i--) {
        if (strcmp(dnode->children[i]->name, dnode->children[i - 1]->name) < 0) {
            node_t *child_temp = dnode->children[i - 1];
            dnode->children[i - 1] = dnode->children[i];
            dnode->children[i] = child_temp;
        }
        else {
            break;
        }
    }
    // printf("added inode: %s", dnode->children[dnode->num_children - 1]->name);
}

// void fill_buffer_by_depth(char **buffer, size_t *line_position, size_t *depth) {
//     for (size_t i = 0; i < *depth; i += 1) {
//         // buffer[i] = 0b00100000;
//         strncpy(&(*buffer)[*line_position], "    ", 4 * sizeof(char));
//         *line_position += 4;
//     }
// }

// #define FAT_PATH_LIMIT 256

// size_t get_output_buffer_line_size(size_t *depth) {
//     printf("depth: %zu depth_size: %zu\n", *depth,
//            ((4 * (*depth) + FAT_PATH_LIMIT) * sizeof(char)));
//     return ((4 * (*depth) + FAT_PATH_LIMIT) * sizeof(char));
// }

// void recurse_tree(node_t *node, char ***output_buffer, size_t *depth, size_t
// *line_index,
//                   size_t *line_position) {
//     if (node != NULL) {
//         if (node->type == DIRECTORY_TYPE) {
//             *line_position = 0;

//             directory_node_t *cast_node = (directory_node_t *) node;
//             *output_buffer = realloc(
//                 *output_buffer, sizeof(char *) * (cast_node->num_children +
//                 *line_index));
//             (*output_buffer)[*line_index] = malloc(get_output_buffer_line_size(depth));
//             fill_buffer_by_depth(&(*output_buffer)[*line_index], line_position, depth);

//             strcpy(&((*output_buffer)[*line_index][*line_position]), node->name);
//             printf("%s", &((*output_buffer)[*line_index][*line_position]));
//             *depth += 1;
//             *line_index += 1;

//             for (size_t i = 0; i < cast_node->num_children; i++) {
//                 recurse_tree(cast_node->children[i], output_buffer, depth, line_index,
//                              line_position);
//             }
//             return;
//         }
//         else {
//             *line_position = 0;
//             assert(node->type == FILE_TYPE);

//             // file_node_t *cast_node = (file_node_t *) node;
//             fill_buffer_by_depth(&((*output_buffer)[*line_index]), line_position,
//             depth); strcpy(&((*output_buffer)[*line_index][*line_position]),
//             node->name);
//             // output_buffer[]
//             return;
//         }
//     }
//     else {
//         assert(node == NULL);
//         return;
//     }
// }

void recurse_print_helper(node_t *node, size_t depth) {
    for (size_t i = 0; i < depth; i++) {
        printf("    ");
    }

    printf("%s\n", node->name);

    if (node->type == DIRECTORY_TYPE) {
        directory_node_t *cast_node = (directory_node_t *) node;
        for (size_t i = 0; i < cast_node->num_children; i++) {
            recurse_print_helper(cast_node->children[i], depth + 1);
        }
    }
}

void print_directory_tree(node_t *node) {
    size_t depth = 0;
    recurse_print_helper(node, depth);
    // size_t line_position = 0;
    // size_t line_index = 0;
    // size_t depth = 0;

    // Hurray 256 character path limit in Fat32/NTFS
    // size_t output_buffer_line_size = get_output_buffer_size(&depth);
    // char **output_buffer = malloc(sizeof(char *));
    // output_buffer[line_index] = malloc(get_output_buffer_line_size(&depth));
    // strcpy(&output_buffer[line_index][line_position], "ROOT\n");
    // // Size of the above ROOT name (and null terminator)
    // line_index++;
    // depth++;

    // recurse_tree(node, &output_buffer, &depth, &line_index, &line_position);
    // for (size_t i = 0; i < line_index; i++) {
    //     fprintf(stdout, "%s\n", output_buffer[i]);
    //     free(output_buffer[i]);
    // }

    // free(output_buffer);
    // (void) node;
}

void recurse_create_tree(node_t *node, char *current_path) {
    char *new_path =
        malloc(sizeof(char) * (strlen(node->name) + strlen(current_path) + 2));
    strcpy(new_path, current_path);
    strcat(new_path, node->name);

    if (node->type == DIRECTORY_TYPE) {
        directory_node_t *cast_node = (directory_node_t *) node;
        strcat(new_path, "/");
        int result = mkdir(new_path, D_MKDIR_MODE);
        // assert(result == 0);
        (void) result;
        for (size_t i = 0; i < cast_node->num_children; i++) {
            recurse_create_tree(cast_node->children[i], new_path);
        }
    }
    else {
        assert(node->type == FILE_TYPE);
        file_node_t *cast_node = (file_node_t *) node;
        FILE *current_file = fopen(new_path, "w+");
        assert(current_file != NULL);

        size_t written =
            fwrite(cast_node->contents, sizeof(uint8_t), cast_node->size, current_file);
        assert(written == cast_node->size);

        int close_result = fclose(current_file);
        assert(close_result == 0);
    }
    free(new_path);
}

void create_directory_tree(node_t *node) {
    char *current_path = malloc(sizeof(char) * D_PATH_INITIAL_SIZE);
    strcpy(current_path, "./");

    recurse_create_tree(node, current_path);
    free(current_path);
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
