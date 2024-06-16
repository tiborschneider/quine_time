.PHONY: default clean

default: time matrix

time: time_quine.rs
	rustc time_quine.rs
	mv time_quine time

time_quine.rs: compile time.rs
	./compile time 133

matrix: matrix_quine.rs
	rustc matrix_quine.rs
	mv matrix_quine matrix

matrix_quine.rs: compile matrix.rs
	./compile matrix 150

compile: compile.rs
	rustc compile.rs

clean:
	rm -f compile time time_quine.rs matrix matrix_quine.rs
