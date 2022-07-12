/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstruction.h"

#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


// NOLINTBEGIN(readability-magic-numbers)

void RabbitizerInstruction_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    switch (opcode) {
        // 0b000000: "SPECIAL",
        // 0b000001: "REGIMM",
        case 0b000010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_j;
            break;
        case 0b000011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_jal;
            break;
        case 0b000100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_beq;
            break;
        case 0b000101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bne;
            break;
        case 0b000110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_blez;
            break;
        case 0b000111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgtz;
            break;

        case 0b001000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_addi;
            break;
        case 0b001001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_addiu;
            break;
        case 0b001010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_slti;
            break;
        case 0b001011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sltiu;
            break;
        case 0b001100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_andi;
            break;
        case 0b001101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ori;
            break;
        case 0b001110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_xori;
            break;
        case 0b001111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lui;
            break;

        // 0b010000: "COP0", // Coprocessor OPeration z
        // 0b010001: "COP1", // Coprocessor OPeration z
        // 0b010010: "COP2", // Coprocessor OPeration z
        // 0b010011: "COP3", // Coprocessor OPeration z
        case 0b010100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_beql;
            break;
        case 0b010101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bnel;
            break;
        case 0b010110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_blezl;
            break;
        case 0b010111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgtzl;
            break;

        case 0b011000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddi;
            break;
        case 0b011001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddiu;
            break;
        case 0b011010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldl;
            break;
        case 0b011011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldr;
            break;

        case 0b100000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lb;
            break;
        case 0b100001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lh;
            break;
        case 0b100010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwl;
            break;
        case 0b100011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lw;
            break;
        case 0b100100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lbu;
            break;
        case 0b100101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lhu;
            break;
        case 0b100110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwr;
            break;
        case 0b100111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwu;
            break;

        case 0b101000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sb;
            break;
        case 0b101001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sh;
            break;
        case 0b101010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_swl;
            break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sw;
            break;
        case 0b101100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdl;
            break;
        case 0b101101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdr;
            break;
        case 0b101110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_swr;
            break;
        case 0b101111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_cache;
            break;

        case 0b110000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ll;
            break;
        case 0b110001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwc1;
            break;
        case 0b110010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwc2;
            break;
        case 0b110011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_pref;
            break;
        case 0b110100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lld;
            break;
        case 0b110101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldc1;
            break;
        case 0b110110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldc2;
            break;
        case 0b110111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ld;
            break;

        case 0b111000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sc;
            break;
        case 0b111001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_swc1;
            break;
        case 0b111010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_swc2;
            // 0b111011: "",
            break;
        case 0b111100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_scd;
            break;
        case 0b111101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdc1;
            break;
        case 0b111110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdc2;
            break;
        case 0b111111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sd;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
            break;
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
        case 0b000000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sll;
            break;
        // 0b000_001: "MOVCI", // TODO
        case 0b000010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_srl;
            break;
        case 0b000011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sra;
            break;
        case 0b000100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sllv;
            break;
        // 0b000_101: "",
        case 0b000110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_srlv;
            break;
        case 0b000111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_srav;
            break;

        case 0b001000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_jr;
            break;
        case 0b001001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_jalr;
            self->_mandatorybits = RAB_INSTR_PACK_rd(self->_mandatorybits, RAB_INSTR_GET_rd(self));
            break;
        case 0b001010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_movz;
            break;
        case 0b001011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_movn;
            break;
        case 0b001100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_syscall;
            break;
        case 0b001101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_break;
            break;
        // 0b001_110: "",
        case 0b001111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sync;
            break;

        case 0b010000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mfhi;
            break;
        case 0b010001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mthi;
            break;
        case 0b010010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mflo;
            break;
        case 0b010011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mtlo;
            break;
        case 0b010100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsllv;
            break;
        // 0b010_101: "",
        case 0b010110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrlv;
            break;
        case 0b010111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrav;
            break;

        case 0b011000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mult;
            break;
        case 0b011001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_multu;
            break;
        case 0b011010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_div;
            break;
        case 0b011011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_divu;
            break;
        case 0b011100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmult;
            break;
        case 0b011101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmultu;
            break;
        case 0b011110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ddiv;
            break;
        case 0b011111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ddivu;
            break;

        case 0b100000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_add;
            break;
        case 0b100001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_addu;
            break;
        case 0b100010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sub;
            break;
        case 0b100011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_subu;
            break;
        case 0b100100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_and;
            break;
        case 0b100101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_or;
            break;
        case 0b100110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_xor;
            break;
        case 0b100111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_nor;
            break;

        // 0b101_000: "",
        // 0b101_001: "",
        case 0b101010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_slt;
            break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sltu;
            break;
        case 0b101100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dadd;
            break;
        case 0b101101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddu;
            break;
        case 0b101110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsub;
            break;
        case 0b101111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsubu;
            break;

        case 0b110000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tge;
            break;
        case 0b110001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgeu;
            break;
        case 0b110010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlt;
            break;
        case 0b110011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tltu;
            break;
        case 0b110100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_teq;
            break;
        // 0b110_101: "",
        case 0b110110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tne;
            break;
            // 0b110_111: "",

        case 0b111000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsll;
            break;
        // 0b111_001: "",
        case 0b111010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrl;
            break;
        case 0b111011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsra;
            break;
        case 0b111100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsll32;
            break;
        // 0b111_101: "",
        case 0b111110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrl32;
            break;
        case 0b111111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsra32;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
            break;
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
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltz;
            break;
        case 0b00001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgez;
            break;
        case 0b00010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzl;
            break;
        case 0b00011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezl;
            break;

        case 0b01000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgei;
            break;
        case 0b01001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgeiu;
            break;
        case 0b01010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlti;
            break;
        case 0b01011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tltiu;
            break;

        case 0b10000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzal;
            break;
        case 0b10001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezal;
            break;
        case 0b10010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzall;
            break;
        case 0b10011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezall;
            break;

        case 0b01100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_teqi;
            break;
        case 0b01110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_tnei;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstruction_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);
    uint32_t tf;
    uint32_t nd;
    uint32_t function;

    self->_handwrittenCategory = true;
    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);

    switch (fmt) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mfc0;
            break;
        case 0b00001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmfc0;
            break;
        case 0b00010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_cfc0;
            break;
        // 0b00_011: "",
        case 0b00100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mtc0;
            break;
        case 0b00101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmtc0;
            break;
        case 0b00110:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_ctc0;
            break;
            // 0b00_111: "",

        case 0b01000:
            tf = RAB_INSTR_GET_tf(self);
            nd = RAB_INSTR_GET_nd(self);
            self->_mandatorybits = RAB_INSTR_PACK_tf(self->_mandatorybits, tf);
            self->_mandatorybits = RAB_INSTR_PACK_nd(self->_mandatorybits, nd);
            if (tf) {
                if (nd) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc0tl;
                } else {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc0t;
                }
            } else {
                if (nd) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc0fl;
                } else {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bc0f;
                }
            }
            break;

        default:
            function = RAB_INSTR_GET_function(self);
            self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
            switch (function) {
                case 0b000001:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlbr;
                    break;
                case 0b000010:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlbwi;
                    break;
                case 0b000110:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlbwr;
                    break;
                case 0b001000:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlbp;
                    break;
                case 0b011000:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_eret;
                    break;

                default:
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_INVALID;
                    break;
            }
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

void RabbitizerInstruction_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

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
