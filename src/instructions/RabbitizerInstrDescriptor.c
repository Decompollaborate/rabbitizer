/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrDescriptor.h"

#include "instructions/RabbitizerInstr.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = { __VA_ARGS__ }

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = { __VA_ARGS__ }


const RabbitizerInstrDescriptor RabbitizerInstrDescriptor_Descriptors[] = {
    #include "instructions/RabbitizerInstrId_cpu.inc"
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


bool RabbitizerInstrDescriptor_IsJType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_J;
}
bool RabbitizerInstrDescriptor_IsIType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_I;
}
bool RabbitizerInstrDescriptor_IsRType(const RabbitizerInstrDescriptor *self) {
    return self->instrType == RABBITIZER_INSTR_TYPE_R;
}

bool RabbitizerInstrDescriptor_IsBranch(const RabbitizerInstrDescriptor *self) {
    return self->isBranch;
}
bool RabbitizerInstrDescriptor_IsBranchLikely(const RabbitizerInstrDescriptor *self) {
    return self->isBranchLikely;
}
bool RabbitizerInstrDescriptor_IsJump(const RabbitizerInstrDescriptor *self) {
    return self->isJump;
}
bool RabbitizerInstrDescriptor_IsTrap(const RabbitizerInstrDescriptor *self) {
    return self->isTrap;
}

bool RabbitizerInstrDescriptor_IsFloat(const RabbitizerInstrDescriptor *self) {
    return self->isFloat;
}
bool RabbitizerInstrDescriptor_IsDouble(const RabbitizerInstrDescriptor *self) {
    return self->isDouble;
}

bool RabbitizerInstrDescriptor_IsUnsigned(const RabbitizerInstrDescriptor *self) {
    return self->isUnsigned;
}

bool RabbitizerInstrDescriptor_ModifiesRt(const RabbitizerInstrDescriptor *self) {
    return self->modifiesRt;
}
bool RabbitizerInstrDescriptor_ModifiesRd(const RabbitizerInstrDescriptor *self) {
    return self->modifiesRd;
}
