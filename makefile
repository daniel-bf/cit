install:
	cargo build --release && sudo cp target/release/cit /usr/local/bin/cit

uninstall:
	sudo rm /usr/local/bin/cit

