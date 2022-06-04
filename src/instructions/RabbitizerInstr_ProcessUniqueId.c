/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"


void RabbitizerInstr_ProcessUniqueId_Normal(RabbitizerInstr *self) {
    switch (self->opcode) {
        // 0b000000: "SPECIAL",
        // 0b000001: "REGIMM",
        case 0b000010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_j;
            break;
        case 0b000011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_jal;
            break;
        case 0b000100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_beq;
            break;
        case 0b000101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bne;
            break;
        case 0b000110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_blez;
            break;
        case 0b000111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgtz;
            break;

        case 0b001000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_addi;
            break;
        case 0b001001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_addiu;
            break;
        case 0b001010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_slti;
            break;
        case 0b001011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sltiu;
            break;
        case 0b001100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_andi;
            break;
        case 0b001101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ori;
            break;
        case 0b001110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_xori;
            break;
        case 0b001111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lui;
            break;

        // 0b010000: "COP0", // Coprocessor OPeration z
        // 0b010001: "COP1", // Coprocessor OPeration z
        // 0b010010: "COP2", // Coprocessor OPeration z
        // 0b010011: "COP3", // Coprocessor OPeration z
        case 0b010100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_beql;
            break;
        case 0b010101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bnel;
            break;
        case 0b010110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_blezl;
            break;
        case 0b010111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgtzl;

            break;
        case 0b011000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_daddi;
            break;
        case 0b011001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_daddiu;
            break;
        case 0b011010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldl;
            break;
        case 0b011011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldr;
            break;

        case 0b100000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lb;
            break;
        case 0b100001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lh;
            break;
        case 0b100010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwl;
            break;
        case 0b100011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lw;
            break;
        case 0b100100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lbu;
            break;
        case 0b100101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lhu;
            break;
        case 0b100110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwr;
            break;
        case 0b100111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwu;
            break;

        case 0b101000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sb;
            break;
        case 0b101001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sh;
            break;
        case 0b101010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swl;
            break;
        case 0b101011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sw;
            break;
        case 0b101100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdl;
            break;
        case 0b101101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdr;
            break;
        case 0b101110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swr;
            break;
        case 0b101111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cache;
            break;

        case 0b110000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ll;
            break;
        case 0b110001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwc1;
            break;
        case 0b110010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwc2;
            break;
        case 0b110011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_pref;
            break;
        case 0b110100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lld;
            break;
        case 0b110101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldc1;
            break;
        case 0b110110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldc2;
            break;
        case 0b110111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ld;
            break;

        case 0b111000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sc;
            break;
        case 0b111001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swc1;
            break;
        case 0b111010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swc2;
        // 0b111011: "",
            break;
        case 0b111100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_scd;
            break;
        case 0b111101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdc1;
            break;
        case 0b111110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdc2;
            break;
        case 0b111111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sd;
            break;

        default:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;
            break;
    }

    /*if (InstructionConfig.PSEUDO_INSTRUCTIONS) */ {
        if (self->rt == 0) {
            if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_beq) {
                if (self->rs == 0) {
                    /* if (InstructionConfig.PSEUDO_B) */ {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_b;
                    }
                } else {
                    /*if (InstructionConfig.PSEUDO_BEQZ) */ {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_beqz;
                    }
                }
            } else if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_bne) {
                /* if InstructionConfig.PSEUDO_BNEZ */ {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bnez;
                }
            }
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];
}


void RabbitizerInstr_ProcessUniqueId_Special(RabbitizerInstr *self) {
    switch (self->function) {
        case 0b000000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sll;
            break;
        // 0b000_001: "MOVCI", // TODO
        case 0b000010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_srl;
            break;
        case 0b000011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sra;
            break;
        case 0b000100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sllv;
            break;
        // 0b000_101: "",
        case 0b000110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_srlv;
            break;
        case 0b000111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_srav;
            break;

        case 0b001000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_jr;
            break;
        case 0b001001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_jalr;
            break;
        case 0b001010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_movz;
            break;
        case 0b001011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_movn;
            break;
        case 0b001100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_syscall;
            break;
        case 0b001101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_break;
            break;
        // 0b001_110: "",
        case 0b001111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sync;
            break;

        case 0b010000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mfhi;
            break;
        case 0b010001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mthi;
            break;
        case 0b010010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mflo;
            break;
        case 0b010011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mtlo;
            break;
        case 0b010100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsllv;
            break;
        // 0b010_101: "",
        case 0b010110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsrlv;
            break;
        case 0b010111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsrav;
            break;

        case 0b011000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mult;
            break;
        case 0b011001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_multu;
            break;
        case 0b011010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_div;
            break;
        case 0b011011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_divu;
            break;
        case 0b011100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmult;
            break;
        case 0b011101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmultu;
            break;
        case 0b011110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ddiv;
            break;
        case 0b011111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ddivu;
            break;

        case 0b100000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_add;
            break;
        case 0b100001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_addu;
            break;
        case 0b100010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sub;
            break;
        case 0b100011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_subu;
            break;
        case 0b100100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_and;
            break;
        case 0b100101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_or;
            break;
        case 0b100110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_xor;
            break;
        case 0b100111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_nor;
            break;

        // 0b101_000: "",
        // 0b101_001: "",
        case 0b101010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_slt;
            break;
        case 0b101011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sltu;
            break;
        case 0b101100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dadd;
            break;
        case 0b101101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_daddu;
            break;
        case 0b101110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsub;
            break;
        case 0b101111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsubu;
            break;

        case 0b110000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tge;
            break;
        case 0b110001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tgeu;
            break;
        case 0b110010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlt;
            break;
        case 0b110011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tltu;
            break;
        case 0b110100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_teq;
            break;
        // 0b110_101: "",
        case 0b110110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tne;
            break;
        // 0b110_111: "",

        case 0b111000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsll;
            break;
        // 0b111_001: "",
        case 0b111010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsrl;
            break;
        case 0b111011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsra;
            break;
        case 0b111100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsll32;
            break;
        // 0b111_101: "",
        case 0b111110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsrl32;
            break;
        case 0b111111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dsra32;
            break;

        default:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;
            break;
    }

    if (RabbitizerInstr_IsNop(self)) {
        // NOP is special enough
        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_nop;
    } else /*if (InstructionConfig.PSEUDO_INSTRUCTIONS)*/ {
        if (self->rt == 0) {
            if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_or) {
                /*if (InstructionConfig.PSEUDO_MOVE)*/ {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_move;
                }
            } else if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_nor) {
                /*if (InstructionConfig.PSEUDO_NOT)*/ {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_not;
                }
            }
        } else if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_subu) {
            if (self->rs == 0) {
                /*if (InstructionConfig.PSEUDO_NEGU)*/ {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_negu;
                }
            }
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];

    if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_jalr) {
        // $ra
        if (self->rd != 31) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_CPU_ID_jalr_rd];
        }
    } else if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_div) {
        if (/*InstructionConfig.SN64_DIV_FIX */ false && !self->inHandwrittenFunction) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_CPU_ID_sn64_div];
        }
    } else if (self->uniqueId.cpuId == RABBITIZER_INSTR_CPU_ID_divu) {
        if (/*InstructionConfig.SN64_DIV_FIX */ false && !self->inHandwrittenFunction) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_CPU_ID_sn64_divu];
        }
    }
}


void RabbitizerInstr_ProcessUniqueId_Regimm(RabbitizerInstr *self) {
    switch (self->opcode) {
        case 0b00000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bltz;
            break;
        case 0b00001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgez;
            break;
        case 0b00010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bltzl;
            break;
        case 0b00011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgezl;
            break;

        case 0b01000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tgei;
            break;
        case 0b01001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tgeiu;
            break;
        case 0b01010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlti;
            break;
        case 0b01011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tltiu;
            break;

        case 0b10000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bltzal;
            break;
        case 0b10001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgezal;
            break;
        case 0b10010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bltzall;
            break;
        case 0b10011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgezall;
            break;

        case 0b01100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_teqi;
            break;
        case 0b01110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tnei;
            break;

        default:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];
}


void RabbitizerInstr_ProcessUniqueId_Coprocessor0(RabbitizerInstr *self) {
    self->_handwrittenCategory = true;

    switch (RabbitizerInstr_GetFmt(self)) {
        case 0b00000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mfc0;
            break;
        case 0b00001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmfc0;
            break;
        case 0b00010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cfc0;
            break;
        // 0b00_011: "",
        case 0b00100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mtc0;
            break;
        case 0b00101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmtc0;
            break;
        case 0b00110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ctc0;
            break;
        // 0b00_111: "",

        case 0b01000:
            if (RabbitizerInstr_GetTf(self)) {
                if (RabbitizerInstr_GetNd(self)) {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc0tl;
                } else {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc0t;
                }
            } else {
                if (RabbitizerInstr_GetNd(self)) {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc0fl;
                } else {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc0f;
                }
            }
            break;

        default:
            switch (self->function) {
                case 0b000001:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlbr;
                    break;
                case 0b000010:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlbwi;
                    break;
                case 0b000110:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlbwr;
                    break;
                case 0b001000:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_tlbp;
                    break;
                case 0b011000:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_eret;
                    break;

                default:
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;
                    break;
            }
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];
}


void RabbitizerInstr_ProcessUniqueId_Coprocessor1(RabbitizerInstr *self) {
    uint8_t fmt = RabbitizerInstr_GetFmt(self);
    uint8_t fc;

    self->_handwrittenCategory = true;
    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_INVALID;

    switch (fmt) {
        case 0b00000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mfc1;
            break;
        case 0b00001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmfc1;
            break;
        case 0b00010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cfc1;
            break;

        case 0b00100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mtc1;
            break;
        case 0b00101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_dmtc1;
            break;
        case 0b00110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ctc1;
            break;

        case 0b01000: // fmt = BC
            if (RabbitizerInstr_GetTf(self)) {
                if (RabbitizerInstr_GetNd(self)) {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc1tl;
                } else {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc1t;
                }
            } else {
                if (RabbitizerInstr_GetNd(self)) {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc1fl;
                } else {
                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bc1f;
                }
            }
            break;

        default:
            fmt = fmt & 0x07;
            switch (self->function) {
                case 0b000000:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_add_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_add_d;
                    }
                    break;
                case 0b000001:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sub_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sub_d;
                    }
                    break;
                case 0b000010:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mul_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mul_d;
                    }
                    break;
                case 0b000011:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_div_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_div_d;
                    }
                    break;

                case 0b000100:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sqrt_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sqrt_d;
                    }
                    break;
                case 0b000101:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_abs_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_abs_d;
                    }
                    break;
                case 0b000110:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mov_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_mov_d;
                    }
                    break;
                case 0b000111:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_neg_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_neg_d;
                    }
                    break;

                case 0b001000:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_round_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_round_l_d;
                    }
                    break;
                case 0b001001:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_trunc_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_trunc_l_d;
                    }
                    break;
                case 0b001010:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ceil_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ceil_l_d;
                    }
                    break;
                case 0b001011:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_floor_l_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_floor_l_d;
                    }
                    break;

                case 0b001100:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_round_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_round_w_d;
                    }
                    break;
                case 0b001101:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_trunc_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_trunc_w_d;
                    }
                    break;
                case 0b001110:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ceil_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ceil_w_d;
                    }
                    break;
                case 0b001111:
                    if (fmt == 0) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_floor_w_s;
                    } else if (fmt == 1) {
                        self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_floor_w_d;
                    }
                    break;

                default:
                    fc = RabbitizerInstr_GetFc(self);
                    if (fc == 0b11) {
                        // Compare conditions codes
                        switch (RabbitizerInstr_GetCond(self)) {
                            case 0b0000:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_f_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_f_d;
                                }
                                break;
                            case 0b0001:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_un_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_un_d;
                                }
                                break;
                            case 0b0010:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_eq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_eq_d;
                                }
                                break;
                            case 0b0011:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ueq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ueq_d;
                                }
                                break;
                            case 0b0100:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_olt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_olt_d;
                                }
                                break;
                            case 0b0101:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ult_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ult_d;
                                }
                                break;
                            case 0b0110:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ole_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ole_d;
                                }
                                break;
                            case 0b0111:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ule_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ule_d;
                                }
                                break;

                            case 0b1000:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_sf_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_sf_d;
                                }
                                break;
                            case 0b1001:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngle_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngle_d;
                                }
                                break;
                            case 0b1010:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_seq_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_seq_d;
                                }
                                break;
                            case 0b1011:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngl_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngl_d;
                                }
                                break;
                            case 0b1100:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_lt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_lt_d;
                                }
                                break;
                            case 0b1101:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_nge_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_nge_d;
                                }
                                break;
                            case 0b1110:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_le_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_le_d;
                                }
                                break;
                            case 0b1111:
                                if (fmt == 0) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngt_s;
                                } else if (fmt == 1) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_c_ngt_d;
                                }
                                break;
                        }

                    } else if (fc == 0b10) {
                        // Convert codes
                        switch (self->function & 0x07) {
                            case 0b000:
                                if (fmt == 0b001) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_s_d;
                                } else if (fmt == 0b100) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_s_w;
                                } else if (fmt == 0b101) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_s_l;
                                }
                                break;
                            case 0b001:
                                if (fmt == 0b000) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_d_s;
                                } else if (fmt == 0b100) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_d_w;
                                } else if (fmt == 0b101) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_d_l;
                                }
                                break;
                            case 0b100:
                                if (fmt == 0b000) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_w_s;
                                } else if (fmt == 0b001) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_w_d;
                                }
                                break;
                            case 0b101:
                                if (fmt == 0b000) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_l_s;
                                } else if (fmt == 0b001) {
                                    self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cvt_l_d;
                                }
                                break;
                        }
                    }
                    break;
            }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];
}


void RabbitizerInstr_ProcessUniqueId_Coprocessor2(RabbitizerInstr *self) {
    self->_handwrittenCategory = true;

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId.cpuId];
}



void RabbitizerInstr_ProcessUniqueId(RabbitizerInstr *self) {
    switch (self->opcode) {
        default:
            RabbitizerInstr_ProcessUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstr_ProcessUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstr_ProcessUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstr_ProcessUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstr_ProcessUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstr_ProcessUniqueId_Coprocessor2(self);
            break;
    }
}
