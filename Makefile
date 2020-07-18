CC = clang-with-asan
CFLAGS = -Iinclude -Wall -Wextra

BINS = bin/fake bin/recover

all: $(BINS)

out/%.o: src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

bin/fake: out/fake.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@

bin/recover: out/recover.o out/fat16.o out/directory_tree.o
	$(CC) $(CFLAGS) $^ -o $@

fake: bin/fake
	rm -rf $@ && mkdir $@ && cd $@ && ../$^

usb.dmg:
	curl "https://com.puter.systems/20fa/projects/assets/00/usb.dmg" -o $@

recovery: bin/recover usb.dmg
	rm -rf $@ && mkdir $@ && cd $@ && $(patsubst %,../%,$^)

clean:
	rm -rf out/* bin/* fake usb.dmg recovery
