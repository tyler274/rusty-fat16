CC = clang-with-asan
CFLAGS = -Iinclude -Wall -Wextra -g

BINS = bin/fake_tar bin/read_tar

all: $(BINS)

out/%.o: src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

bin/fake_tar: out/fake_tar.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@

bin/read_tar: out/read_tar.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@


clean:
	rm -f out/* bin/* 
