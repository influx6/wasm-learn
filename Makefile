PWD:=$(shell pwd)
APP?=""
WASM?=${APP}
VERBOSITY=-vv

define build_book
endef

define inspect
endef


build:
	wat2wasm ${PWD}/${APP} -o ${PWD}/${WASM}.wasm

dump:
	wasm-objdump ${PWD}/${APP}.wasm -x

build-dump: build dump