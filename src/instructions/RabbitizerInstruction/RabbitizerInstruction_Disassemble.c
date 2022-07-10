/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"
#include "instructions/RabbitizerInstructionRsp.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"

typedef size_t (*OperandCallback)(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

size_t RabbitizerOperandType_processRs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(RAB_INSTR_GET_rs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processRt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(RAB_INSTR_GET_rt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processRd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(RAB_INSTR_GET_rd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop0d(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop0(RAB_INSTR_GET_cop0d(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RAB_INSTR_GET_fs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RAB_INSTR_GET_ft(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RAB_INSTR_GET_fd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop1Cs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1Control(RAB_INSTR_GET_cop1cs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop2t(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop2(RAB_INSTR_GET_cop2t(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processSa(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

// TODO: consider making this a proper configuration
#if 0
    if (RAB_INSTR_GET_sa(self) < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", RAB_INSTR_GET_sa(self));
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", RAB_INSTR_GET_sa(self));
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", RAB_INSTR_GET_sa(self));
    return totalSize;
}

size_t RabbitizerOperandType_processOp(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

// TODO: consider making this a proper configuration
#if 0
    if (RAB_INSTR_GET_op(self) < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", RAB_INSTR_GET_op(self));
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", RAB_INSTR_GET_op(self));
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%02X", RAB_INSTR_GET_op(self));
    return totalSize;
}

size_t RabbitizerOperandType_processCode(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    int code = RAB_INSTR_GET_code_upper(self);
    int lower = RAB_INSTR_GET_code_lower(self);

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", code);
    if (lower) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, ", %i", lower);
    }

    return totalSize;
}

size_t RabbitizerOperandType_processLabel(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "func_%06X", RabbitizerInstruction_getInstrIndexAsVram(self));
    return totalSize;
}

size_t RabbitizerOperandType_processImmediate(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    int32_t number;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    number = RabbitizerInstruction_getProcessedImmediate(self);
    if (RabbitizerConfig_Cfg.misc.omit0XOnSmallImm) {
        if (number > -10 && number < 10) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", number);
            return totalSize;
        }
    }
    if (number < 0) {
        if (RabbitizerConfig_Cfg.misc.upperCaseImm) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "-0x%X", -number);
        } else {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "-0x%x", -number);
        }
    } else {
        if (RabbitizerConfig_Cfg.misc.upperCaseImm) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", number);
        } else {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", number);
        }
    }
    return totalSize;
}

size_t RabbitizerOperandType_processImmediateBase(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

// TODO: consider making this a proper configuration
#if 0
    if (immOverride != NULL || RAB_INSTR_GET_immediate(self) != 0) {
        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processImmediate(self, dst, immOverride, immOverrideLength));
    }
#endif
    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processImmediate(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processRs(self, dst, immOverride, immOverrideLength));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspGpr(RAB_INSTR_GET_rs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspGpr(RAB_INSTR_GET_rt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_GET_rd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processCop0d(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspCop0(RAB_INSTR_GET_cop0d(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_VS(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_VT(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_VD(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVtElementhigh(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t element;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVt(self, dst, immOverride, immOverrideLength));

    element = RabbitizerInstructionRsp_processVectorElement(self, RAB_INSTR_RSP_GET_elementhigh(self));
    if (element != 0) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", element);
    }
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVtElementlow(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t element;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVt(self, dst, immOverride, immOverrideLength));

    element = RabbitizerInstructionRsp_processVectorElement(self, RAB_INSTR_RSP_GET_elementlow(self));
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", element);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVdVs(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVd(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", RAB_INSTR_RSP_GET_VS(self));
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVdIndex(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVd(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", RAB_INSTR_RSP_GET_index(self));
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processOffsetVs(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", RabbitizerInstructionRsp_GetOffsetVector(self));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processRs(self, dst, immOverride, immOverrideLength));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');
    return totalSize;
}

const OperandCallback instrOpercandCallbacks[] = {
    [RABBITIZER_OPERAND_TYPE_rs] = RabbitizerOperandType_processRs,
    [RABBITIZER_OPERAND_TYPE_rt] = RabbitizerOperandType_processRt,
    [RABBITIZER_OPERAND_TYPE_rd] = RabbitizerOperandType_processRd,
    [RABBITIZER_OPERAND_TYPE_cop0d] = RabbitizerOperandType_processCop0d,
    [RABBITIZER_OPERAND_TYPE_fs] = RabbitizerOperandType_processFs,
    [RABBITIZER_OPERAND_TYPE_ft] = RabbitizerOperandType_processFt,
    [RABBITIZER_OPERAND_TYPE_fd] = RabbitizerOperandType_processFd,
    [RABBITIZER_OPERAND_TYPE_cop1cs] = RabbitizerOperandType_processCop1Cs,
    [RABBITIZER_OPERAND_TYPE_cop2t] = RabbitizerOperandType_processCop2t,
    [RABBITIZER_OPERAND_TYPE_sa] = RabbitizerOperandType_processSa,
    [RABBITIZER_OPERAND_TYPE_op] = RabbitizerOperandType_processOp,
    [RABBITIZER_OPERAND_TYPE_code] = RabbitizerOperandType_processCode,
    [RABBITIZER_OPERAND_TYPE_LABEL] = RabbitizerOperandType_processLabel,
    [RABBITIZER_OPERAND_TYPE_IMM] = RabbitizerOperandType_processImmediate,
    [RABBITIZER_OPERAND_TYPE_IMM_base] = RabbitizerOperandType_processImmediateBase,

    // rsp
    [RABBITIZER_OPERAND_TYPE_RSP_rs] = RabbitizerOperandTypeRsp_processRs,
    [RABBITIZER_OPERAND_TYPE_RSP_rt] = RabbitizerOperandTypeRsp_processRt,
    [RABBITIZER_OPERAND_TYPE_RSP_rd] = RabbitizerOperandTypeRsp_processRd,
    [RABBITIZER_OPERAND_TYPE_RSP_cop0d] = RabbitizerOperandTypeRsp_processCop0d,
    [RABBITIZER_OPERAND_TYPE_RSP_vs] = RabbitizerOperandTypeRsp_processVs,
    [RABBITIZER_OPERAND_TYPE_RSP_vt] = RabbitizerOperandTypeRsp_processVt,
    [RABBITIZER_OPERAND_TYPE_RSP_vd] = RabbitizerOperandTypeRsp_processVd,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh] = RabbitizerOperandTypeRsp_processVtElementhigh,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow] = RabbitizerOperandTypeRsp_processVtElementlow,
    [RABBITIZER_OPERAND_TYPE_RSP_vd_vs] = RabbitizerOperandTypeRsp_processVdVs,
    [RABBITIZER_OPERAND_TYPE_RSP_vd_index] = RabbitizerOperandTypeRsp_processVdIndex,
    [RABBITIZER_OPERAND_TYPE_RSP_offset_rs] = RabbitizerOperandTypeRsp_processOffsetVs,
};

size_t RabbitizerInstruction_getSizeForBufferOperandsDisasm(const RabbitizerInstruction *self, size_t immOverrideLength) {
    size_t totalSize = 0;

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
        if (i != 0) {
            totalSize += 2;
        }

        // A bit arbitrary, but no operand should be longer than 25 characters
        totalSize += 25;
        totalSize += immOverrideLength;
    }

    return totalSize;
}

size_t RabbitizerInstruction_disassembleOperands(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
        RabbitizerOperandType operand;
        OperandCallback callback;

        if (i != 0) {
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ',');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
        }

        operand = self->descriptor->operands[i];
        assert(operand > RABBITIZER_OPERAND_TYPE_INVALID);
        assert(operand < RABBITIZER_OPERAND_TYPE_MAX);

        callback = instrOpercandCallbacks[operand];
        assert(callback != NULL);

        RABUTILS_BUFFER_ADVANCE(dst, totalSize, callback(self, dst, immOverride, immOverrideLength));
    }

    *dst = '\0';
    return totalSize;
}

size_t RabbitizerInstruction_getSizeForBufferInstrDisasm(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust) {
    size_t totalSize = 0;
    size_t opcodeNameLength;

    opcodeNameLength = strlen(RabbitizerInstrId_getOpcodeName(self->uniqueId));

    totalSize += opcodeNameLength;

    if (self->descriptor->operands[0] == RABBITIZER_OPERAND_TYPE_INVALID) {
        // There are no operands
        return totalSize;
    }

    totalSize += extraLJust;
    totalSize++;

    totalSize += RabbitizerInstruction_getSizeForBufferOperandsDisasm(self, immOverrideLength);

    return totalSize;
}

size_t RabbitizerInstruction_disassembleInstruction(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength,
                                                    int extraLJust) {
    size_t totalSize = 0;
    const char *opcodeName = RabbitizerInstrId_getOpcodeName(self->uniqueId);

    RABUTILS_BUFFER_CPY(dst, totalSize, opcodeName);

    if (self->descriptor->operands[0] == RABBITIZER_OPERAND_TYPE_INVALID) {
        // There are no operands
        *dst = '\0';
        return totalSize;
    }

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust - totalSize, ' '));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleOperands(self, dst, immOverride, immOverrideLength));

    *dst = '\0';
    return totalSize;
}

size_t RabbitizerInstruction_getSizeForBufferDataDisasm(UNUSED const RabbitizerInstruction *self, int extraLJust) {
    size_t totalSize = 0;

    totalSize += strlen(".word");
    totalSize += RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust;
    totalSize += 11;
    return totalSize;
}

size_t RabbitizerInstruction_disassembleAsData(const RabbitizerInstruction *self, char *dst, int extraLJust) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, ".word");

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, RabbitizerConfig_Cfg.misc.opcodeLJust + extraLJust - totalSize, ' '));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, " 0x%08X", RabbitizerInstruction_getRaw(self));
    return totalSize;
}

bool RabbitizerInstruction_mustDisasmAsData(const RabbitizerInstruction *self) {
    if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix) {
        if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_break) {
            return true;
        }
    }

    if (!RabbitizerInstruction_isValid(self)) {
        return true;
    }
    return false;
}

size_t RabbitizerInstruction_getSizeForBuffer(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust) {
    if (!RabbitizerInstruction_isImplemented(self) || RabbitizerInstruction_mustDisasmAsData(self)) {
        size_t totalSize = RabbitizerInstruction_getSizeForBufferDataDisasm(self, extraLJust);

        if (RabbitizerConfig_Cfg.misc.unknownInstrComment) {
            totalSize += 40;
            totalSize += 3;
            totalSize += RabbitizerInstruction_getSizeForBufferInstrDisasm(self, immOverrideLength, extraLJust);
        }
        return totalSize;
    }

    return RabbitizerInstruction_getSizeForBufferInstrDisasm(self, immOverrideLength, extraLJust);
}

size_t RabbitizerInstruction_disassemble(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    assert(dst != NULL);

    if (!RabbitizerInstruction_isImplemented(self) || RabbitizerInstruction_mustDisasmAsData(self)) {
        size_t totalSize = 0;

        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleAsData(self, dst, extraLJust));

        if (RabbitizerConfig_Cfg.misc.unknownInstrComment) {
            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, 40 - totalSize, ' '));

            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '#');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust));
        }

        return totalSize;
    }

    return RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
}
