EXEC=tp1-rust
ARCH="avr-atmega324pa"

all *.rs:
	cargo +nightly build -r
	avr-objcopy -O ihex target/$(ARCH)/release/$(EXEC).elf $(EXEC).hex

clean:
	cargo clean
	rm $(EXEC).hex