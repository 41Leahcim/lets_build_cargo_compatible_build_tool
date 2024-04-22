lib = src/lib.rs
src = src/main.rs
bootstrap_dir = target/bootstrap
bootstrap = $(bootstrap_dir)/freight
output_dir = target/debug
output = $(output_dir)/freight

build: clean $(output)

run: $(output)
	$(output) help

test: $(output)
	mkdir -p target/test
	@# Test that we can pass args to the tests
	$(output) test ignored-arg -- --list
	
	@# Actually run tests
	$(output) test

clean:
	rm -r target

$(bootstrap_dir):
	mkdir $(bootstrap_dir) -p

$(bootstrap): $(bootstrap_dir)
	@# Build crate dependencies
	rustc src/lib.rs --edition 2021 --crate-type=lib --crate-name=freight --out-dir=$(bootstrap_dir)

	@# Create the executable
	rustc src/main.rs --edition 2021 --crate-type=bin --crate-name=freight --out-dir=$(bootstrap_dir) -L $(bootstrap_dir) --extern=freight 

$(output): $(bootstrap)
	$(bootstrap) build
