/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerInstructionR5900.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"
#include "instructions/RabbitizerInstrSuffix.h"

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

size_t RabbitizerOperandType_processZero(UNUSED const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameGpr(0);

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

size_t RabbitizerOperandType_processCopraw(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%X", RAB_INSTR_GET_copraw(self));

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
    const char *reg = RabbitizerRegister_getNameRspGpr(RAB_INSTR_GET_rd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processCop0d(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspCop0(RAB_INSTR_GET_cop0d(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processCop2t(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspCop2(RAB_INSTR_RSP_GET_cop2t(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processcop2cd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspCop2Control(RAB_INSTR_RSP_GET_cop2cd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVt(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameRspVector(RAB_INSTR_RSP_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVtElementhigh(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t element;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVt(self, dst, immOverride, immOverrideLength));

    element = RAB_INSTR_RSP_GET_elementhigh(self);
    if (element != 0) {
        if ((element & 0x8) == 0x8) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", element & 7);
        } else if ((element & 0xC) == 0x4) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%ih]", element & (~0xC));
        } else if ((element & 0xE) == 0x2) {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%iq]", element & (~0xE));
        } else {
            RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", element);
        }
    }
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVtElementlow(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVt(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", RAB_INSTR_RSP_GET_elementlow(self));
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVdDe(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t de;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVd(self, dst, immOverride, immOverrideLength));

    de = RAB_INSTR_RSP_GET_de(self);
    if ((de & 0x8) == 0x8) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", de & (~0x8));
    } else if ((de & 0xC) == 0x4) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%ih]", de & (~0xC));
    } else if ((de & 0xE) == 0x2) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%iq]", de & (~0xE));
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "[%i]", de);
    }
    return totalSize;
}

size_t RabbitizerOperandTypeRsp_processVsIndex(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processVs(self, dst, immOverride, immOverrideLength));

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

size_t RabbitizerOperandTypeRsp_processImmediateBase(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

// TODO: consider making this a proper configuration
#if 0
    if (immOverride != NULL || RAB_INSTR_GET_immediate(self) != 0) {
        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processImmediate(self, dst, immOverride, immOverrideLength));
    }
#endif
    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_processImmediate(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeRsp_processRs(self, dst, immOverride, immOverrideLength));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processI(UNUSED const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride,
                                           UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, "$I");

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processQ(UNUSED const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride,
                                           UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, "$Q");

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processR(UNUSED const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride,
                                           UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, "$R");

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processACC(UNUSED const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride,
                                             UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_CPY(dst, totalSize, "$ACC");

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processACCxyzw(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processACC(self, dst, immOverride, immOverrideLength));

#if 0
    if (RAB_INSTR_R5900_GET_xyzw_x(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'x');
    }
    if (RAB_INSTR_R5900_GET_xyzw_y(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'y');
    }
    if (RAB_INSTR_R5900_GET_xyzw_z(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'z');
    }
    if (RAB_INSTR_R5900_GET_xyzw_w(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'w');
    }
#endif

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfs(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VF(RAB_INSTR_R5900_GET_vfs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVft(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VF(RAB_INSTR_R5900_GET_vft(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfd(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VF(RAB_INSTR_R5900_GET_vfd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfsxyzw(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfs(self, dst, immOverride, immOverrideLength));

#if 0
    if (RAB_INSTR_R5900_GET_xyzw_x(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'x');
    }
    if (RAB_INSTR_R5900_GET_xyzw_y(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'y');
    }
    if (RAB_INSTR_R5900_GET_xyzw_z(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'z');
    }
    if (RAB_INSTR_R5900_GET_xyzw_w(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'w');
    }
#endif

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVftxyzw(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVft(self, dst, immOverride, immOverrideLength));

#if 0
    if (RAB_INSTR_R5900_GET_xyzw_x(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'x');
    }
    if (RAB_INSTR_R5900_GET_xyzw_y(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'y');
    }
    if (RAB_INSTR_R5900_GET_xyzw_z(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'z');
    }
    if (RAB_INSTR_R5900_GET_xyzw_w(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'w');
    }
#endif

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfdxyzw(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfd(self, dst, immOverride, immOverrideLength));

#if 0
    if (RAB_INSTR_R5900_GET_xyzw_x(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'x');
    }
    if (RAB_INSTR_R5900_GET_xyzw_y(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'y');
    }
    if (RAB_INSTR_R5900_GET_xyzw_z(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'z');
    }
    if (RAB_INSTR_R5900_GET_xyzw_w(self)) {
        RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, 'w');
    }
#endif

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfsn(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfs(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_n(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVftn(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVft(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_n(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfdn(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfd(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_n(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfsl(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfs(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_l(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVftl(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVft(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_l(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfdl(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfd(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_l(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfsm(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfs(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_m(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVftm(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVft(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_m(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVfdm(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t n;

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVfd(self, dst, immOverride, immOverrideLength));

    n = RAB_INSTR_R5900_GET_m(self);
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, "xyzw"[n]);

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVis(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VI(RAB_INSTR_R5900_GET_vis(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVit(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VI(RAB_INSTR_R5900_GET_vit(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVid(const RabbitizerInstruction *self, char *dst, UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR5900VI(RAB_INSTR_R5900_GET_vid(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);
    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVis_predecr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVis(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVit_predecr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVit(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVid_predecr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '-');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVid(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVis_postincr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVis(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVit_postincr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVit(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processVid_postincr(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandTypeR5900_processVid(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '+');
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandTypeR5900_processImm5(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    int32_t number;

    if (immOverride != NULL) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    number = RAB_INSTR_R5900_GET_imm5(self);
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

const OperandCallback instrOpercandCallbacks[] = {
    [RABBITIZER_OPERAND_TYPE_rs] = RabbitizerOperandType_processRs,
    [RABBITIZER_OPERAND_TYPE_rt] = RabbitizerOperandType_processRt,
    [RABBITIZER_OPERAND_TYPE_rd] = RabbitizerOperandType_processRd,
    [RABBITIZER_OPERAND_TYPE_zero] = RabbitizerOperandType_processZero,
    [RABBITIZER_OPERAND_TYPE_cop0d] = RabbitizerOperandType_processCop0d,
    [RABBITIZER_OPERAND_TYPE_fs] = RabbitizerOperandType_processFs,
    [RABBITIZER_OPERAND_TYPE_ft] = RabbitizerOperandType_processFt,
    [RABBITIZER_OPERAND_TYPE_fd] = RabbitizerOperandType_processFd,
    [RABBITIZER_OPERAND_TYPE_cop1cs] = RabbitizerOperandType_processCop1Cs,
    [RABBITIZER_OPERAND_TYPE_cop2t] = RabbitizerOperandType_processCop2t,
    [RABBITIZER_OPERAND_TYPE_sa] = RabbitizerOperandType_processSa,
    [RABBITIZER_OPERAND_TYPE_op] = RabbitizerOperandType_processOp,
    [RABBITIZER_OPERAND_TYPE_code] = RabbitizerOperandType_processCode,
    [RABBITIZER_OPERAND_TYPE_copraw] = RabbitizerOperandType_processCopraw,
    [RABBITIZER_OPERAND_TYPE_LABEL] = RabbitizerOperandType_processLabel,
    [RABBITIZER_OPERAND_TYPE_IMM] = RabbitizerOperandType_processImmediate,
    [RABBITIZER_OPERAND_TYPE_IMM_base] = RabbitizerOperandType_processImmediateBase,

    // rsp
    [RABBITIZER_OPERAND_TYPE_RSP_rs] = RabbitizerOperandTypeRsp_processRs,
    [RABBITIZER_OPERAND_TYPE_RSP_rt] = RabbitizerOperandTypeRsp_processRt,
    [RABBITIZER_OPERAND_TYPE_RSP_rd] = RabbitizerOperandTypeRsp_processRd,
    [RABBITIZER_OPERAND_TYPE_RSP_cop0d] = RabbitizerOperandTypeRsp_processCop0d,
    [RABBITIZER_OPERAND_TYPE_RSP_cop2t] = RabbitizerOperandTypeRsp_processCop2t,
    [RABBITIZER_OPERAND_TYPE_RSP_cop2cd] = RabbitizerOperandTypeRsp_processcop2cd,
    [RABBITIZER_OPERAND_TYPE_RSP_vs] = RabbitizerOperandTypeRsp_processVs,
    [RABBITIZER_OPERAND_TYPE_RSP_vt] = RabbitizerOperandTypeRsp_processVt,
    [RABBITIZER_OPERAND_TYPE_RSP_vd] = RabbitizerOperandTypeRsp_processVd,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh] = RabbitizerOperandTypeRsp_processVtElementhigh,
    [RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow] = RabbitizerOperandTypeRsp_processVtElementlow,
    [RABBITIZER_OPERAND_TYPE_RSP_vd_de] = RabbitizerOperandTypeRsp_processVdDe,
    [RABBITIZER_OPERAND_TYPE_RSP_vs_index] = RabbitizerOperandTypeRsp_processVsIndex,
    [RABBITIZER_OPERAND_TYPE_RSP_offset_rs] = RabbitizerOperandTypeRsp_processOffsetVs,
    [RABBITIZER_OPERAND_TYPE_RSP_IMM_base] = RabbitizerOperandTypeRsp_processImmediateBase,

    // r5900
    [RABBITIZER_OPERAND_TYPE_R5900_I] = RabbitizerOperandTypeR5900_processI,
    [RABBITIZER_OPERAND_TYPE_R5900_Q] = RabbitizerOperandTypeR5900_processQ,
    [RABBITIZER_OPERAND_TYPE_R5900_R] = RabbitizerOperandTypeR5900_processR,
    [RABBITIZER_OPERAND_TYPE_R5900_ACC] = RabbitizerOperandTypeR5900_processACC,
    [RABBITIZER_OPERAND_TYPE_R5900_ACCxyzw] = RabbitizerOperandTypeR5900_processACCxyzw,
    [RABBITIZER_OPERAND_TYPE_R5900_vfs] = RabbitizerOperandTypeR5900_processVfs,
    [RABBITIZER_OPERAND_TYPE_R5900_vft] = RabbitizerOperandTypeR5900_processVft,
    [RABBITIZER_OPERAND_TYPE_R5900_vfd] = RabbitizerOperandTypeR5900_processVfd,
    [RABBITIZER_OPERAND_TYPE_R5900_vfsxyzw] = RabbitizerOperandTypeR5900_processVfsxyzw,
    [RABBITIZER_OPERAND_TYPE_R5900_vftxyzw] = RabbitizerOperandTypeR5900_processVftxyzw,
    [RABBITIZER_OPERAND_TYPE_R5900_vfdxyzw] = RabbitizerOperandTypeR5900_processVfdxyzw,
    [RABBITIZER_OPERAND_TYPE_R5900_vfsn] = RabbitizerOperandTypeR5900_processVfsn,
    [RABBITIZER_OPERAND_TYPE_R5900_vftn] = RabbitizerOperandTypeR5900_processVftn,
    [RABBITIZER_OPERAND_TYPE_R5900_vfdn] = RabbitizerOperandTypeR5900_processVfdn,
    [RABBITIZER_OPERAND_TYPE_R5900_vfsl] = RabbitizerOperandTypeR5900_processVfsl,
    [RABBITIZER_OPERAND_TYPE_R5900_vftl] = RabbitizerOperandTypeR5900_processVftl,
    [RABBITIZER_OPERAND_TYPE_R5900_vfdl] = RabbitizerOperandTypeR5900_processVfdl,
    [RABBITIZER_OPERAND_TYPE_R5900_vfsm] = RabbitizerOperandTypeR5900_processVfsm,
    [RABBITIZER_OPERAND_TYPE_R5900_vftm] = RabbitizerOperandTypeR5900_processVftm,
    [RABBITIZER_OPERAND_TYPE_R5900_vfdm] = RabbitizerOperandTypeR5900_processVfdm,
    [RABBITIZER_OPERAND_TYPE_R5900_vis] = RabbitizerOperandTypeR5900_processVis,
    [RABBITIZER_OPERAND_TYPE_R5900_vit] = RabbitizerOperandTypeR5900_processVit,
    [RABBITIZER_OPERAND_TYPE_R5900_vid] = RabbitizerOperandTypeR5900_processVid,
    [RABBITIZER_OPERAND_TYPE_R5900_vis_predecr] = RabbitizerOperandTypeR5900_processVis_predecr,
    [RABBITIZER_OPERAND_TYPE_R5900_vit_predecr] = RabbitizerOperandTypeR5900_processVit_predecr,
    [RABBITIZER_OPERAND_TYPE_R5900_vid_predecr] = RabbitizerOperandTypeR5900_processVid_predecr,
    [RABBITIZER_OPERAND_TYPE_R5900_vis_postincr] = RabbitizerOperandTypeR5900_processVis_postincr,
    [RABBITIZER_OPERAND_TYPE_R5900_vit_postincr] = RabbitizerOperandTypeR5900_processVit_postincr,
    [RABBITIZER_OPERAND_TYPE_R5900_vid_postincr] = RabbitizerOperandTypeR5900_processVid_postincr,
    [RABBITIZER_OPERAND_TYPE_R5900_imm5] = RabbitizerOperandTypeR5900_processImm5,
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

    totalSize += RabbitizerInstrSuffix_getSizeForBuffer(self, self->descriptor->instrSuffix);

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

    RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstrSuffix_processSuffix(self, dst, self->descriptor->instrSuffix));

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
            totalSize += 11;
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
            uint32_t validBits;

            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerUtils_CharFill(dst, 40 - totalSize, ' '));

            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '#');
            RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ' ');

            RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust));

            validBits = RabbitizerInstruction_getValidBits(self);

            RABUTILS_BUFFER_SPRINTF(dst, totalSize, " # %08X", ((~validBits) & self->word));
        }

        return totalSize;
    }

    return RabbitizerInstruction_disassembleInstruction(self, dst, immOverride, immOverrideLength, extraLJust);
}
