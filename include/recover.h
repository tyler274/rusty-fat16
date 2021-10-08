#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "directory_tree.h"
#include "fat16.h"

void recurse_follow(FILE *disk, directory_entry_t entry, directory_node_t *node,
                    bios_parameter_block_t *bpb);