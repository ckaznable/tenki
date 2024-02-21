build:
	cargo build --release

install:
	./scripts/setup.sh

uninstall:
	./scripts/setup.sh uninstall

clean:
	cargo clean -q

all: build install clean
