VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")
RUST_VERSION = $(word 2, $(shell rustc -V))
LONG_VERSION = "$(VERSION) ( rev: $(GIT_REVISION), rustc: $(RUST_VERSION), build at: $(BUILD_TIME) )"
BIN_NAME = tjuptatt

export LONG_VERSION

.PHONY: all test clean release_lnx release_win release_mac

all: test

test:
	cargo test --locked --workspace

clean:
	cargo clean

release_lnx:
	cargo build --locked --release --target=x86_64-unknown-linux-musl
	zip -j ${BIN_NAME}-v${VERSION}-x86_64-linux.zip target/x86_64-unknown-linux-musl/release/${BIN_NAME} LICENSE CHANGELOG.md README.md

release_win:
	cargo build --locked --release --target=x86_64-pc-windows-msvc
	mv -v target/x86_64-pc-windows-msvc/release/${BIN_NAME}.exe ./
	7z a ${BIN_NAME}-v${VERSION}-x86_64-windows.zip ${BIN_NAME}.exe LICENSE CHANGELOG.md README.md

release_mac:
	cargo build --locked --release --target=x86_64-apple-darwin
	zip -j ${BIN_NAME}-v${VERSION}-x86_64-mac.zip target/x86_64-apple-darwin/release/${BIN_NAME} LICENSE CHANGELOG.md README.md
