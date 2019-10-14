STEPS = step0_repl

all: $(STEPS)

dist: mal

mal: step0_repl
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS =

step0_repl: $(STEP0_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
