
HANGEUL_PATH='hangeul'
RUSTC=rustc
RLIBFLAGS=-O -L./$(HANGEUL_PATH)
RFLAGS=-O -L. -L./$(HANGEUL_PATH) -Z lto

all: lib aheui
	

lib: module
	$(RUSTC) $(RLIBFLAGS) lib.rs

aheui: lib
	$(RUSTC) $(RFLAGS) aheui.rs

test: aheui
	$(RUSTC) $(RFLAGS) --test test.rs
	./test
	rm aheui

	$(RUSTC) $(RFLAGS) aheui.rs
	./aheui snippets/hello-world/hello-world.puzzlet.aheui
	./aheui snippets/hello-world/hello.puzzlet.aheui

install: aheui
	cp ./aheui /usr/local/bin/rsaheui

dist-clean:
	rm /usr/local/bin/rsaheui

clean: clean-module
	rm aheui test *.rlib

module:
	cd $(HANGEUL_PATH) && make

clean-module:
	cd $(HANGEUL_PATH) && make clean
