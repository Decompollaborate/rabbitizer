# Build options can be changed by modifying the makefile or by building with 'make SETTING=value'.
DEBUG           ?= 0
WERROR          ?= 0
ASAN            ?= 0
EXPERIMENTAL    ?= 0

CC              := clang
AR              := ar
IINC            := -I include
CSTD            := -std=c11
CFLAGS          := -fPIC
LDFLAGS         := -Lbuild -lrabbitizer
WARNINGS        := -Wall -Wextra -Wpedantic
# WARNINGS        := -Wall -Wextra -Wpedantic -Wpadded
WARNINGS        += -Werror=implicit-function-declaration -Werror=incompatible-pointer-types -Werror=vla -Werror=switch -Werror=implicit-fallthrough -Werror=unused-function -Werror=unused-parameter -Werror=shadow

ifeq ($(CC),gcc)
    WARNINGS    += -Wno-cast-function-type
endif

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


SRC_DIRS        := $(shell find src -type d)
C_FILES         := $(foreach dir,$(SRC_DIRS),$(wildcard $(dir)/*.c))
H_FILES         := $(foreach dir,$(IINC),$(wildcard $(dir)/**/*.h))
O_FILES         := $(foreach f,$(C_FILES:.c=.o),build/$f)
DEP_FILES       := $(O_FILES:%.o=%.d)

STATIC_LIB      := build/librabbitizer.a
DYNAMIC_LIB     := build/librabbitizer.so

# create build directories
$(shell mkdir -p $(foreach dir,$(SRC_DIRS),build/$(dir)))


#### Main Targets ###

all: static tests

static: $(STATIC_LIB)
dynamic: $(DYNAMIC_LIB)

clean:
	$(RM) -rf build

distclean: clean
	$(RM) -rf dist rabbitizer.egg-info .mypy_cache

format:
	clang-format-11 -i -style=file $(C_FILES)

tidy:
	clang-tidy-11 -p . --fix --fix-errors $(C_FILES) $(H_FILES) -- $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(CFLAGS)

tests: build/test.elf build/rsptest.elf build/r5900test.elf build/registersTrackerTest.elf

.PHONY: all clean distclean format tidy tests
.DEFAULT_GOAL := all
.SECONDARY:


#### Various Recipes ####

build/%.elf: %.c | $(STATIC_LIB)
	$(CC) -MMD $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(CFLAGS) -o $@ $^ $(LDFLAGS)

build/%.a: $(O_FILES)
	$(AR) rcs $@ $^

build/%.so: $(O_FILES)
	$(CC) -shared -o $@ $^

build/%.o: %.c
#	The -MMD flags additionaly creates a .d file with the same name as the .o file.
	$(CC) -MMD -c $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(CFLAGS) -o $@ $<


-include $(DEP_FILES)
