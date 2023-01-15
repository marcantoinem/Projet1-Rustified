PROJECTNAME=tp2-rust
ARCH=avr
MCU=atmega324pa
CARGOCOMMAND=cargo +nightly build -r --target $(ARCH)-$(MCU).json
HEXROM=$(PROJECTNAME).hex

all *.rs:
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROJECTNAME).elf $(HEXROM)
	
install: $(HEXROMTRG)
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROJECTNAME).elf $(HEXROM)
	avrdude -c usbasp -p $(MCU) -P -e -U flash:w:$(HEXROM)

clean:
	cargo clean
	rm $(HEXROM)