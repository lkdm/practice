init:
	@echo "Initializing submodules"
	cargo install trunk
	rustup toolchain install nightly
	rustup override set nightly
	rustup target add wasm32-unknown-unknown

css:
	tailwindcss -i ./src/style.css -o ./tailwind.css --watch

run:
	@echo "Running the app"
	cargo clippy --fix & cargo fmt
	trunk serve --port 3000 --open

build-release:
	@echo "Building the app for release"
	trunk build --release
