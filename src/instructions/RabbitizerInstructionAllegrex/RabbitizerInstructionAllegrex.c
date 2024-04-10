/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionAllegrex.h"

void RabbitizerInstructionAllegrex_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram) {
    RabbitizerInstruction_init(self, word, vram);

    self->uniqueId = RABBITIZER_INSTR_ID_allegrex_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    self->category = RABBITIZER_INSTRCAT_ALLEGREX;
}

void RabbitizerInstructionAllegrex_destroy(RabbitizerInstruction *self) {
    RabbitizerInstruction_destroy(self);
}
