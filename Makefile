PREFIX=/usr/local

all: release

release: target/release/vct

target/release/vct:
	cargo build --release

install: release
	mv target/release/vct $(PREFIX)/bin/

clean:
	rm -rf target/release
