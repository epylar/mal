STEPS = step0_repl step1_read_print step2_eval

all: $(STEPS)

dist: mal

mal: step2_eval
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS =
STEP1_DEPS = $(STEP0_DEPS) reader.rs
STEP2_DEPS = $(STEP1_DEPS)

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP1_DEPS)
step2_eval: $(STEP2_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
