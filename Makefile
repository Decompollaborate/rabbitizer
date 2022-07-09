# Build options can be changed by modifying the makefile or by building with 'make SETTING=value'.
DEBUG           ?= 0
WERROR          ?= 0
ASAN            ?= 0
EXPERIMENTAL    ?= 0

CC              := clang
IINC            := -I include
CSTD            := -std=c11
CFLAGS          :=
LDFLAGS         :=
WARNINGS        := -Wall -Wextra
# WARNINGS        := -Wall -Wextra -Wpedantic # binary constants :s
WARNINGS        += -Wno-cast-function-type
WARNINGS        += -Werror=implicit-function-declaration -Werror=incompatible-pointer-types -Werror=vla -Werror=switch -Werror=implicit-fallthrough -Werror=unused-function -Werror=unused-parameter -Werror=shadow

ifeq ($(DEBUG),0)
	OPTFLAGS    := -O2 -g
else
	OPTFLAGS    := -O0 -g3
	CFLAGS      += -DDEVELOPMENT=1
endif

ifneq ($(WERROR),0)
	WARNINGS    += -Werror
endif

ifneq ($(ASAN),0)
	CFLAGS      += -fsanitize=address -fsanitize=pointer-compare -fsanitize=pointer-subtract -fsanitize=undefined
endif

ifneq ($(EXPERIMENTAL),0)
	CFLAGS      += -DEXPERIMENTAL
endif


SRC_DIRS    := $(shell find src -type d)
C_FILES     := $(foreach dir,$(SRC_DIRS),$(wildcard $(dir)/*.c))
H_FILES     := $(foreach dir,$(IINC),$(wildcard $(dir)/**/*.h))
O_FILES     := $(foreach f,$(C_FILES:.c=.o),build/$f)


all: tests

clean:
	$(RM) -rf build

distclean: clean
	$(RM) -rf dist rabbitizer.egg-info .mypy_cache

format:
	@echo "TODO"

tests: build/test.elf

.PHONY: all clean distclean format tests
.SECONDARY:


# create build directories
$(shell mkdir -p $(foreach dir,$(SRC_DIRS),build/$(dir)))

build/%.elf: %.c $(O_FILES)
	$(CC) $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(CFLAGS) $(LDFLAGS) -o $@ $^

build/%.o: %.c
	$(CC) -c $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(CFLAGS) -o $@ $<
