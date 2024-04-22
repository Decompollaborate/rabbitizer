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

size_t RabbitizerOperandType_process_r4000allegrex_mp_vs_transpose(const RabbitizerInstruction *self, char *dst,
                                                                   UNUSED const char *immOverride,
                                                                   UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM2x2(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(self));

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

size_t RabbitizerOperandType_process_r4000allegrex_mt_vs_transpose(const RabbitizerInstruction *self, char *dst,
                                                                   UNUSED const char *immOverride,
                                                                   UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM3x3(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(self));

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

size_t RabbitizerOperandType_process_r4000allegrex_mq_vs_transpose(const RabbitizerInstruction *self, char *dst,
                                                                   UNUSED const char *immOverride,
                                                                   UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexM4x4(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(self));

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

size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond(const RabbitizerInstruction *self, char *dst,
                                                             UNUSED const char *immOverride,
                                                             UNUSED size_t immOverrideLength) {
    static const char *const condition_mnemonics[16] = {
        [0] = "fl",  // Always false
        [1] = "eq",  // Equal
        [2] = "lt",  // Less than
        [3] = "le",  // Less than or equal
        [4] = "tr",  // Always true
        [5] = "ne",  // Not equal
        [6] = "ge",  // Greater than or equal
        [7] = "gt",  // Greater than
        [8] = "ez",  // Equal to zero
        [9] = "en",  // Equal to NaN
        [10] = "ei", // Absolute value equal to infinity
        [11] = "es", // Equal to infinity or NaN
        [12] = "nz", // Not equal to zero
        [13] = "nn", // Not equal to NaN
        [14] = "ni", // Absolute value not equal to infinity
        [15] = "ns", // Not equal to infinity and not equal to NaN
    };
    size_t totalSize = 0;
    uint8_t cond = RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, condition_mnemonics[cond]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt(const RabbitizerInstruction *self,
                                                                                 char *dst, const char *immOverride,
                                                                                 size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t cond = RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self);
    uint8_t vs = RAB_INSTR_R4000ALLEGREX_GET_vs(self);
    uint8_t vt = RAB_INSTR_R4000ALLEGREX_GET_vt(self);

    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_vcmp_cond(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            // If the other operands are 0 then we can omit them

            if ((vs == 0) && (vt == 0)) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_s_vs(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            break;

        case 1: // eq
        case 2: // lt
        case 3: // le
        case 5: // ne
        case 6: // ge
        case 7: // gt
            break;

        case 8:  // ez
        case 9:  // en
        case 10: // ei
        case 11: // es
        case 12: // nz
        case 13: // nn
        case 14: // ni
        case 15: // ns
            // If the vt operands is 0 then we can omit it

            if (vt == 0) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_s_vt(self, dst, immOverride, immOverrideLength));

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt(const RabbitizerInstruction *self,
                                                                                 char *dst, const char *immOverride,
                                                                                 size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t cond = RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self);
    uint8_t vs = RAB_INSTR_R4000ALLEGREX_GET_vs(self);
    uint8_t vt = RAB_INSTR_R4000ALLEGREX_GET_vt(self);

    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_vcmp_cond(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            // If the other operands are 0 then we can omit them

            if ((vs == 0) && (vt == 0)) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_p_vs(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            break;

        case 1: // eq
        case 2: // lt
        case 3: // le
        case 5: // ne
        case 6: // ge
        case 7: // gt
            break;

        case 8:  // ez
        case 9:  // en
        case 10: // ei
        case 11: // es
        case 12: // nz
        case 13: // nn
        case 14: // ni
        case 15: // ns
            // If the vt operands is 0 then we can omit it

            if (vt == 0) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_p_vt(self, dst, immOverride, immOverrideLength));

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt(const RabbitizerInstruction *self,
                                                                                 char *dst, const char *immOverride,
                                                                                 size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t cond = RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self);
    uint8_t vs = RAB_INSTR_R4000ALLEGREX_GET_vs(self);
    uint8_t vt = RAB_INSTR_R4000ALLEGREX_GET_vt(self);

    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_vcmp_cond(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            // If the other operands are 0 then we can omit them

            if ((vs == 0) && (vt == 0)) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_t_vs(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            break;

        case 1: // eq
        case 2: // lt
        case 3: // le
        case 5: // ne
        case 6: // ge
        case 7: // gt
            break;

        case 8:  // ez
        case 9:  // en
        case 10: // ei
        case 11: // es
        case 12: // nz
        case 13: // nn
        case 14: // ni
        case 15: // ns
            // If the vt operands is 0 then we can omit it

            if (vt == 0) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_t_vt(self, dst, immOverride, immOverrideLength));

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt(const RabbitizerInstruction *self,
                                                                                 char *dst, const char *immOverride,
                                                                                 size_t immOverrideLength) {
    size_t totalSize = 0;
    uint8_t cond = RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self);
    uint8_t vs = RAB_INSTR_R4000ALLEGREX_GET_vs(self);
    uint8_t vt = RAB_INSTR_R4000ALLEGREX_GET_vt(self);

    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize,
        RabbitizerOperandType_process_r4000allegrex_vcmp_cond(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            // If the other operands are 0 then we can omit them

            if ((vs == 0) && (vt == 0)) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_q_vs(self, dst, immOverride, immOverrideLength));

    switch (cond) {
        case 0: // fl
        case 4: // tr
            break;

        case 1: // eq
        case 2: // lt
        case 3: // le
        case 5: // ne
        case 6: // ge
        case 7: // gt
            break;

        case 8:  // ez
        case 9:  // en
        case 10: // ei
        case 11: // es
        case 12: // nz
        case 13: // nn
        case 14: // ni
        case 15: // ns
            // If the vt operands is 0 then we can omit it

            if (vt == 0) {
                return totalSize;
            }
            break;
    }

    RABUTILS_BUFFER_CPY(dst, totalSize, ", ");
    RABUTILS_BUFFER_ADVANCE(
        dst, totalSize, RabbitizerOperandType_process_r4000allegrex_q_vt(self, dst, immOverride, immOverrideLength));

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_vconstant(const RabbitizerInstruction *self, char *dst,
                                                             UNUSED const char *immOverride,
                                                             UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    const char *reg = RabbitizerRegister_getNameR4000AllegrexVConstant(RAB_INSTR_R4000ALLEGREX_GET_vconstant(self));

    RABUTILS_BUFFER_CPY(dst, totalSize, reg);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_power_of_two(const RabbitizerInstruction *self, char *dst,
                                                                UNUSED const char *immOverride,
                                                                UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_power_of_two(self);

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

size_t RabbitizerOperandType_process_r4000allegrex_vfpu_cc_bit(const RabbitizerInstruction *self, char *dst,
                                                               UNUSED const char *immOverride,
                                                               UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_vfpu_cc_bit(self);

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

size_t RabbitizerOperandType_process_r4000allegrex_bn(const RabbitizerInstruction *self, char *dst,
                                                      UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_bn(self);

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

size_t RabbitizerOperandType_process_r4000allegrex_int16(const RabbitizerInstruction *self, char *dst,
                                                         UNUSED const char *immOverride,
                                                         UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    int32_t number = RabbitizerUtils_From2Complement(RAB_INSTR_R4000ALLEGREX_GET_intfloat16(self), 16);

#if 0
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
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", number);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_float16(const RabbitizerInstruction *self, char *dst,
                                                           UNUSED const char *immOverride,
                                                           UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    union {
        float f;
        uint32_t ui;
    } floatNumber;
    floatNumber.ui = RabbitizerUtils_floatRepr_32From16(RAB_INSTR_R4000ALLEGREX_GET_intfloat16(self));

    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%.10g", (double)floatNumber.f);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_p_vrot_code(const RabbitizerInstruction *self, char *dst,
                                                               UNUSED const char *immOverride,
                                                               UNUSED size_t immOverrideLength) {
    static const char *const mnemonics[32] = {
        [0] = "[C,S]",    //
        [1] = "[S,C]",    //
        [2] = "[S,0]",    //
        [3] = "[S,0]",    //
        [4] = "[C,S]",    //
        [5] = "[S,C]",    //
        [6] = "[0,S]",    //
        [7] = "[0,S]",    //
        [8] = "[C,0]",    //
        [9] = "[0,C]",    //
        [10] = "[S,S]",   //
        [11] = "[0,0]",   //
        [12] = "[C,0]",   //
        [13] = "[0,C]",   //
        [14] = "[0,0]",   //
        [15] = "[S,S]",   //
        [16] = "[C,-S]",  //
        [17] = "[-S,C]",  //
        [18] = "[-S,0]",  //
        [19] = "[-S,0]",  //
        [20] = "[C,-S]",  //
        [21] = "[-S,C]",  //
        [22] = "[0,-S]",  //
        [23] = "[0,-S]",  //
        [24] = "[C,0]",   //
        [25] = "[0,C]",   //
        [26] = "[-S,-S]", //
        [27] = "[0,0]",   //
        [28] = "[C,0]",   //
        [29] = "[0,C]",   //
        [30] = "[0,0]",   //
        [31] = "[-S,-S]", //
    };
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_vrot_code(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, mnemonics[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_t_vrot_code(const RabbitizerInstruction *self, char *dst,
                                                               UNUSED const char *immOverride,
                                                               UNUSED size_t immOverrideLength) {
    static const char *const mnemonics[32] = {
        [0] = "[C,S,S]",     //
        [1] = "[S,C,0]",     //
        [2] = "[S,0,C]",     //
        [3] = "[S,0,0]",     //
        [4] = "[C,S,0]",     //
        [5] = "[S,C,S]",     //
        [6] = "[0,S,C]",     //
        [7] = "[0,S,0]",     //
        [8] = "[C,0,S]",     //
        [9] = "[0,C,S]",     //
        [10] = "[S,S,C]",    //
        [11] = "[0,0,S]",    //
        [12] = "[C,0,0]",    //
        [13] = "[0,C,0]",    //
        [14] = "[0,0,C]",    //
        [15] = "[S,S,S]",    //
        [16] = "[C,-S,-S]",  //
        [17] = "[-S,C,0]",   //
        [18] = "[-S,0,C]",   //
        [19] = "[-S,0,0]",   //
        [20] = "[C,-S,0]",   //
        [21] = "[-S,C,-S]",  //
        [22] = "[0,-S,C]",   //
        [23] = "[0,-S,0]",   //
        [24] = "[C,0,-S]",   //
        [25] = "[0,C,-S]",   //
        [26] = "[-S,-S,C]",  //
        [27] = "[0,0,-S]",   //
        [28] = "[C,0,0]",    //
        [29] = "[0,C,0]",    //
        [30] = "[0,0,C]",    //
        [31] = "[-S,-S,-S]", //
    };
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_vrot_code(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, mnemonics[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_q_vrot_code(const RabbitizerInstruction *self, char *dst,
                                                               UNUSED const char *immOverride,
                                                               UNUSED size_t immOverrideLength) {
    static const char *const mnemonics[32] = {
        [0] = "[C,S,S,S]",     //
        [1] = "[S,C,0,0]",     //
        [2] = "[S,0,C,0]",     //
        [3] = "[S,0,0,C]",     //
        [4] = "[C,S,0,0]",     //
        [5] = "[S,C,S,S]",     //
        [6] = "[0,S,C,0]",     //
        [7] = "[0,S,0,C]",     //
        [8] = "[C,0,S,0]",     //
        [9] = "[0,C,S,0]",     //
        [10] = "[S,S,C,S]",    //
        [11] = "[0,0,S,C]",    //
        [12] = "[C,0,0,S]",    //
        [13] = "[0,C,0,S]",    //
        [14] = "[0,0,C,S]",    //
        [15] = "[S,S,S,C]",    //
        [16] = "[C,-S,-S,-S]", //
        [17] = "[-S,C,0,0]",   //
        [18] = "[-S,0,C,0]",   //
        [19] = "[-S,0,0,C]",   //
        [20] = "[C,-S,0,0]",   //
        [21] = "[-S,C,-S,-S]", //
        [22] = "[0,-S,C,0]",   //
        [23] = "[0,-S,0,C]",   //
        [24] = "[C,0,-S,0]",   //
        [25] = "[0,C,-S,0]",   //
        [26] = "[-S,-S,C,-S]", //
        [27] = "[0,0,-S,C]",   //
        [28] = "[C,0,0,-S]",   //
        [29] = "[0,C,0,-S]",   //
        [30] = "[0,0,C,-S]",   //
        [31] = "[-S,-S,-S,C]", //
    };
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_vrot_code(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, mnemonics[temp]);

    return totalSize;
}

static const char *const source_target_prefix_instruction_formats[32] = {
    [0] = "X",     //
    [1] = "Y",     //
    [2] = "Z",     //
    [3] = "W",     //
    [4] = "|X|",   //
    [5] = "|Y|",   //
    [6] = "|Z|",   //
    [7] = "|W|",   //
    [8] = "0",     //
    [9] = "1",     //
    [10] = "2",    //
    [11] = "1/2",  //
    [12] = "3",    //
    [13] = "1/3",  //
    [14] = "1/4",  //
    [15] = "1/6",  //
    [16] = "-X",   //
    [17] = "-Y",   //
    [18] = "-Z",   //
    [19] = "-W",   //
    [20] = "-|X|", //
    [21] = "-|Y|", //
    [22] = "-|Z|", //
    [23] = "-|W|", //
    [24] = "-0",   //
    [25] = "-1",   //
    [26] = "-2",   //
    [27] = "-1/2", //
    [28] = "-3",   //
    [29] = "-1/3", //
    [30] = "-1/4", //
    [31] = "-1/6", //
};

size_t RabbitizerOperandType_process_r4000allegrex_rpx(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_rpx(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, source_target_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_rpy(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_rpy(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, source_target_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_rpz(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_rpz(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, source_target_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_rpw(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_rpw(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, source_target_prefix_instruction_formats[temp]);

    return totalSize;
}

static const char *const destination_prefix_instruction_formats[8] = {
    [0] = "",          //
    [1] = "0",         //
    [2] = "INVALID_2", //
    [3] = "1",         //
    [4] = "M",         //
    [5] = "INVALID_5", //
    [6] = "INVALID_6", //
    [7] = "INVALID_7", //
};

size_t RabbitizerOperandType_process_r4000allegrex_wpx(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_wpx(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, destination_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_wpy(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_wpy(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, destination_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_wpz(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_wpz(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, destination_prefix_instruction_formats[temp]);

    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_wpw(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_wpw(self);

    RABUTILS_BUFFER_CPY(dst, totalSize, destination_prefix_instruction_formats[temp]);

    return totalSize;
}
