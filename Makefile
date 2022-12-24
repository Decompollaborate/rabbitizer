# Build options can be changed by modifying the makefile or by building with 'make SETTING=value'.
DEBUG           ?= 0
WERROR          ?= 0
ASAN            ?= 0
EXPERIMENTAL    ?= 0
SANITY_CHECKS   ?= 1

CC              := clang
CXX             := clang++
AR              := ar
IINC            := -I include
IINC_XX         := -I include -I cplusplus/include
CSTD            := -std=c11
CXXSTD          := -std=c++17
CFLAGS          := -fPIC
CXXFLAGS        := -fPIC
LDFLAGS         := -Lbuild -lrabbitizer
LDXXFLAGS       := -Lbuild -lrabbitizerpp
WARNINGS        := -Wall -Wextra -Wpedantic
# WARNINGS        := -Wall -Wextra -Wpedantic -Wpadded
WARNINGS        += -Werror=vla -Werror=switch -Werror=implicit-fallthrough -Werror=unused-function -Werror=unused-parameter -Werror=shadow -Werror=switch
WARNINGS_C      := -Werror=implicit-function-declaration -Werror=incompatible-pointer-types
WARNINGS_CXX    :=

TABLE_GEN       := tools/table_gen.sh

ifeq ($(CC),gcc)
    WARNINGS    += -Wno-cast-function-type
endif

ifeq ($(DEBUG),0)
	OPTFLAGS    := -Os -g
else
	OPTFLAGS    := -O0 -g3
	CFLAGS      += -DDEVELOPMENT=1
	CXXFLAGS    += -DDEVELOPMENT=1
endif

ifneq ($(WERROR),0)
	WARNINGS    += -Werror
endif

ifneq ($(ASAN),0)
	CFLAGS      += -fsanitize=address -fsanitize=pointer-compare -fsanitize=pointer-subtract -fsanitize=undefined
	CXXFLAGS    += -fsanitize=address -fsanitize=pointer-compare -fsanitize=pointer-subtract -fsanitize=undefined
endif

ifneq ($(EXPERIMENTAL),0)
	CFLAGS      += -DEXPERIMENTAL
	CXXFLAGS    += -DEXPERIMENTAL
endif

ifneq ($(SANITY_CHECKS),0)
	CFLAGS      += -DRAB_SANITY_CHECKS=1
	CXXFLAGS    += -DRAB_SANITY_CHECKS=1
endif


SRC_DIRS        := $(shell find src -type d)
C_FILES         := $(foreach dir,$(SRC_DIRS),$(wildcard $(dir)/*.c))
H_FILES         := $(foreach dir,$(IINC),$(wildcard $(dir)/**/*.h))
O_FILES         := $(foreach f,$(C_FILES:.c=.o),build/$f)

SRCXX_DIRS      := $(shell find cplusplus/src -type d)
CXX_FILES       := $(foreach dir,$(SRCXX_DIRS),$(wildcard $(dir)/*.cpp))
HXX_FILES       := $(foreach dir,$(IINC_XX),$(wildcard $(dir)/**/*.hpp))
OXX_FILES       := $(foreach f,$(CXX_FILES:.cpp=.o),build/$f)

DEP_FILES       := $(O_FILES:%.o=%.d) $(OXX_FILES:%.o=%.d)

TABLE_DIRS      := $(shell find include src cplusplus rabbitizer -type d)
TABLE_TEMPLATES := $(foreach dir,$(TABLE_DIRS),$(wildcard $(dir)/*.table.template))
TABLE_GENERATED := $(TABLE_TEMPLATES:%.table.template=%.table.h)

TABLE_DEP_FILES := $(TABLE_GENERATED:%.table.h=%.table.d)

TESTS_DIRS      := $(shell find tests -type d)
TESTS_C         := $(foreach dir,$(TESTS_DIRS),$(wildcard $(dir)/*.c))
TESTS_CXX       := $(foreach dir,$(TESTS_DIRS),$(wildcard $(dir)/*.cpp))
TESTS_ELFS      := $(foreach f,$(TESTS_C:.c=.elf) $(TESTS_CXX:.cpp=.elf),build/$f)

STATIC_LIB      := build/librabbitizer.a
DYNAMIC_LIB     := build/librabbitizer.so

STATIC_LIB_XX   := build/librabbitizerpp.a
DYNAMIC_LIB_XX  := build/librabbitizerpp.so

# create build directories
$(shell mkdir -p $(foreach dir,$(SRC_DIRS) $(SRCXX_DIRS) $(TESTS_DIRS),build/$(dir)))


# Dependencies of libraries

$(STATIC_LIB):  $(O_FILES)
$(DYNAMIC_LIB): $(O_FILES)

$(STATIC_LIB_XX):  $(O_FILES) $(OXX_FILES)
$(DYNAMIC_LIB_XX): $(O_FILES) $(OXX_FILES)


#### Main Targets ###

all: static tests

static: $(STATIC_LIB) $(STATIC_LIB_XX)
dynamic: $(DYNAMIC_LIB) $(DYNAMIC_LIB_XX)

clean:
	$(RM) -rf build

distclean: clean
	$(RM) -rf dist *.egg-info .mypy_cache
	$(RM) -rf $(TABLE_GENERATED)
	$(RM) -rf target/

format:
	clang-format-11 -i -style=file $(C_FILES)
	clang-format-11 -i -style=file $(CXX_FILES)

tidy:
	clang-tidy-11 -p . --fix --fix-errors $(C_FILES) -- $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(WARNINGS_C) $(CFLAGS)

tests: $(TESTS_ELFS)

.PHONY: all clean distclean format tidy tests
.DEFAULT_GOAL := all
.SECONDARY:


#### Various Recipes ####

build/%.elf: %.c $(STATIC_LIB)
	$(CC) -MMD -MP $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(WARNINGS_C) $(CFLAGS) -o $@ $< $(LDFLAGS)

build/%.elf: %.cpp $(STATIC_LIB_XX)
	$(CXX) -MMD -MP $(CXXSTD) $(OPTFLAGS) $(IINC_XX) $(WARNINGS) $(WARNINGS_CXX) $(CXXFLAGS) -o $@ $< $(LDXXFLAGS)

build/%.a:
	$(AR) rcs $@ $^

build/%.so:
	$(CC) -shared -o $@ $^

build/%.o: %.c | $(TABLE_GENERATED)
#	The -MMD flags additionaly creates a .d file with the same name as the .o file.
	$(CC) -MMD -MP -c $(CSTD) $(OPTFLAGS) $(IINC) $(WARNINGS) $(WARNINGS_C) $(CFLAGS) -o $@ $<

build/%.o: %.cpp | $(TABLE_GENERATED)
#	The -MMD flags additionaly creates a .d file with the same name as the .o file.
	$(CXX) -MMD -MP -c $(CXXSTD) $(OPTFLAGS) $(IINC_XX) $(WARNINGS) $(WARNINGS_CXX) $(CXXFLAGS) -o $@ $<


%.table.h: %.table.template
	cpp -P $(IINC) -M -MM -MMD -MP -MT $@ -MF $(@:.table.h=.table.d) $<
	$(TABLE_GEN) $< $@ $(@F)


-include $(DEP_FILES)
-include $(TABLE_DEP_FILES)

# Print target for debugging
print-% : ; $(info $* is a $(flavor $*) variable set to [$($*)]) @true
