build:
	cargo build --release --frozen

install:
	cp target/release/tenki /usr/local/bin/
	chmod +x /usr/local/bin/tenki

uninstall:
	rm /usr/local/bin/tenki

clean:
	cargo clean -q

all: build install clean
