PROJECTNAME=tp3-rust
ARCH=avr
MCU=atmega324pa
CARGOEXEC=/home/$(USER)/.cargo/bin/cargo
CARGOCOMMAND=$(CARGOEXEC) +nightly build -r --target $(ARCH)-$(MCU).json
HEXROM=$(PROJECTNAME).hex

.PHONY: all, clean

all:
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROJECTNAME).elf $(HEXROM)
	
install: $(HEXROMTRG)
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROJECTNAME).elf $(HEXROM)
	avrdude -c usbasp -p $(MCU) -P -e -U flash:w:$(HEXROM)

clean:
	$(CARGOEXEC) clean
	rm $(HEXROM)