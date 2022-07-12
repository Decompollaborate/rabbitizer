/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionRsp.h"

#include "common/RabbitizerConfig.h"


// NOLINTBEGIN(readability-magic-numbers)

void RabbitizerInstructionRsp_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);
    uint32_t rd;
    uint32_t elementlow;

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    switch (opcode) {
        case 0b000010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_j;
            break;
        case 0b000011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_jal;
            break;
        case 0b000100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_beq;
            break;
        case 0b000101:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bne;
            break;
        case 0b000110:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_blez;
            break;
        case 0b000111:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bgtz;
            break;

        case 0b001000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_addi;
            break;
        case 0b001001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_addiu;
            break;
        case 0b001010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_slti;
            break;
        case 0b001011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sltiu;
            break;
        case 0b001100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_andi;
            break;
        case 0b001101:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_ori;
            break;
        case 0b001110:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_xori;
            break;
        case 0b001111:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lui;
            break;

        case 0b100000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lb;
            break;
        case 0b100001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lh;
            break;
        case 0b100011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lw;
            break;
        case 0b100100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lbu;
            break;
        case 0b100101:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lhu;
            break;

        case 0b101000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sb;
            break;
        case 0b101001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sh;
            break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sw;
            break;
        case 0b101111:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_cache;
            break;

        case 0b110001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_lwc1;
            break;
        case 0b110011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_pref;
            break;

        case 0b111001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_swc1;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;

        // new rsp stuff
        case 0b111010:
            rd = RAB_INSTR_GET_rd(self);
            self->_mandatorybits = RAB_INSTR_PACK_rd(self->_mandatorybits, rd);
            switch (rd) {
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
                    elementlow = RAB_INSTR_RSP_GET_elementlow(self);
                    self->_mandatorybits = RAB_INSTR_RSP_PACK_elementlow(self->_mandatorybits, elementlow);
                    if (elementlow == 0) {
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
            rd = RAB_INSTR_GET_rd(self);
            self->_mandatorybits = RAB_INSTR_PACK_rd(self->_mandatorybits, rd);
            switch (rd) {
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

    if (RabbitizerConfig_Cfg.pseudos.enablePseudos) {
        if (RAB_INSTR_GET_rt(self) == 0) {
            if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_beq) {
                if (RAB_INSTR_GET_rs(self) == 0) {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoB) {
                        self->uniqueId = RABBITIZER_INSTR_ID_rsp_b;
                    }
                } else {
                    if (RabbitizerConfig_Cfg.pseudos.pseudoBeqz) {
                        self->uniqueId = RABBITIZER_INSTR_ID_rsp_beqz;
                    }
                }
            } else if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_bne) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoBnez) {
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_bnez;
                }
            }
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionRsp_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);

    switch (function) {
        case 0b000000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sll;
            break;
        case 0b000010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_srl;
            break;
        case 0b000011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sra;
            break;
        case 0b000100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sllv;
            break;
        case 0b000110:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_srlv;
            break;
        case 0b000111:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_srav;
            break;

        case 0b001000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_jr;
            break;
        case 0b001001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_jalr;
            break;
        case 0b001010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_movz;
            break;
        case 0b001011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_movn;
            break;
        case 0b001101:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_break;
            break;

        case 0b100000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_add;
            break;
        case 0b100001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_addu;
            break;
        case 0b100010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sub;
            break;
        case 0b100011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_subu;
            break;
        case 0b100100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_and;
            break;
        case 0b100101:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_or;
            break;
        case 0b100110:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_xor;
            break;
        case 0b100111:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_nor;
            break;

        case 0b101010:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_slt;
            break;
        case 0b101011:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_sltu;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    if (RabbitizerInstruction_isNop(self)) {
        // NOP is special enough
        self->uniqueId = RABBITIZER_INSTR_ID_rsp_nop;
    } else if (RabbitizerConfig_Cfg.pseudos.enablePseudos) {
        if (RAB_INSTR_GET_rt(self) == 0) {
            if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_or) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoMove) {
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_move;
                }
            } else if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_nor) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoNot) {
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_not;
                }
            }
        } else if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_subu) {
            if (RAB_INSTR_GET_rs(self) == 0) {
                if (RabbitizerConfig_Cfg.pseudos.pseudoNegu) {
                    self->uniqueId = RABBITIZER_INSTR_ID_rsp_negu;
                }
            }
        }
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];

    if (self->uniqueId == RABBITIZER_INSTR_ID_rsp_jalr) {
        // $ra
        if (RAB_INSTR_GET_rd(self) != 31) {
            self->descriptor = &RabbitizerInstrDescriptor_Descriptors[RABBITIZER_INSTR_ID_rsp_jalr_rd];
        }
    }
}

void RabbitizerInstructionRsp_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);

    switch (rt) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bltz;
            break;
        case 0b00001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bgez;
            break;

        case 0b10000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bltzal;
            break;
        case 0b10001:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_bgezal;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionRsp_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);

    switch (fmt) {
        case 0b00000:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_mfc0;
            break;

        case 0b00100:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_mtc0;
            break;

        default:
            self->uniqueId = RABBITIZER_INSTR_ID_rsp_INVALID;
            break;
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionRsp_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint32_t aux = SHIFTR(self->word, 25, 1);
    uint32_t elementhigh;
    uint32_t function;

    // TODO: name this bit range
    self->_mandatorybits = BITREPACK(self->_mandatorybits, aux, 25, 1);

    if (aux == 0) {
        elementhigh = RAB_INSTR_RSP_GET_elementhigh(self);
        self->_mandatorybits = RAB_INSTR_RSP_PACK_elementhigh(self->_mandatorybits, elementhigh);
        switch (elementhigh) {
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
        function = RAB_INSTR_GET_function(self);
        self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
        switch (function) {
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

void RabbitizerInstructionRsp_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);

    switch (opcode) {
        default:
            RabbitizerInstructionRsp_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionRsp_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionRsp_processUniqueId_Regimm(self);
            break;
        case 0x10:
            RabbitizerInstructionRsp_processUniqueId_Coprocessor0(self);
            break;
        // case 0x11:
        //    RabbitizerInstructionRsp_processUniqueId_Coprocessor1(self);
        //    break;
        case 0x12:
            RabbitizerInstructionRsp_processUniqueId_Coprocessor2(self);
            break;
    }
}

// NOLINTEND(readability-magic-numbers)
