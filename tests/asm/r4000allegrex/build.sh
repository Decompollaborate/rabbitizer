#!/bin/bash

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

set -e

COMPILER_PATH=$1

# compile
echo wine "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_instrs.s -o allegrex_instrs_pspsnc.o
wine "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_instrs.s -o allegrex_instrs_pspsnc.o

# objdump
echo wine ${COMPILER_PATH}/snbin.exe -d allegrex_instrs_pspsnc.o ">" allegrex_instrs_pspsnc.dump.s
wine ${COMPILER_PATH}/snbin.exe -d allegrex_instrs_pspsnc.o > allegrex_instrs_pspsnc.dump.s
dos2unix allegrex_instrs_pspsnc.dump.s


# compile
echo wine "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_vfpu.s -o allegrex_vfpu_pspsnc.o
wine "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_vfpu.s -o allegrex_vfpu_pspsnc.o

# objdump
echo wine ${COMPILER_PATH}/snbin.exe -d allegrex_vfpu_pspsnc.o ">" allegrex_vfpu_pspsnc.dump.s
wine ${COMPILER_PATH}/snbin.exe -d allegrex_vfpu_pspsnc.o > allegrex_vfpu_pspsnc.dump.s
dos2unix allegrex_vfpu_pspsnc.dump.s
