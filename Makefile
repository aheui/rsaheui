
HANGEUL_PATH='hangeul'
RUSTC=rustc
RLIBFLAGS=-O -L./$(HANGEUL_PATH)
RFLAGS=-O -L. -L./$(HANGEUL_PATH) -Z lto

all: lib aheui
	

lib: module
	$(RUSTC) $(RLIBFLAGS) lib.rs

aheui: lib
	$(RUSTC) $(RFLAGS) aheui.rs

test-test: lib
	$(RUSTC) $(RFLAGS) --test test.rs
	./test

test-snippets: aheui
	if [ -e snippets ]; then cd snippets && git pull; else git clone https://github.com/aheui/snippets; fi
	cd snippets && AHEUI=../aheui bash test.sh

test: test-test test-snippets
	

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
