program_name :=rbasename

prod ?=

ifdef prod
	release :=--release
	target :=release
else
	release :=
	target :=debug
endif


test:
	cargo test

build:
	cargo build $(release) 

install:
	cp target/$(target)/$(program_name) /usr/local/bin/$(program_name)

all: test build install

help:
	@echo "usage: $$ make all [prod=1]"
