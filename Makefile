all: rip

clean:
	cargo clean

rip: src/main.rs
	cargo build --release

install: rip
	cp target/release/rip /usr/local/bin/

uninstall:
	rm /usr/local/bin/rip
