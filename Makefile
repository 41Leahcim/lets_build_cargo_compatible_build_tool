lib = src/lib.rs
src = src/main.rs
bootstrap_dir = target/bootstrap
bootstrap = $(bootstrap_dir)/freight
output_dir = target/debug
output = $(output_dir)/freight

$(bootstrap_dir):
	mkdir $(bootstrap_dir) -p

$(bootstrap): $(bootstrap_dir)
	rustc --crate-type=lib --crate-name=freight src/lib.rs --out-dir=$(bootstrap_dir)
	rustc --crate-type=bin --crate-name=freight --extern=freight -L $(bootstrap_dir) --edition=2021 src/main.rs --out-dir=$(bootstrap_dir)

$(output): $(bootstrap)
	$(bootstrap) build

help: $(output)
	$(output) help

run: help

clean:
	rm -r target
