bin:
	cargo build --example=spi_test --release

prog: bin
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/examples/spi_test test.hex
	python3 ~/projects/uf2/utils/uf2conv.py test.hex


bin-cube:
	cargo build --example=cube-controller --release

prog-cube: bin-cube
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/examples/cube-controller test.hex
	python3 ~/projects/uf2/utils/uf2conv.py test.hex