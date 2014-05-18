
RUSTC=rustc
RFLAGS='-L./hangeul'

all: compile
	

compile: module
	$(RUSTC) $(RFLAGS) aheui.rs

test:
	$(RUSTC) $(RFLAGS) --test aheui.rs
	./aheui
	rm aheui
	$(RUSTC) $(RFLAGS) aheui.rs
	./aheui hello.ah

run: compile
	./aheui

clean: clean-module
	rm aheui

module:
	cd hangeul && make

clean-module:
	cd hangeul && make clean
