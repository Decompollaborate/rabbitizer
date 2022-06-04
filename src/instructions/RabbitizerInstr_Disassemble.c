/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include <macros.h>


// TODO: move to RabbitizerInstrId.c?
const char *RabbitizerInstr_GetOpcodeName(RabbitizerInstr* self) {
    return RabbitizerInstrId_Names[self->uniqueId.cpuId];
}




typedef size_t (*OperandCallback)(RabbitizerInstr* self, char *dst, const char *immOverride, size_t immOverrideLength);


size_t RabbitizerRegisterType_ProcessLabel(RabbitizerInstr* self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    size_t tempSize;
    int aux;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }


    tempSize = strlen("func_");
    memcpy(dst, "func_", tempSize);
    dst += tempSize;
    totalSize += tempSize;

    aux = sprintf(dst, "%06X", RabbitizerInstr_GetInstrIndexAsVram(self));
    assert(aux > 0);
    dst += aux;
    totalSize += aux;

    return totalSize;
}



const OperandCallback instrOpercandCallbacks[] = {
    // [RABBITIZER_REGISTER_TYPE_rs] = ,
    // [RABBITIZER_REGISTER_TYPE_rt] = ,
    // [RABBITIZER_REGISTER_TYPE_rd] = ,
    // [RABBITIZER_REGISTER_TYPE_sa] = ,
    // [RABBITIZER_REGISTER_TYPE_ft] = ,
    // [RABBITIZER_REGISTER_TYPE_fs] = ,
    // [RABBITIZER_REGISTER_TYPE_fd] = ,
    // [RABBITIZER_REGISTER_TYPE_IMM] = ,
    [RABBITIZER_REGISTER_TYPE_LABEL] = RabbitizerRegisterType_ProcessLabel,
    // [RABBITIZER_REGISTER_TYPE_cop2t] = ,
    // [RABBITIZER_REGISTER_TYPE_cop0d] = ,
    // [RABBITIZER_REGISTER_TYPE_code] = ,
    // [RABBITIZER_REGISTER_TYPE_op] = ,
    // [RABBITIZER_REGISTER_TYPE_IMM_base] = ,
};


void RabbitizerInstr_DisassembleInstruction(RabbitizerInstr* self, char *dst, const char *immOverride, size_t immOverrideLength) {
    const char *opcodeName;
    size_t opcodeNameLength;

    opcodeName = RabbitizerInstr_GetOpcodeName(self);
    opcodeNameLength = strlen(opcodeName);

    memcpy(dst, opcodeName, opcodeNameLength);

    dst += opcodeNameLength;

    if (self->descriptor->operands[0] == RABBITIZER_REGISTER_TYPE_INVALID) {
        // There are no operands
        *dst = '\0';
        return;
    }

    // TODO: ljust
    // result = opcode.ljust(InstructionConfig.OPCODE_LJUST + self.extraLjustWidthOpcode, ' ') + " "

    *dst = ' ';
    dst++;

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_REGISTER_TYPE_INVALID; i++) {
        RabbitizerRegisterType operand;
        OperandCallback callback;
        size_t writtenBytes;

        if (i != 0) {
            *dst = ',';
            dst++;
            *dst = ' ';
            dst++;
        }

        operand = self->descriptor->operands[i];
        assert(operand > RABBITIZER_REGISTER_TYPE_INVALID);
        assert(operand < RABBITIZER_REGISTER_TYPE_MAX);

        callback = instrOpercandCallbacks[operand];
        assert(callback != NULL);

        writtenBytes = callback(self, dst, immOverride, immOverrideLength);
        dst += writtenBytes;
    }

    *dst = '\0';
}
