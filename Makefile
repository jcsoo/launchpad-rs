.PHONY: build

build:
	xargo build --target thumbv7em-none-eabihf --example launchpad_minimal
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/examples/launchpad_minimal target/thumbv7em-none-eabihf/debug/examples/launchpad_minimal.bin

release:
	xargo build --target thumbv7em-none-eabihf --release

load:
	sudo lm4flash target/thumbv7em-none-eabihf/debug/examples/launchpad_minimal.bin

openocd:
	sudo openocd -f /usr/local/share/openocd/scripts/board/ek-lm4f120xl.cfg

trace:
	@itmdump /tmp/itm.fifo

debug:
	arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/examples/launchpad_minimal