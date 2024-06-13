install-risc0-toolchain:
	cargo risczero install --version v2024-04-22.0
	@echo "Risc0 toolchain version:"
	cargo +risc0 --version