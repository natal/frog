#nalgebra_lib_path=./lib/nalgebra/lib

all:
	mkdir -p bin
	mkdir -p lib
	rust build --lib --opt-level=3 src/lib.rs -L./lib/nalgebra/lib --out-dir lib
	rust build --opt-level=3 examples/dejong.rs -L./lib/nalgebra/lib -L./lib --out-dir bin

deps:
	make -C lib/nalgebra


distcheck:
	make deps
	make

clean:
	rm -rf bin/* lib/*lib*
