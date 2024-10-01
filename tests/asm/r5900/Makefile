CROSS   := mips-linux-gnu-
AS      := $(CROSS)as
OJBDUMP := $(CROSS)objdump

%.o: %.s
	$(AS) -march=r5900 -mabi=eabi -o $@ $^
	$(OJBDUMP) -dr $@
