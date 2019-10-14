STEPS = step0_repl step1_read_print

all: $(STEPS)

dist: mal

mal: step0_repl
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS =
STEP1_DEPS = $(STEP0_DEPS) reader.rs

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP1_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
