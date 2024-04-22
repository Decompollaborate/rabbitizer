/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR4000Allegrex.h"

void RabbitizerInstructionR4000Allegrex_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram) {
    RabbitizerInstruction_init(self, word, vram);

    self->uniqueId = RABBITIZER_INSTR_ID_r4000allegrex_INVALID;
    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    self->category = RABBITIZER_INSTRCAT_R4000ALLEGREX;
}

void RabbitizerInstructionR4000Allegrex_destroy(RabbitizerInstruction *self) {
    RabbitizerInstruction_destroy(self);
}
