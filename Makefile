build:
	cargo build --release --frozen

install:
	cp target/release/tenki /usr/local/bin/
	chmod +x /usr/local/bin/tenki

uninstall:
	rm /usr/local/bin/tenki

clean:
	rm -rf target

all: build install clean
