STEPS = step0_repl step1_read_print step2_eval step3_env step4_if_fn_do

all: $(STEPS)

dist: mal

mal: step4_if_fn_do
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) reader.rs types.rs printer.rs
STEP2_DEPS = $(STEP1_DEPS)
STEP3_DEPS = $(STEP2_DEPS) env.rs

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP1_DEPS)
step2_eval: $(STEP2_DEPS)
step3_env: $(STEP3_DEPS)
step4_if_fn_do: $(STEP4_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
