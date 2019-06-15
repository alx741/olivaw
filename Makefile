ELF = target/thumbv7m-none-eabi/debug
BINARY = olivaw

$(BINARY):
	cargo build

$(BINARY).bin: $(BINARY)
	arm-none-eabi-objcopy -O binary $(ELF)/$^ ./$@

burn: $(BINARY).bin
	st-flash write $< 0x8000000

interface:
	picocom --imap lfcrlf -b 9600 -c /dev/ttyUSB0

clean:
	rm -f $(BINARY).bin
