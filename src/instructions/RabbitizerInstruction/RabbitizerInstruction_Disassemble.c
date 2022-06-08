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


#define RABUTILS_BUFFER_WRITE_CHAR(buffer, totalSize, character) \
    do { \
        *(buffer) = (character); \
        (buffer)++; \
        (totalSize)++; \
    } while(0)

#define RABUTILS_BUFFER_SPRINTF(buffer, totalSize, format, ...) \
    do { \
        int len = sprintf(buffer, format, __VA_ARGS__);\
        assert(len > 0); \
        (buffer) += len; \
        (totalSize) += len; \
    } while(0)

#define RABUTILS_BUFFER_CPY(buffer, totalSize, string) \
    do { \
        size_t tempSize = strlen(string);\
        memcpy(buffer, string, tempSize); \
        (buffer) += tempSize; \
        (totalSize) += tempSize; \
    } while(0)

#define RABUTILS_BUFFER_ADVANCE(buffer, totalSize, expression) \
    do { \
        size_t tempSize = expression;\
        (buffer) += tempSize; \
        (totalSize) += tempSize; \
    } while(0)


typedef size_t (*OperandCallback)(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);


size_t RabbitizerOperandType_processRs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rs);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processRt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rt);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processRd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(self->rd);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop0d(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop0(self->rd);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstruction_getFs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstruction_getFt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processFd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1(RabbitizerInstruction_getFd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop1Cs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop1Control(RabbitizerInstruction_getFs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processCop2t(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameCop2(self->rt);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandType_processSa(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", self->sa);
    return totalSize;
}

size_t RabbitizerOperandType_processOp(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%02X", self->rt);
    return totalSize;
}

size_t RabbitizerOperandType_processCode(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    int code = (self->rs << 5) | (self->rt);
    int lower = (self->rd << 5) | (self->sa);

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

    RABUTILS_BUFFER_CPY(dst, totalSize, "func_");
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%06X", RabbitizerInstruction_getInstrIndexAsVram(self));
    return totalSize;
}

size_t RabbitizerOperandType_processImmediate(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t imm;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    imm = RabbitizerInstruction_getImmediate(self);
    if (!self->descriptor->isUnsigned) {
        int32_t number = RabbitizerUtils_From2Complement(imm, 16);

        if (number < 0) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "-0x%X", -number);
            return totalSize;
        }
    }

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", imm);
    return totalSize;
}

size_t RabbitizerOperandType_processImmediateBase(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processImmediate(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processRs(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspGpr(self->rs);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspGpr(self->rt);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processRd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(self->rd);

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processCop0d(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspCop0(self->rd);

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

    element = RabbitizerInstructionRsp_processVectorElement(self, RAB_INSTR_RSP_GET_ELEMENT_HIGH(self));
    if (element != 0) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '[');
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", element);
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ']');
    }
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVtElementlow(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t element;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVt(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '[');

    element = RabbitizerInstructionRsp_processVectorElement(self, RAB_INSTR_RSP_GET_ELEMENT_LOW(self));
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", element);

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ']');
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVdVs(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVd(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '[');
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", RAB_INSTR_RSP_GET_VS(self));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ']');
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVdIndex(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVd(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '[');
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", (self->sa >> 1));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ']');
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
    [RABBITIZER_OPERAND_TYPE_rs]                   = RabbitizerOperandType_processRs,
    [RABBITIZER_OPERAND_TYPE_rt]                   = RabbitizerOperandType_processRt,
    [RABBITIZER_OPERAND_TYPE_rd]                   = RabbitizerOperandType_processRd,
    [RABBITIZER_OPERAND_TYPE_cop0d]                = RabbitizerOperandType_processCop0d,
    [RABBITIZER_OPERAND_TYPE_fs]                   = RabbitizerOperandType_processFs,
    [RABBITIZER_OPERAND_TYPE_ft]                   = RabbitizerOperandType_processFt,
    [RABBITIZER_OPERAND_TYPE_fd]                   = RabbitizerOperandType_processFd,
    [RABBITIZER_OPERAND_TYPE_cop1cs]               = RabbitizerOperandType_processCop1Cs,
    [RABBITIZER_OPERAND_TYPE_cop2t]                = RabbitizerOperandType_processCop2t,
    [RABBITIZER_OPERAND_TYPE_sa]                   = RabbitizerOperandType_processSa,
    [RABBITIZER_OPERAND_TYPE_op]                   = RabbitizerOperandType_processOp,
    [RABBITIZER_OPERAND_TYPE_code]                 = RabbitizerOperandType_processCode,
    [RABBITIZER_OPERAND_TYPE_LABEL]                = RabbitizerOperandType_processLabel,
    [RABBITIZER_OPERAND_TYPE_IMM]                  = RabbitizerOperandType_processImmediate,
    [RABBITIZER_OPERAND_TYPE_IMM_base]             = RabbitizerOperandType_processImmediateBase,

    // rsp
    [RABBITIZER_OPERAND_TYPE_RSP_rs]               = RabbitizerOperandTypeRsp_processRs,
    [RABBITIZER_OPERAND_TYPE_RSP_rt]               = RabbitizerOperandTypeRsp_processRt,
    [RABBITIZER_OPERAND_TYPE_RSP_rd]               = RabbitizerOperandTypeRsp_processRd,
    [RABBITIZER_OPERAND_TYPE_RSP_cop0d]            = RabbitizerOperandTypeRsp_processCop0d,
    [RABBITIZER_OPERAND_TYPE_RSP_vs]               = RabbitizerOperandTypeRsp_processVs,
    [RABBITIZER_OPERAND_TYPE_RSP_vt]               = RabbitizerOperandTypeRsp_processVt,
    [RABBITIZER_OPERAND_TYPE_RSP_vd]               = RabbitizerOperandTypeRsp_processVd,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh]   = RabbitizerOperandTypeRsp_processVtElementhigh,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow]    = RabbitizerOperandTypeRsp_processVtElementlow,
    [RABBITIZER_OPERAND_TYPE_RSP_vd_vs]            = RabbitizerOperandTypeRsp_processVdVs,
    [RABBITIZER_OPERAND_TYPE_RSP_vd_index]         = RabbitizerOperandTypeRsp_processVdIndex,
    [RABBITIZER_OPERAND_TYPE_RSP_offset_rs]        = RabbitizerOperandTypeRsp_processOffsetVs,
};


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


size_t RabbitizerInstruction_disassembleInstruction(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
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

    if (self->descriptor->instrType == RABBITIZER_INSTR_TYPE_R) {
        bool hasCode = false;
        bool hasRs = false;
        bool hasRt = false;
        bool hasRd = false;
        bool hasSa = false;

        for (size_t i = 0; i < ARRAY_COUNT(self->descriptor->operands) && self->descriptor->operands[i] != RABBITIZER_OPERAND_TYPE_INVALID; i++) {
            RabbitizerOperandType operand = self->descriptor->operands[i];

            if (operand == RABBITIZER_OPERAND_TYPE_code) {
                hasCode = true;
            }
            if (operand == RABBITIZER_OPERAND_TYPE_rs) {
                hasRs = true;
            }
            if (operand == RABBITIZER_OPERAND_TYPE_rt) {
                hasRt = true;
            }
            if (operand == RABBITIZER_OPERAND_TYPE_rd) {
                hasRd = true;
            }
            if (operand == RABBITIZER_OPERAND_TYPE_sa) {
                hasSa = true;
            }
        }

        if (!hasCode) {
            if (!hasRs) {
                if (self->rs != 0) {
                    return true;
                }
            }
            if (!hasRt) {
                if (self->rt != 0) {
                    return true;
                }
            }
            if (!hasRd) {
                if (self->rd != 0 && self->uniqueId != RABBITIZER_INSTR_ID_cpu_jalr) {
                    return true;
                }
            }
            if (!hasSa) {
                if (self->sa != 0) {
                    return true;
                }
            }
        }
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

    return RabbitizerInstruction_getSizeForBufferInstrDisasm(self,immOverrideLength, extraLJust);
}


size_t RabbitizerInstruction_disassemble(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) {
    assert(dst != NULL);

    if (!RabbitizerInstruction_isImplemented(self) || RabbitizerInstruction_mustDisasmAsData(self)) {
        size_t totalSize = 0;

        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleAsData(self, dst, extraLJust));

        if (RabbitizerConfig_Cfg.misc.unknownInstrComment) {
            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, 40-totalSize, ' '));

            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '#');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust));
        }

        return totalSize;
    }

    return RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
}
