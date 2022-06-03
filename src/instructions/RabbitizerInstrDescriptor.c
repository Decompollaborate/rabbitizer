/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrDescriptor.h"

const RabbitizerInstrDescriptor RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_MAX] = {
    [RABBITIZER_INSTR_CPU_ID_INVALID] = {.operands={RABBITIZER_REGISTER_TYPE_rs, RABBITIZER_REGISTER_TYPE_rt, RABBITIZER_REGISTER_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN},
};
