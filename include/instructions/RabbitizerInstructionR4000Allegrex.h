/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R4000ALLEGREX_H
#define RABBITIZER_INSTRUCTION_R4000ALLEGREX_H
#pragma once

#include "RabbitizerInstruction.h"

#ifdef __cplusplus
extern "C" {
#endif


#define RAB_INSTR_R4000ALLEGREX_GET_vt(self)                                    (SHIFTR((self)->word, 16,  7))
#define RAB_INSTR_R4000ALLEGREX_GET_vs(self)                                    (SHIFTR((self)->word,  8,  7))
#define RAB_INSTR_R4000ALLEGREX_GET_vd(self)                                    (SHIFTR((self)->word,  0,  7))
// For whatever reason the transpose just toggles bit 5, no clue why.
#define RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(self)                          (SHIFTR((self)->word,  8,  7) ^ 0x20)

#define RAB_INSTR_R4000ALLEGREX_GET_vt_imm(self)                                ((SHIFTR((self)->word,  0,  2) << 5) | (SHIFTR((self)->word, 16,  5)))
#define RAB_INSTR_R4000ALLEGREX_GET_vd_imm(self)                                (SHIFTR((self)->word, 16,  7))

#define RAB_INSTR_R4000ALLEGREX_GET_vt_6_imm(self)                              ((SHIFTR((self)->word,  0,  1) << 5) | (SHIFTR((self)->word, 16,  5)))

#define RAB_INSTR_R4000ALLEGREX_GET_cop2cs(self)                                (SHIFTR((self)->word,  8,  7))
#define RAB_INSTR_R4000ALLEGREX_GET_cop2cd(self)                                (SHIFTR((self)->word,  0,  7))

#define RAB_INSTR_R4000ALLEGREX_GET_pos(self)                                   (SHIFTR((self)->word,  6,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_size(self)                                  (SHIFTR((self)->word, 11,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_size_plus_pos(self)                         (SHIFTR((self)->word, 11,  5))

#define RAB_INSTR_R4000ALLEGREX_GET_bc2_fmt(self)                               (SHIFTR((self)->word, 16,  2))
#define RAB_INSTR_R4000ALLEGREX_GET_mxhc2(self)                                 (SHIFTR((self)->word,  7,  1))
#define RAB_INSTR_R4000ALLEGREX_GET_mfhc2_p_fmt(self)                           (SHIFTR((self)->word,  4,  3))
#define RAB_INSTR_R4000ALLEGREX_GET_mfhc2_p_s_fmt(self)                         (SHIFTR((self)->word,  0,  4))

#define RAB_INSTR_R4000ALLEGREX_GET_imm3(self)                                  (SHIFTR((self)->word, 18,  3))
#define RAB_INSTR_R4000ALLEGREX_GET_offset14(self)                              (SHIFTR((self)->word,  2, 14))
#define RAB_INSTR_R4000ALLEGREX_GET_wb(self)                                    (SHIFTR((self)->word,  1,  1))

#define RAB_INSTR_R4000ALLEGREX_GET_tp(self)                                    ((SHIFTR((self)->word, 15,  1) << 1) | (SHIFTR((self)->word, 7,  1)))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu0_fmt(self)                             ((SHIFTR((self)->word, 23,  3) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt(self)                             ((SHIFTR((self)->word, 24,  2) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt(self)                        (SHIFTR((self)->word, 19,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt0_fmt0_fmt(self)                   ((SHIFTR((self)->word, 16,  3) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt2_fmt(self)                        ((SHIFTR((self)->word, 21,  3) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu4_fmt2_cndmove_fmt(self)                ((SHIFTR((self)->word, 19,  2) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu5_fmt(self)                             (SHIFTR((self)->word, 23,  3))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt(self)                             ((SHIFTR((self)->word, 23,  3) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt7_fmt(self)                        ((SHIFTR((self)->word, 21,  2) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu6_fmt7_fmt0_fmt(self)                   ((SHIFTR((self)->word, 16,  3) << 2) | RAB_INSTR_R4000ALLEGREX_GET_tp(self))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu7_fmt(self)                             (SHIFTR((self)->word,  0, 26))

#define RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(self)                             (SHIFTR((self)->word,  0,  4))
#define RAB_INSTR_R4000ALLEGREX_GET_vconstant(self)                             (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_power_of_two(self)                          (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_vfpu_cc_bit(self)                           (SHIFTR((self)->word, 16,  3))
#define RAB_INSTR_R4000ALLEGREX_GET_bn(self)                                    (SHIFTR((self)->word, 16,  8))

#define RAB_INSTR_R4000ALLEGREX_GET_intfloat16(self)                            (SHIFTR((self)->word,  0, 16))
#define RAB_INSTR_R4000ALLEGREX_GET_vrot_code(self)                             (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_R4000ALLEGREX_GET_rpx(self)                                   ((SHIFTR((self)->word, 16,  1) << 4) | (SHIFTR((self)->word, 12,  1) << 3) | (SHIFTR((self)->word,  8,  1) << 2) | (SHIFTR((self)->word,  0,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_rpy(self)                                   ((SHIFTR((self)->word, 17,  1) << 4) | (SHIFTR((self)->word, 13,  1) << 3) | (SHIFTR((self)->word,  9,  1) << 2) | (SHIFTR((self)->word,  2,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_rpz(self)                                   ((SHIFTR((self)->word, 18,  1) << 4) | (SHIFTR((self)->word, 14,  1) << 3) | (SHIFTR((self)->word, 10,  1) << 2) | (SHIFTR((self)->word,  4,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_rpw(self)                                   ((SHIFTR((self)->word, 19,  1) << 4) | (SHIFTR((self)->word, 15,  1) << 3) | (SHIFTR((self)->word, 11,  1) << 2) | (SHIFTR((self)->word,  6,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_wpx(self)                                   ((SHIFTR((self)->word,  8,  1) << 2) | (SHIFTR((self)->word,  0,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_wpy(self)                                   ((SHIFTR((self)->word,  9,  1) << 2) | (SHIFTR((self)->word,  2,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_wpz(self)                                   ((SHIFTR((self)->word, 10,  1) << 2) | (SHIFTR((self)->word,  4,  2) << 0))
#define RAB_INSTR_R4000ALLEGREX_GET_wpw(self)                                   ((SHIFTR((self)->word, 11,  1) << 2) | (SHIFTR((self)->word,  6,  2) << 0))


#define RAB_INSTR_R4000ALLEGREX_PACK_vt(word, value)                            (BITREPACK((word), (value), 16,  7))
#define RAB_INSTR_R4000ALLEGREX_PACK_vs(word, value)                            (BITREPACK((word), (value),  8,  7))
#define RAB_INSTR_R4000ALLEGREX_PACK_vd(word, value)                            (BITREPACK((word), (value),  0,  7))

#define RAB_INSTR_R4000ALLEGREX_PACK_vt_imm(word, value)                        (BITREPACK(BITREPACK((word), (value) >> 5,  0,  2), (value), 16,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_vd_imm(word, value)                        (BITREPACK((word), (value), 16,  7))

#define RAB_INSTR_R4000ALLEGREX_PACK_vt_6_imm(word, value)                      (BITREPACK(BITREPACK((word), (value) >> 5,  0,  1), (value), 16,  5))

#define RAB_INSTR_R4000ALLEGREX_PACK_cop2cs(word, value)                        (BITREPACK((word), (value),  8,  7))
#define RAB_INSTR_R4000ALLEGREX_PACK_cop2cd(word, value)                        (BITREPACK((word), (value),  0,  7))

#define RAB_INSTR_R4000ALLEGREX_PACK_pos(word, value)                           (BITREPACK((word), (value),  6,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_size(word, value)                          (BITREPACK((word), (value), 11,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_size_plus_pos(word, value)                 (BITREPACK((word), (value), 11,  5))

#define RAB_INSTR_R4000ALLEGREX_PACK_bc2_fmt(word, value)                       (BITREPACK((word), (value), 16,  2))
#define RAB_INSTR_R4000ALLEGREX_PACK_mxhc2(word, value)                         (BITREPACK((word), (value),  7,  1))
#define RAB_INSTR_R4000ALLEGREX_PACK_mfhc2_p_fmt(word, value)                   (BITREPACK((word), (value),  4,  3))
#define RAB_INSTR_R4000ALLEGREX_PACK_mfhc2_p_s_fmt(word, value)                 (BITREPACK((word), (value),  0,  4))

#define RAB_INSTR_R4000ALLEGREX_PACK_imm3(word, value)                          (BITREPACK((word), (value), 18,  3))
#define RAB_INSTR_R4000ALLEGREX_PACK_offset14(word, value)                      (BITREPACK((word), (value),  2, 14))
#define RAB_INSTR_R4000ALLEGREX_PACK_wb(word, value)                            (BITREPACK((word), (value),  1,  1))

#define RAB_INSTR_R4000ALLEGREX_PACK_tp(word, value)                            (BITREPACK(BITREPACK((word), (value) >> 1, 15,  1), (value),  7,  1))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu0_fmt(word, value)                     (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 23,  3), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt(word, value)                     (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 24,  2), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt(word, value)                (BITREPACK((word), (value), 19,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt0_fmt0_fmt(word, value)           (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 16,  3), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt2_fmt(word, value)                (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 21,  3), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu4_fmt2_cndmove_fmt(word, value)        (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 19,  2), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu5_fmt(word, value)                     (BITREPACK((word), (value), 23,  3))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt(word, value)                     (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 23,  3), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt7_fmt(word, value)                (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 21,  2), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu6_fmt7_fmt0_fmt(word, value)           (RAB_INSTR_R4000ALLEGREX_PACK_tp(BITREPACK((word), (value) >> 2, 16,  3), (value)))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu7_fmt(word, value)                     (BITREPACK((word), (value),  0,  26))

#define RAB_INSTR_R4000ALLEGREX_PACK_vcmp_cond(word, value)                     (BITREPACK((word), (value),  0,  4))
#define RAB_INSTR_R4000ALLEGREX_PACK_vconstant(word, value)                     (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_power_of_two(word, value)                  (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_vfpu_cc_bit(word, value)                   (BITREPACK((word), (value), 16,  3))
#define RAB_INSTR_R4000ALLEGREX_PACK_bn(word, value)                            (BITREPACK((word), (value), 16,  8))

#define RAB_INSTR_R4000ALLEGREX_PACK_intfloat16(word, value)                    (BITREPACK((word), (value),  0, 16))
#define RAB_INSTR_R4000ALLEGREX_PACK_vrot_code(word, value)                     (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_R4000ALLEGREX_PACK_rpx(word, value)                           (BITREPACK(BITREPACK(BITREPACK(BITREPACK((word), (value) >> 4, 16,  1), (value) >> 3, 12, 1), (value) >> 2,  8, 1), (value),  0, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_rpy(word, value)                           (BITREPACK(BITREPACK(BITREPACK(BITREPACK((word), (value) >> 4, 17,  1), (value) >> 3, 13, 1), (value) >> 2,  9, 1), (value),  2, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_rpz(word, value)                           (BITREPACK(BITREPACK(BITREPACK(BITREPACK((word), (value) >> 4, 18,  1), (value) >> 3, 14, 1), (value) >> 2, 10, 1), (value),  4, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_rpw(word, value)                           (BITREPACK(BITREPACK(BITREPACK(BITREPACK((word), (value) >> 4, 19,  1), (value) >> 3, 15, 1), (value) >> 2, 11, 1), (value),  6, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_wpx(word, value)                           (BITREPACK(BITREPACK((word), (value) >> 2,  8, 1), (value),  0, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_wpy(word, value)                           (BITREPACK(BITREPACK((word), (value) >> 2,  9, 1), (value),  2, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_wpz(word, value)                           (BITREPACK(BITREPACK((word), (value) >> 2, 10, 1), (value),  4, 2))
#define RAB_INSTR_R4000ALLEGREX_PACK_wpw(word, value)                           (BITREPACK(BITREPACK((word), (value) >> 2, 11, 1), (value),  6, 2))


NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_destroy(RabbitizerInstruction *self);


NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Normal(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Rs(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special_Sa(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Regimm(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special2(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special3(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Special3_Bshfl(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_BC0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor0_Tlb(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_BC1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuS(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor1_FpuW(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_BC2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MFHC2_p_s(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Coprocessor2_MTHC2(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu3(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt3(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Rnd(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtFlt(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_CvtInt(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt8(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Fmt9(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Control(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Color(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt0_Cst(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu4_Fmt2_CndMove(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu5(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu6_Fmt7_Fmt0(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Vfpu7(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId_Quadlr(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000Allegrex_processUniqueId(RabbitizerInstruction *self);


#ifdef __cplusplus
}
#endif

#endif
