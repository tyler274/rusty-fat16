CC = clang-with-asan
CFLAGS = -Iinclude -Wall -Wextra -g

BINS = bin/fake bin/recover

all: $(BINS)

out/%.o: src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

bin/fake: out/fake.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@

bin/recover: out/recover.o out/fat16.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@


clean:
	rm -f out/* bin/* ROOT
