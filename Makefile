# https://tech.davis-hansson.com/p/make/
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

CARGO := $(shell command -v cargo 2> /dev/null)
ifndef CARGO
  $(error Cargo is not installed. Please visit `https://rustup.rs/` and follow their instructions, or try to run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
endif

DESTDIR ?=
PREFIX  ?= /usr/local

# pull target directory from cargo, could be different from 'target' if configured is build.target-dir
JQ := $(shell command -v jq 2> /dev/null)
ifdef JQ
  target := $(shell cargo metadata --no-deps --offline --format-version 1 | jq -r '.target_directory')
else
  target := target
endif

APP := macstats


# generate release build
all: $(target)/release/$(APP)
build: $(target)/release/$(APP)

# install release build to local cargo bin directory
install: $(DESTDIR)$(PREFIX)/bin/$(APP)

# clean build output
clean:
> cargo clean

# Remove installed binary
uninstall:
> -rm -f -- "$(DESTDIR)$(PREFIX)/bin/$(APP)"

.PHONY: all build clean install uninstall

### build targets

$(target)/debug/$(APP): Cargo.toml Cargo.lock $(shell find src -type f)
> cargo build --bin $(APP)

$(target)/release/$(APP): Cargo.toml Cargo.lock $(shell find src -type f)
> RUSTFLAGS="-C link-arg=-s -C opt-level=2 -C target-cpu=native --emit=asm" cargo build --bin $(APP) --release

$(DESTDIR)$(PREFIX)/bin/$(APP): $(target)/release/$(APP)
> install -m755 -- $(target)/release/$(APP) "$(DESTDIR)$(PREFIX)/bin/"
