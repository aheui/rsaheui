
RUSTC=rustc
RLIBFLAGS='-L./hangeul'
RFLAGS='-L.'

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
	cd hangeul && make
	cp hangeul/*.rlib .

clean-module:
	cd hangeul && make clean
