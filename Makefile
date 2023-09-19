lib = src/lib.rs
src = src/main.rs
stage0_dir = target/bootstrap
stage0 = target/bootstrap/freight
stage1 = target/bootstrap_stage1/freight_stage1

$(stage0_dir):
	mkdir $(stage0_dir) -p

static_library: $(stage0_dir)
	rustc --crate-type=lib --crate-name=freight src/lib.rs --out-dir=$(stage0_dir)

dynamic_library: $(stage0_dir)
	rustc --crate-type=cdylib --crate-name=freight src/lib.rs --out-dir=$(stage0_dir)

$(stage0): static_library
	rustc --crate-type=bin --crate-name=freight --extern=freight -L $(stage0_dir) --edition=2021 src/main.rs --out-dir=$(stage0_dir)

$(stage1): $(stage0)
	$(stage0)

run: $(stage1)
	$(stage1)

help: $(stage1)
	$(stage1) help

clean:
	rm -r target
