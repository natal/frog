#nalgebra_lib_path=./lib/nalgebra/lib

all:
	mkdir -p bin
	mkdir -p lib
	rustc --lib --opt-level=3 src/lib.rs --out-dir lib
	rustc --opt-level=3 examples/dejong.rs -L./lib --out-dir bin
	rustc --opt-level=3 examples/michalewicz.rs -L./lib --out-dir bin
	rustc --opt-level=3 examples/dejongf2.rs -L./lib --out-dir bin
	rustc --opt-level=3 examples/dejongf3.rs -L./lib --out-dir bin
	rustc --opt-level=3 examples/goldstein.rs -L./lib --out-dir bin
	rustc --opt-level=3 examples/rosenbrock.rs -L./lib --out-dir bin

#deps:
#	make -C lib/nalgebra
#	rust build $(rust_sfml_rc) --opt-level 3 --out-dir lib
#	

distcheck:
	make deps
	make

clean:
	rm -rf bin/* lib/*lib*
