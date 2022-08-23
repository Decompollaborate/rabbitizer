/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...) \
        case (caseBits): \
            self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
            break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

// NOLINTBEGIN(readability-magic-numbers)

void RabbitizerInstruction_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    switch (opcode) {
#include "instructions/instr_id/cpu/cpu_normal.inc"
    }

    if (RabbitizerConfig_Cfg.pseudos.enablePseudos) {
        switch (self->uniqueId) {
            case RABBITIZER_INSTR_ID_cpu_beq:
                if (RAB_INSTR_GET_rt(self) == 0) {
                    if (RAB_INSTR_GET_rs(self) == 0) {
                        if (RabbitizerConfig_Cfg.pseudos.pseudoB) {
                            self->uniqueId = RABBITIZER_INSTR_ID_cpu_b;
                        }
                    } else {
                        if (RabbitizerConfig_Cfg.pseudos.pseudoBeqz) {
                            self->uniqueId = RABBITIZER_INSTR_ID_cpu_beqz;
                        }
                    }
                }
                break;

            case RABBITIZER_INSTR_ID_cpu_bne:
                if (RAB_INSTR_GET_rt(self) == 0) {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoBnez) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_bnez;
                    }
                }
                break;

            default:
                break;
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
#include "instructions/instr_id/cpu/cpu_special.inc"
    }

    if (RabbitizerInstruction_isNop(self)) {
        // NOP is special enough
        self->uniqueId = RABBITIZER_INSTR_ID_cpu_nop;
    } else if (RabbitizerConfig_Cfg.pseudos.enablePseudos) {
        switch (self->uniqueId) {
            case RABBITIZER_INSTR_ID_cpu_or:
                if (RAB_INSTR_GET_rt(self) == 0) {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoMove) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_move;
                    }
                }
                break;

            case RABBITIZER_INSTR_ID_cpu_nor:
                if (RAB_INSTR_GET_rt(self) == 0) {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoNot) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_not;
                    }
                }
                break;

            case RABBITIZER_INSTR_ID_cpu_subu:
                if (RAB_INSTR_GET_rs(self) == 0) {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoNegu) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_negu;
                    }
                }
                break;

            default:
                break;
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    switch (self->uniqueId) {
        case RABBITIZER_INSTR_ID_cpu_jalr:
            self->_mandatorybits = RAB_INSTR_PACK_rd(self->_mandatorybits, RAB_INSTR_GET_rd(self));

            if (RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_NUMERIC || RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_O32) {
                if (RAB_INSTR_GET_rd(self) != RABBITIZER_REG_GPR_O32_ra) {
                    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_jalr_rd];
                }
            } else {
                if (RAB_INSTR_GET_rd(self) != RABBITIZER_REG_GPR_N32_ra) {
                    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_jalr_rd];
                }
            }
            break;

        case RABBITIZER_INSTR_ID_cpu_div:
            if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix && !self->inHandwrittenFunction) {
                self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_sn64_div];
            }
            break;

        case RABBITIZER_INSTR_ID_cpu_divu:
            if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix && !self->inHandwrittenFunction) {
                self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_sn64_divu];
            }
            break;

        default:
            break;
    }
}

void RabbitizerInstruction_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);

    switch (rt) {
#include "instructions/instr_id/cpu/cpu_regimm.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Coprocessor0_BC0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_bc0_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_bc0_fmt(self->_mandatorybits, fmt);

    switch (fmt) {
        #include "instructions/instr_id/cpu/cpu_cop0_bc0.inc"
    }
}

void RabbitizerInstruction_processUniqueId_Coprocessor0_Tlb(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
        #include "instructions/instr_id/cpu/cpu_cop0_tlb.inc"
    }
}

void RabbitizerInstruction_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->_handwrittenCategory = true;

    switch (fmt) {
        #include "instructions/instr_id/cpu/cpu_cop0.inc"

        case 0x08:
            RabbitizerInstruction_processUniqueId_Coprocessor0_BC0(self);
            break;

        case 0x10:
            RabbitizerInstruction_processUniqueId_Coprocessor0_Tlb(self);
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    uint8_t fmt = RAB_INSTR_GET_fmt(self);
    uint8_t fc;
    uint32_t tf;
    uint32_t nd;
    uint32_t function;
    uint32_t cond;

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    // TODO

    switch (fmt) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mfc1;
            break;
        case 0b00001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmfc1;
            break;
        case 0b00010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_cfc1;
            break;

        case 0b00100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mtc1;
            break;
        case 0b00101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmtc1;
            break;
        case 0b00110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ctc1;
            break;

        case 0b01000: // fmt = BC
            tf = RAB_INSTR_GET_tf(self);
            nd = RAB_INSTR_GET_nd(self);
            self->_mandatorybits = RAB_INSTR_PACK_tf(self->_mandatorybits, tf);
            self->_mandatorybits = RAB_INSTR_PACK_nd(self->_mandatorybits, nd);
            if (tf) {
                if (nd) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc1tl;
                } else {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc1t;
                }
            } else {
                if (nd) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc1fl;
                } else {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc1f;
                }
            }
            break;

        default:
            fmt = fmt & 0x07;
            function = RAB_INSTR_GET_function(self);
            self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
            switch (function) {
                case 0b000000:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_add_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_add_d;
                    }
                    break;
                case 0b000001:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_sub_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_sub_d;
                    }
                    break;
                case 0b000010:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_mul_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_mul_d;
                    }
                    break;
                case 0b000011:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_div_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_div_d;
                    }
                    break;

                case 0b000100:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_sqrt_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_sqrt_d;
                    }
                    break;
                case 0b000101:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_abs_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_abs_d;
                    }
                    break;
                case 0b000110:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_mov_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_mov_d;
                    }
                    break;
                case 0b000111:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_neg_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_neg_d;
                    }
                    break;

                case 0b001000:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_round_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_round_l_d;
                    }
                    break;
                case 0b001001:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_trunc_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_trunc_l_d;
                    }
                    break;
                case 0b001010:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_ceil_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_ceil_l_d;
                    }
                    break;
                case 0b001011:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_floor_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_floor_l_d;
                    }
                    break;

                case 0b001100:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_round_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_round_w_d;
                    }
                    break;
                case 0b001101:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_trunc_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_trunc_w_d;
                    }
                    break;
                case 0b001110:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_ceil_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_ceil_w_d;
                    }
                    break;
                case 0b001111:
                    if (fmt == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_floor_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId = RABBITIZER_INSTR_ID_cpu_floor_w_d;
                    }
                    break;

                default:
                    fc = RAB_INSTR_GET_fc(self);
                    self->_mandatorybits = RAB_INSTR_PACK_fc(self->_mandatorybits, fc);
                    if (fc == 0b11) {
                        // Compare conditions codes
                        cond = RAB_INSTR_GET_cond(self);
                        self->_mandatorybits = RAB_INSTR_PACK_cond(self->_mandatorybits, cond);
                        switch (cond) {
                            case 0b0000:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_f_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_f_d;
                                }
                                break;
                            case 0b0001:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_un_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_un_d;
                                }
                                break;
                            case 0b0010:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_eq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_eq_d;
                                }
                                break;
                            case 0b0011:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ueq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ueq_d;
                                }
                                break;
                            case 0b0100:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_olt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_olt_d;
                                }
                                break;
                            case 0b0101:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ult_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ult_d;
                                }
                                break;
                            case 0b0110:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ole_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ole_d;
                                }
                                break;
                            case 0b0111:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ule_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ule_d;
                                }
                                break;

                            case 0b1000:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_sf_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_sf_d;
                                }
                                break;
                            case 0b1001:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngle_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngle_d;
                                }
                                break;
                            case 0b1010:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_seq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_seq_d;
                                }
                                break;
                            case 0b1011:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngl_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngl_d;
                                }
                                break;
                            case 0b1100:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_lt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_lt_d;
                                }
                                break;
                            case 0b1101:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_nge_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_nge_d;
                                }
                                break;
                            case 0b1110:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_le_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_le_d;
                                }
                                break;
                            case 0b1111:
                                if (fmt == 0) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_c_ngt_d;
                                }
                                break;
                        }

                    } else if (fc == 0b10) {
                        // Convert codes
                        switch (function & 0x07) {
                            case 0b000:
                                if (fmt == 0b001) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_s_d;
                                } else if (fmt == 0b100) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_s_w;
                                } else if (fmt == 0b101) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_s_l;
                                }
                                break;
                            case 0b001:
                                if (fmt == 0b000) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_d_s;
                                } else if (fmt == 0b100) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_d_w;
                                } else if (fmt == 0b101) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_d_l;
                                }
                                break;
                            case 0b100:
                                if (fmt == 0b000) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_w_s;
                                } else if (fmt == 0b001) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_w_d;
                                }
                                break;
                            case 0b101:
                                if (fmt == 0b000) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_l_s;
                                } else if (fmt == 0b001) {
                                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_cvt_l_d;
                                }
                                break;
                        }
                    }
                    break;
            }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    self->_handwrittenCategory = true;

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstruction_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstruction_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstruction_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstruction_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstruction_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstruction_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstruction_processUniqueId_Coprocessor2(self);
            break;
    }
}

// NOLINTEND(readability-magic-numbers)
