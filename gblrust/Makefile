#####################

SOURCES_BASE = src/types.rs src/readline.rs \
	       src/reader.rs \
	       src/lib.rs
SOURCES_LISP = src/bin/step2_eval.rs
SOURCES = $(SOURCES_BASE) $(SOURCES_LISP)

#####################

SRCS = step0_repl.rs step1_read_print.rs step2_eval.rs
BINS = $(SRCS:%.rs=target/release/%)

#####################

all: $(BINS)

dist: mal

mal: target/release/step2_eval
	cp $< $@

# TODO: would be nice to build just the step requested
$(BINS): target/release/%: src/bin/%.rs $(wildcard src/*.rs)
	cargo build --release

clean:
	cargo clean
	rm -f mal

.PHONY: stats stats-lisp mal

stats: $(SOURCES)
	@wc $^
	@printf "%5s %5s %5s %s\n" `grep -E "^[[:space:]]*//|^[[:space:]]*$$" $^ | wc` "[comments/blanks]"
stats-lisp: $(SOURCES_LISP)
	@wc $^
	@printf "%5s %5s %5s %s\n" `grep -E "^[[:space:]]*//|^[[:space:]]*$$" $^ | wc` "[comments/blanks]"
