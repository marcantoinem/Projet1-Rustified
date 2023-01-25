PROJECTNAME=tp3-rust
ARCH=avr
MCU=atmega324pa
CARGOEXEC=/home/$(USER)/.cargo/bin/cargo
HEXROM=$(PROJECTNAME).hex

.PHONY: all, build, clean, install

PROBLEMNUM := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(PROBLEMNUM):;@:)


CARGOCOMMAND=$(CARGOEXEC) +nightly build -r --target $(ARCH)-$(MCU).json --bin $(PROBLEMNUM)

all:
	@echo "Please use build + the name of the target, E.g: make build problem1"

build:
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROBLEMNUM).elf $(HEXROM)
	
install: $(HEXROMTRG)
	$(CARGOCOMMAND)
	avr-objcopy -O ihex target/$(ARCH)-$(MCU)/release/$(PROBLEMNUM).elf $(HEXROM)
	avrdude -c usbasp -p $(MCU) -P -e -U flash:w:$(HEXROM)

clean:
	$(CARGOEXEC) clean
	rm $(HEXROM)