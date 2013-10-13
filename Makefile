nalgebra_lib_path=./lib/nalgebra/lib
build_ex_cmd=rustc --opt-level=3 -L./lib -L$(nalgebra_lib_path) --out-dir bin

all:
	mkdir -p bin
	mkdir -p lib
	rustc --lib --opt-level=3 src/lib.rs -L$(nalgebra_lib_path) --out-dir lib
	$(build_ex_cmd) examples/circuit.rs
	$(build_ex_cmd) examples/dejong.rs
	$(build_ex_cmd) examples/michalewicz.rs
	$(build_ex_cmd) examples/dejongf2.rs
	$(build_ex_cmd) examples/dejongf3.rs
	$(build_ex_cmd) examples/goldstein.rs
	$(build_ex_cmd) examples/rosenbrock.rs
	$(build_ex_cmd) examples/schwefel.rs

deps:
	make -C lib/nalgebra


distcheck:
	make deps
	make

clean:
	rm -rf bin/* lib/*lib*
