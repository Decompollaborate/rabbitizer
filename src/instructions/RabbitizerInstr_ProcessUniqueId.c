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
