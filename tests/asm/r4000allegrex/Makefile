# SPDX-FileCopyrightText: © 2024-2025 Decompollaborate
# SPDX-License-Identifier: MIT

TARGETS := allegrex_instrs_pspsnc.dump.s allegrex_vfpu_pspsnc.dump.s allegrex_vfpu_expecteds_pspsnc.dump.s


COMPILER_PATH   ?=
WINE            ?= wine

ifeq ($(COMPILER_PATH),)
$(error Please set 'COMPILER_PATH' to the path where the SN toolchain is located)
endif

CC          := $(WINE) "$(COMPILER_PATH)/pspsnc.exe"
OBJDUMP     := $(WINE) "${COMPILER_PATH}/snbin.exe"

COMP_FLAGS  := -O2 -g


all: $(TARGETS)


%_pspsnc.dump.s: %.s
	$(CC) -c $(COMP_FLAGS) -o $(<:.s=_pspsnc.o) $<
	$(OBJDUMP) -d $(<:.s=_pspsnc.o) > $@
	dos2unix $@

