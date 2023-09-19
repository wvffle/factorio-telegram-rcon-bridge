build:
	cargo build --release
	upx --best --lzma target/release/factorio-telegram-rcon-bridge
