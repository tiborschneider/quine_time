.PHONY: default clean

default: quine

quine: quine_compiled.rs
	rustc quine_compiled.rs
	mv quine_compiled quine

quine_compiled.rs: compile quine.rs
	./compile

compile: compile.rs
	rustc compile.rs

clean:
	rm -f compile quine quine_compiled.rs
