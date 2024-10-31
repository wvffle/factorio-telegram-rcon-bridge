build:
	cargo build --release
	upx --best --lzma target/release/cracktorio-bot
