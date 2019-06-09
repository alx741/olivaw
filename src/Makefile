STM32F1LIB_DIR = /home/alx/lab/stm32f1-minimal-lib/
STM32F1LIB_INCLUDE_DIR = $(STM32F1LIB_DIR)/include
STM32F1LIB_LINKER_DIR = $(STM32F1LIB_DIR)/linker

DEBUG = -DDEBUG

SRC_DIR = .
BINARY = main
LINKER_SCRIPT = stm32f103x8.ld
OBJECTS=$(foreach c_file, $(wildcard $(SRC_DIR)/*.c), $(c_file:.c=.o))

$(BINARY).bin: $(BINARY).elf
	arm-none-eabi-objcopy -O binary $^ $@

$(BINARY).elf: $(OBJECTS)
	arm-none-eabi-gcc -mcpu=cortex-m3 -mthumb -nostartfiles -Wl,--gc-sections,-static -L$(STM32F1LIB_DIR) -L$(STM32F1LIB_LINKER_DIR) -T$(STM32F1LIB_LINKER_DIR)/$(LINKER_SCRIPT) $^ -lstm32f1 -ggdb -lm -lc -lrdimon --specs=rdimon.specs -o $@

%.o: %.c
	arm-none-eabi-gcc $(DEBUG) -mcpu=cortex-m3 -mthumb -std=gnu11 -ggdb -lc -lrdimon --specs=rdimon.specs -O0 -c -I$(STM32F1LIB_INCLUDE_DIR) $^ -o $@

interface:
	picocom --imap lfcrlf -b 9600 -c /dev/ttyUSB0

ocd:
	openocd

burn: $(BINARY).bin
	st-flash write $< 0x8000000

clean:
	rm -f $(OBJECTS) $(BINARY).elf $(BINARY).bin
