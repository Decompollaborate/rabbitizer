/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrRsp.h"

#include "common/RabbitizerConfig.h"




void RabbitizerInstrRsp_processUniqueId_Normal(RabbitizerInstr *self) {
    switch (self->opcode) {
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

        //case 0b010100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_beql;
        //    break;
        //case 0b010101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bnel;
        //    break;
        //case 0b010110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_blezl;
        //    break;
        //case 0b010111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgtzl;
        //    break;

        //case 0b011000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddi;
        //    break;
        //case 0b011001:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddiu;
        //    break;
        //case 0b011010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldl;
        //    break;
        //case 0b011011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldr;
        //    break;

        case 0b100000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lb;
            break;
        case 0b100001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lh;
            break;
        //case 0b100010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwl;
        //    break;
        case 0b100011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lw;
            break;
        case 0b100100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lbu;
            break;
        case 0b100101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lhu;
            break;
        //case 0b100110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwr;
        //    break;
        //case 0b100111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwu;
        //    break;

        case 0b101000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sb;
            break;
        case 0b101001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sh;
            break;
        //case 0b101010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_swl;
        //    break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sw;
            break;
        //case 0b101100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdl;
        //    break;
        //case 0b101101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdr;
        //    break;
        //case 0b101110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_swr;
        //    break;
        case 0b101111:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_cache;
            break;

        //case 0b110000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ll;
        //    break;
        case 0b110001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwc1;
            break;
        //case 0b110010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_lwc2;
        //    break;
        case 0b110011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_pref;
            break;
        //case 0b110100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_lld;
        //    break;
        //case 0b110101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldc1;
        //    break;
        //case 0b110110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ldc2;
        //    break;
        //case 0b110111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ld;
        //    break;

        //case 0b111000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sc;
        //    break;
        case 0b111001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_swc1;
            break;
        //case 0b111010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_swc2;
        //    break;

        //case 0b111100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_scd;
        //    break;
        //case 0b111101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdc1;
        //    break;
        //case 0b111110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sdc2;
        //    break;
        //case 0b111111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sd;
        //    break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;

        // new rsp stuff
        case 0b111010:
            switch (self->rd) {
                case 0b00000:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_sbv;
                    break;
                case 0b00001:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_ssv;
                    break;
                case 0b00010:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_slv;
                    break;
                case 0b00011:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_sdv;
                    break;
                case 0b00100:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_sqv;
                    break;
                case 0b00101:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_srv;
                    break;
                case 0b00110:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_spv;
                    break;
                case 0b00111:
                    if (RAB_INSTR_RSP_GET_ELEMENT_LOW(self) == 0) {
                        self->uniqueId = RABBITIZER_INSTR_ID_rsp_suv;
                    } else {
                        self->uniqueId = RABBITIZER_INSTR_ID_rsp_swv;
                    }
                    break;
                case 0b01000:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_shv;
                    break;
                case 0b01001:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_sfv;
                    break;
                case 0b01011:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_stv;
                    break;

                default:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
                    break;
            }
            break;

        case 0b110010:
            switch (self->rd) {
                case 0b00000:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lbv;
                    break;
                case 0b00001:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lsv;
                    break;
                case 0b00010:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_llv;
                    break;
                case 0b00011:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_ldv;
                    break;
                case 0b00100:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lqv;
                    break;
                case 0b00101:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lrv;
                    break;
                case 0b00110:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lpv;
                    break;
                case 0b00111:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_luv;
                    break;
                case 0b01000:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lhv;
                    break;
                case 0b01001:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_lfv;
                    break;
                case 0b01011:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_ltv;
                    break;

                default:
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
                    break;
            }
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}


void RabbitizerInstrRsp_processUniqueId_Special(RabbitizerInstr *self) {
    switch (self->function) {
        case 0b000000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sll;
            break;
        case 0b000010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_srl;
            break;
        case 0b000011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sra;
            break;
        case 0b000100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sllv;
            break;
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
            break;
        case 0b001010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_movz;
            break;
        case 0b001011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_movn;
            break;
        //case 0b001100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_syscall;
        //    break;
        case 0b001101:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_break;
            break;
        //case 0b001111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_sync;
        //    break;

        //case 0b010000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_mfhi;
        //    break;
        //case 0b010001:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_mthi;
        //    break;
        //case 0b010010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_mflo;
        //    break;
        //case 0b010011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_mtlo;
        //    break;
        //case 0b010100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsllv;
        //    break;
        //case 0b010110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrlv;
        //    break;
        //case 0b010111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrav;
        //    break;

        //case 0b011000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_mult;
        //    break;
        //case 0b011001:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_multu;
        //    break;
        //case 0b011010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_div;
        //    break;
        //case 0b011011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_divu;
        //    break;
        //case 0b011100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmult;
        //    break;
        //case 0b011101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dmultu;
        //    break;
        //case 0b011110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ddiv;
        //    break;
        //case 0b011111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_ddivu;
        //    break;

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

        case 0b101010:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_slt;
            break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_sltu;
            break;
        //case 0b101100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dadd;
        //    break;
        //case 0b101101:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_daddu;
        //    break;
        //case 0b101110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsub;
        //    break;
        //case 0b101111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsubu;
        //    break;

        //case 0b110000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tge;
        //    break;
        //case 0b110001:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgeu;
        //    break;
        //case 0b110010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlt;
        //    break;
        //case 0b110011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tltu;
        //    break;
        //case 0b110100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_teq;
        //    break;
        //case 0b110110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tne;
        //    break;

        //case 0b111000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsll;
        //    break;
        //case 0b111010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrl;
        //    break;
        //case 0b111011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsra;
        //    break;
        //case 0b111100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsll32;
        //    break;
        //case 0b111110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsrl32;
        //    break;
        //case 0b111111:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_dsra32;
        //    break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    if (RabbitizerInstr_isNop(self)) {
        // NOP is special enough
        self->uniqueId = RABBITIZER_INSTR_ID_cpu_nop;
    } else if (RabbitizerConfig_Cfg.pseudos.enablePseudos) {
        if (self->rt == 0) {
            if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_or) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoMove) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_move;
                }
            } else if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_nor) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoNot) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_not;
                }
            }
        } else if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_subu) {
            if (self->rs == 0) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoNegu) {
                    self->uniqueId = RABBITIZER_INSTR_ID_cpu_negu;
                }
            }
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_jalr) {
        // $ra
        if (self->rd != 31) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_jalr_rd];
        }
    } else if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_div) {
        if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix && !self->inHandwrittenFunction) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_sn64_div];
        }
    } else if (self->uniqueId == RABBITIZER_INSTR_ID_cpu_divu) {
        if (RabbitizerConfig_Cfg.toolchainTweaks.sn64DivFix && !self->inHandwrittenFunction) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_cpu_sn64_divu];
        }
    }
}


void RabbitizerInstrRsp_processUniqueId_Regimm(RabbitizerInstr *self) {
    switch (self->rt) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltz;
            break;
        case 0b00001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgez;
            break;
        //case 0b00010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzl;
        //    break;
        //case 0b00011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezl;
        //    break;

        //case 0b01000:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgei;
        //    break;
        //case 0b01001:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tgeiu;
        //    break;
        //case 0b01010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tlti;
        //    break;
        //case 0b01011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tltiu;
        //    break;

        case 0b10000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzal;
            break;
        case 0b10001:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezal;
            break;
        //case 0b10010:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bltzall;
        //    break;
        //case 0b10011:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_bgezall;
        //    break;

        //case 0b01100:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_teqi;
        //    break;
        //case 0b01110:
        //    self->uniqueId = RABBITIZER_INSTR_ID_cpu_tnei;
        //    break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}


void RabbitizerInstrRsp_processUniqueId_Coprocessor0(RabbitizerInstr *self) {
    switch (RabbitizerInstr_getFmt(self)) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mfc0;
            break;

        case 0b00100:
            self->uniqueId = RABBITIZER_INSTR_ID_cpu_mtc0;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}


void RabbitizerInstrRsp_processUniqueId_Coprocessor2(RabbitizerInstr *self) {
    if (((self->rs >> 4) & 0x1) == 0) {
        switch (RAB_INSTR_RSP_GET_ELEMENT_HIGH(self)) {
            case 0b00000:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_mfc2;
                break;

            case 0b00100:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_mtc2;
                break;

            case 0b00010:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_cfc2;
                break;

            case 0b00110:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_ctc2;
                break;

            default:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
                break;
        }
    } else {
        switch (self->function) {
            case 0x00:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmulf;
                break;
            case 0x01:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmulu;
                break;
            case 0b000010:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrndp;
                break;
            case 0b000011:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmulq;
                break;
            case 0x04:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmudl;
                break;
            case 0x05:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmudm;
                break;
            case 0x06:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmudn;
                break;
            case 0x07:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmudh;
                break;
            case 0x08:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmacf;
                break;
            case 0x09:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmacu;
                break;
            case 0b001010:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrndn;
                break;
            case 0b001011:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmacq;
                break;
            case 0x0C:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmadl;
                break;
            case 0x0D:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmadm;
                break;
            case 0x0E:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmadn;
                break;
            case 0x0F:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmadh;
                break;
            case 0x10:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vadd;
                break;
            case 0b010001:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vsub;
                break;
            case 0b010011:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vabs;
                break;
            case 0x14:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vaddc;
                break;
            case 0b010101:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vsubc;
                break;
            case 0x1D:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vsar;
                break;
            case 0x28:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vand;
                break;
            case 0x29:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vnand;
                break;
            case 0x2A:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vor;
                break;
            case 0x2B:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vnor;
                break;
            case 0x2C:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vxor;
                break;
            case 0x2D:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vnxor;
                break;

            case 0x20:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vlt;
                break;
            case 0x21:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_veq;
                break;
            case 0x22:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vne;
                break;
            case 0x23:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vge;
                break;
            case 0x24:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vcl;
                break;
            case 0x25:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vch;
                break;
            case 0x26:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vcr;
                break;
            case 0x27:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmrg;
                break;

            case 0b110000:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrcp;
                break;
            case 0b110001:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrcpl;
                break;
            case 0b110010:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrcph;
                break;
            case 0b110011:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vmov;
                break;
            case 0b110100:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrsq;
                break;
            case 0b110101:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrsql;
                break;
            case 0b110110:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vrsqh;
                break;
            case 0b110111:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_vnop;
                break;

            default:
                self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
                break;
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}


void RabbitizerInstrRsp_processUniqueId(RabbitizerInstr *self) {
    switch (self->opcode) {
        default:
            RabbitizerInstrRsp_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstrRsp_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstrRsp_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstrRsp_processUniqueId_Coprocessor0(self);
            break;
        //case 0x11:
        //    RabbitizerInstrRsp_processUniqueId_Coprocessor1(self);
        //    break;
        case 0x12:
            //RabbitizerInstrRsp_processUniqueId_Coprocessor2(self);
            break;
    }
}
