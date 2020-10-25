export PATH := $(PATH):/usr/local/bin
# CARGO_FLAGS =
CARGO_FLAGS = --release

all: run

run:
	env cargo build $(CARGO_FLAGS)

clean:
	rm -rf target
