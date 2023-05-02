#!/bin/bash

# SPDX-FileCopyrightText: © 2023 Decompollaborate
# SPDX-License-Identifier: MIT

set -e

COMPILER_PATH=$1

# compile
echo "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_instrs.s -o allegrex_instrs_pspsnc.o
wine "${COMPILER_PATH}/pspsnc.exe" -c -O2 -g allegrex_instrs.s -o allegrex_instrs_pspsnc.o

# objdump
echo ${COMPILER_PATH}/snbin.exe -d allegrex_instrs_pspsnc.o ">" allegrex_instrs_pspsnc.dump.s
wine ${COMPILER_PATH}/snbin.exe -d allegrex_instrs_pspsnc.o > allegrex_instrs_pspsnc.dump.s
