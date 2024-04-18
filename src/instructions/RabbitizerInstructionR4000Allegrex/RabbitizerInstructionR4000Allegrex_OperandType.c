/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR4000Allegrex.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"

size_t RabbitizerOperandType_process_r4000allegrex_s_vs(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexS(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_s_vt(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexS(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_s_vd(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexS(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_s_vt_imm(const RabbitizerInstruction *self, char *dst,
                                                            UNUSED const char *immOverride,
                                                            UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexS(RAB_INSTR_R4000ALLEGREX_GET_vt_imm(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_s_vd_imm(const RabbitizerInstruction *self, char *dst,
                                                            UNUSED const char *immOverride,
                                                            UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexS(RAB_INSTR_R4000ALLEGREX_GET_vd_imm(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_p_vs(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV2D(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_p_vt(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV2D(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_p_vd(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV2D(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_t_vs(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV3D(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_t_vt(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV3D(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_t_vd(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV3D(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_q_vs(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV4D(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_q_vt(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV4D(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_q_vd(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV4D(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_q_vt_imm(const RabbitizerInstruction *self, char *dst,
                                                            UNUSED const char *immOverride,
                                                            UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexV4D(RAB_INSTR_R4000ALLEGREX_GET_vt_6_imm(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mp_vs(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM2x2(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mp_vt(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM2x2(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mp_vd(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM2x2(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mt_vs(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM3x3(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mt_vt(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM3x3(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mt_vd(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM3x3(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mq_vs(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM4x4(RAB_INSTR_R4000ALLEGREX_GET_vs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mq_vt(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM4x4(RAB_INSTR_R4000ALLEGREX_GET_vt(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_mq_vd(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM4x4(RAB_INSTR_R4000ALLEGREX_GET_vd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_cop2cs(const RabbitizerInstruction *self, char *dst,
                                                              UNUSED const char *immOverride,
                                                              UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexVfpuControl(RAB_INSTR_R4000ALLEGREX_GET_cop2cs(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_cop2cd(const RabbitizerInstruction *self, char *dst,
                                                              UNUSED const char *immOverride,
                                                              UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexVfpuControl(RAB_INSTR_R4000ALLEGREX_GET_cop2cd(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_pos(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_pos(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_size(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_size(self) + 1;

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_size_plus_pos(const RabbitizerInstruction *self, char *dst,
                                                                 UNUSED const char *immOverride,
                                                                 UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_size_plus_pos(self) + 1 - RAB_INSTR_R4000ALLEGREX_GET_pos(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_imm3(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_imm3(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_offset14(const RabbitizerInstruction *self, char *dst,
                                                            const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;
    int32_t number;

    if ((dst == NULL) && (immOverrideLength > 0)) {
        return immOverrideLength;
    }

    if ((immOverride != NULL) && (immOverrideLength > 0)) {
        memcpy(dst, immOverride, immOverrideLength);
        return immOverrideLength;
    }

    number = RabbitizerUtils_From2Complement(RAB_INSTR_R4000ALLEGREX_GET_offset14(self) << 2, 16);
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

size_t RabbitizerOperandType_process_r4000allegrex_offset14_base(const RabbitizerInstruction *self, char *dst,
                                                                 const char *immOverride, size_t immOverrideLength) {
    size_t totalSize = 0;

// TODO: consider making this a proper configuration
#if 0
    if ((immOverride != NULL) && (immOverrideLength > 0) || RAB_INSTR_GET_immediate(self) != 0) {
        RABUTILS_BUFFER_ADVANCE(dst, totalSize, RabbitizerOperandType_process_r4000allegrex_offset14(self, dst, immOverride, immOverrideLength));
    }
#endif
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_offset14(self, dst, immOverride, immOverrideLength));

    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, '(');
    RABUTILS_BUFFER_ADVANCE(dst, totalSize,
                            RabbitizerOperandType_process_cpu_rs(self, dst, immOverride, immOverrideLength));
    RABUTILS_BUFFER_WRITE_CHAR(dst, totalSize, ')');

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_offset14_base_maybe_wb(const RabbitizerInstruction *self, char *dst,
                                                                          const char *immOverride,
                                                                          size_t immOverrideLength) {
    size_t totalSize = 0;

    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_offset14_base(self, dst, immOverride, immOverrideLength));

    if (RAB_INSTR_R4000ALLEGREX_GET_wb(self)) {
        RABUTILS_BUFFER_CPY(dst, totalSize, ", wb");
    }

    return totalSize;
}
