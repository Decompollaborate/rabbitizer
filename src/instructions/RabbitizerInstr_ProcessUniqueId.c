/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstr.h"

void RabbitizerInstr_ProcessUniqueId_Normal(RabbitizerInstr *self) {
    switch (self->opcode) {
        // 0b000'000: "SPECIAL",
        // 0b000'001: "REGIMM",
        case 0b000'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_j;
            break;
        case 0b000'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_jal;
            break;
        case 0b000'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_beq;
            break;
        case 0b000'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bne;
            break;
        case 0b000'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_blez;
            break;
        case 0b000'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgtz;
            break;

        case 0b001'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_addi;
            break;
        case 0b001'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_addiu;
            break;
        case 0b001'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_slti;
            break;
        case 0b001'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sltiu;
            break;
        case 0b001'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_andi;
            break;
        case 0b001'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ori;
            break;
        case 0b001'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_xori;
            break;
        case 0b001'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lui;
            break;

        // 0b010'000: "COP0", // Coprocessor OPeration z
        // 0b010'001: "COP1", // Coprocessor OPeration z
        // 0b010'010: "COP2", // Coprocessor OPeration z
        // 0b010'011: "COP3", // Coprocessor OPeration z
        case 0b010'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_beql;
            break;
        case 0b010'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bnel;
            break;
        case 0b010'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_blezl;
            break;
        case 0b010'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_bgtzl;

            break;
        case 0b011'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_daddi;
            break;
        case 0b011'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_daddiu;
            break;
        case 0b011'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldl;
            break;
        case 0b011'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldr;
            break;

        case 0b100'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lb;
            break;
        case 0b100'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lh;
            break;
        case 0b100'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwl;
            break;
        case 0b100'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lw;
            break;
        case 0b100'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lbu;
            break;
        case 0b100'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lhu;
            break;
        case 0b100'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwr;
            break;
        case 0b100'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwu;
            break;

        case 0b101'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sb;
            break;
        case 0b101'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sh;
            break;
        case 0b101'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swl;
            break;
        case 0b101'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sw;
            break;
        case 0b101'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdl;
            break;
        case 0b101'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdr;
            break;
        case 0b101'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swr;
            break;
        case 0b101'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_cache;
            break;

        case 0b110'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ll;
            break;
        case 0b110'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwc1;
            break;
        case 0b110'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lwc2;
            break;
        case 0b110'011:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_pref;
            break;
        case 0b110'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_lld;
            break;
        case 0b110'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldc1;
            break;
        case 0b110'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ldc2;
            break;
        case 0b110'111:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_ld;
            break;

        case 0b111'000:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sc;
            break;
        case 0b111'001:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swc1;
            break;
        case 0b111'010:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_swc2;
        // 0b111'011: "",
            break;
        case 0b111'100:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_scd;
            break;
        case 0b111'101:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdc1;
            break;
        case 0b111'110:
            self->uniqueId.cpuId = RABBITIZER_INSTR_CPU_ID_sdc2;
            break;
        case 0b111'111:
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
