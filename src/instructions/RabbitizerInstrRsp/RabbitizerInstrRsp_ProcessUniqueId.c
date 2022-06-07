/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrRsp.h"

//#include "common/RabbitizerConfig.h"




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



void RabbitizerInstrRsp_processUniqueId(RabbitizerInstr *self) {
    switch (self->opcode) {
        default:
            RabbitizerInstrRsp_processUniqueId_Normal(self);
            break;
        case 0x00:
            //RabbitizerInstrRsp_processUniqueId_Special(self);
            break;
        case 0x01:
            //RabbitizerInstrRsp_processUniqueId_Regimm(self);
            break;
        case 0x10:
            //RabbitizerInstrRsp_processUniqueId_Coprocessor0(self);
            break;
        //case 0x11:
        //    RabbitizerInstrRsp_processUniqueId_Coprocessor1(self);
        //    break;
        case 0x12:
            //RabbitizerInstrRsp_processUniqueId_Coprocessor2(self);
            break;
    }
}
