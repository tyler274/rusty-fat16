#include <stdlib.h>
#include <string.h>

#include "directory_tree.h"

#define F(str) s = malloc(sizeof(char *)*(strlen(str) + 1)); strcpy(s, str); add_file(root, s, 0); free(s); 

int main(void) {
    node_t *root = init_directory_node("");
    char *s;
    F("a/b/c/c");
    F("a/b/c/a");
    F("a/b/c/e");
    F("a/b/c/b");
    F("a/b/c/d");
    F("a/b/d/d");
    F("a/b/d/a");
    F("a/b/d/b");
    F("a/b/d/c");
    print_directory_tree(root);
    free_directory_tree(root);
}
