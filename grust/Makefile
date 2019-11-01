STEPS = step0_repl step1_read_print step2_eval step3_env step4_if_fn_do step5_tco step6_file step7_quote

all: $(STEPS)

dist: mal

mal: step6_file
	cp $< $@

%: %.rs
	cargo build --bin $*
	# cargo build --release --bin $*
	cp target/debug/$* $@
	# cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) reader.rs types.rs printer.rs env.rs
STEP2_DEPS = $(STEP1_DEPS)
STEP3_DEPS = $(STEP2_DEPS)
STEP4_DEPS = $(STEP3_DEPS) core.rs
STEP5_DEPS = $(STEP4_DEPS)
STEP6_DEPS = $(STEP5_DEPS)
STEP7_DEPS = $(STEP6_DEPS)

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP1_DEPS)
step2_eval: $(STEP2_DEPS)
step3_env: $(STEP3_DEPS)
step4_if_fn_do: $(STEP4_DEPS)
step5_tco: $(STEP5_DEPS)
step6_file: $(STEP6_DEPS)
step7_quote: $(STEP7_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
