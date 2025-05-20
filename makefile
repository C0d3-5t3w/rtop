# Default build target
.PHONY: all
all: build copy-assets

# Development build
.PHONY: dev
dev:
	cargo build

# Release build
.PHONY: build
build:
	cargo build --release

# Copy assets to the appropriate build directory
.PHONY: copy-assets
copy-assets:
	@echo "Copying assets..."
	@mkdir -p target/debug/pkg
	@mkdir -p target/release/pkg
	@cp -r pkg/* target/debug/pkg/ 2>/dev/null || :
	@cp -r pkg/* target/release/pkg/ 2>/dev/null || :
	@echo "Assets copied successfully!"

# Run the application
.PHONY: run
run: dev copy-assets
	cargo run

# Run the release version
.PHONY: run-release
run-release: build copy-assets
	./target/release/rtop

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf target/debug/pkg
	rm -rf target/release/pkg

# Install the application
.PHONY: install
install: build copy-assets
	@echo "Installing rtop..."
	@mkdir -p $(HOME)/.local/bin
	@cp target/release/rtop $(HOME)/.local/bin/
	@mkdir -p $(HOME)/.config/rtop
	@cp -r pkg/* $(HOME)/.config/rtop/ 2>/dev/null || :
	@echo "rtop installed to $(HOME)/.local/bin/rtop"
	@echo "Configuration files installed to $(HOME)/.config/rtop/"

# Help command
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make dev         - Build in debug mode"
	@echo "  make build       - Build in release mode"
	@echo "  make run         - Build in debug mode and run"
	@echo "  make run-release - Build in release mode and run"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make install     - Install rtop to ~/.local/bin"
	@echo "  make help        - Show this help message"
