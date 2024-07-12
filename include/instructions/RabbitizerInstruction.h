/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_H
#define RABBITIZER_INSTRUCTION_H
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "common/Utils.h"

#include "RabbitizerInstrId.h"
#include "RabbitizerInstrDescriptor.h"
#include "RabbitizerInstrCategory.h"
#include "RabbitizerInstrIdType.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct RabbitizerInstruction {
    uint32_t word;
    uint32_t _mandatorybits;

    RabbitizerInstrId uniqueId;
    const RabbitizerInstrDescriptor *descriptor;
    RabInstrIdType instrIdType;

    uint32_t vram;
    bool _handwrittenCategory;
    bool inHandwrittenFunction;
    RabbitizerInstrCategory category;

    /**
     * Flags are bitpacked, refer to the `RAB_INSTR_FLAGS_` macros to access them.
     *
     * Bit usage:
     * - Bits 0 ~ 1: `r5900DisasmAsData`. Value of the `RabTrinaryValue` enum.
     *     - `RAB_TRINARY_VAL_TRUE` forces the instruction to be disassembled as data.
     *     - `RAB_TRINARY_VAL_FALSE` bypasses the global checks for disassembling a word as data. A word will still be disassembled as data if it can't be decoded.
     *     - `RAB_TRINARY_VAL_NONE` leaves this decision to the global settings.
     *     - Defaults to `RAB_TRINARY_VAL_NONE`
     * - Bit 2 ~ 3: `r5900UseDollar`. Value of the `RabTrinaryValue` enum.
     *     - `RAB_TRINARY_VAL_TRUE` forces the use of dollar signs ($) on R5900's VU instructions
     *     - `RAB_TRINARY_VAL_FALSE` forces disassembling to not use of dollar signs ($) on R5900's VU instructions
     *     - `RAB_TRINARY_VAL_NONE` leaves this decision to the global settings.
     *     - Defaults to `RAB_TRINARY_VAL_NONE`
     * - Bit 4 ~ 5: Reserved for future use.
     * - Bit 6 ~ 7: Reserved for future use.
     * - Bit 8 ~ 9: Reserved for future use.
     * - Bit 10 ~ 11: Reserved for future use.
     * - Bit 12 ~ 13: Reserved for future use.
     * - Bit 14 ~ 15: Reserved for future use.
     * - Bit 16 ~ 17: Reserved for future use.
     * - Bit 18 ~ 19: Reserved for future use.
     * - Bit 20 ~ 21: Reserved for future use.
     * - Bit 22 ~ 23: Reserved for future use.
     * - Bit 24 ~ 25: Reserved for future use.
     * - Bit 26 ~ 27: Reserved for future use.
     * - Bit 28 ~ 29: Reserved for future use.
     * - Bit 30 ~ 31: Reserved for future use.
     */
    uint32_t flags;
} RabbitizerInstruction;


#define RAB_INSTR_GET_opcode(self)                  (SHIFTR((self)->word, 26,  6))
#define RAB_INSTR_GET_rs(self)                      (SHIFTR((self)->word, 21,  5))
#define RAB_INSTR_GET_rt(self)                      (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_GET_rd(self)                      (SHIFTR((self)->word, 11,  5))
#define RAB_INSTR_GET_sa(self)                      (SHIFTR((self)->word,  6,  5))
#define RAB_INSTR_GET_function(self)                (SHIFTR((self)->word,  0,  6))

#define RAB_INSTR_GET_cop0d(self)                   (SHIFTR((self)->word, 11,  5))

#define RAB_INSTR_GET_instr_index(self)             (SHIFTR((self)->word,  0, 26))
#define RAB_INSTR_GET_immediate(self)               (SHIFTR((self)->word,  0, 16))

#define RAB_INSTR_GET_fs(self)                      (SHIFTR((self)->word, 11,  5))
#define RAB_INSTR_GET_ft(self)                      (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_GET_fd(self)                      (SHIFTR((self)->word,  6,  5))
#define RAB_INSTR_GET_cop1cs(self)                  (SHIFTR((self)->word, 11,  5))

#define RAB_INSTR_GET_op(self)                      (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_GET_hint(self)                    (SHIFTR((self)->word, 16,  5))

#define RAB_INSTR_GET_code(self)                    (SHIFTR((self)->word,  6, 20))
#define RAB_INSTR_GET_code_upper(self)              (SHIFTR((self)->word, 16, 10))
#define RAB_INSTR_GET_code_lower(self)              (SHIFTR((self)->word,  6, 10))

#define RAB_INSTR_GET_copraw(self)                  (SHIFTR((self)->word,  0, 25))

#define RAB_INSTR_GET_fmt(self)                     (SHIFTR((self)->word, 21,  5))
#define RAB_INSTR_GET_fc(self)                      (SHIFTR((self)->word,  4,  2))
#define RAB_INSTR_GET_cond(self)                    (SHIFTR((self)->word,  0,  4))

#define RAB_INSTR_GET_cop2t(self)                   (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_GET_cop2cd(self)                  (SHIFTR((self)->word, 11,  5))

#define RAB_INSTR_GET_tf(self)                      (SHIFTR((self)->word, 16,  1))
#define RAB_INSTR_GET_nd(self)                      (SHIFTR((self)->word, 17,  1))
#define RAB_INSTR_GET_bc_fmt(self)                  (SHIFTR((self)->word, 16,  5))

#define RAB_INSTR_GET_stype(self)                   (SHIFTR((self)->word,  6,  5))


#define RAB_INSTR_PACK_opcode(word, value)          (BITREPACK_RIGHT((word), (value), 26,  6))
#define RAB_INSTR_PACK_rs(word, value)              (BITREPACK((word), (value), 21,  5))
#define RAB_INSTR_PACK_rt(word, value)              (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_PACK_rd(word, value)              (BITREPACK((word), (value), 11,  5))
#define RAB_INSTR_PACK_sa(word, value)              (BITREPACK((word), (value),  6,  5))
#define RAB_INSTR_PACK_function(word, value)        (BITREPACK((word), (value),  0,  6))

#define RAB_INSTR_PACK_cop0d(word, value)           (BITREPACK((word), (value), 11,  5))

#define RAB_INSTR_PACK_instr_index(word, value)     (BITREPACK((word), (value),  0, 26))
#define RAB_INSTR_PACK_immediate(word, value)       (BITREPACK((word), (value),  0, 16))

#define RAB_INSTR_PACK_code(word, value)            (BITREPACK((word), (value),  6, 20))
#define RAB_INSTR_PACK_code_upper(word, value)      (BITREPACK((word), (value), 16, 10))
#define RAB_INSTR_PACK_code_lower(word, value)      (BITREPACK((word), (value),  6, 10))

#define RAB_INSTR_PACK_copraw(word, value)          (BITREPACK((word), (value),  0, 25))

#define RAB_INSTR_PACK_fmt(word, value)             (BITREPACK((word), (value), 21,  5))
#define RAB_INSTR_PACK_fc(word, value)              (BITREPACK((word), (value),  4,  2))
#define RAB_INSTR_PACK_cond(word, value)            (BITREPACK((word), (value),  0,  4))

#define RAB_INSTR_PACK_fs(word, value)              (BITREPACK((word), (value), 11,  5))
#define RAB_INSTR_PACK_ft(word, value)              (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_PACK_fd(word, value)              (BITREPACK((word), (value),  6,  5))
#define RAB_INSTR_PACK_cop1cs(word, value)          (BITREPACK((word), (value), 11,  5))

#define RAB_INSTR_PACK_op(word, value)              (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_PACK_hint(word, value)            (BITREPACK((word), (value), 16,  5))

#define RAB_INSTR_PACK_cop2t(word, value)           (BITREPACK((word), (value), 16,  5))
#define RAB_INSTR_PACK_cop2cd(word, value)          (BITREPACK((word), value, 11,  5))

#define RAB_INSTR_PACK_tf(word, value)              (BITREPACK((word), (value), 16,  1))
#define RAB_INSTR_PACK_nd(word, value)              (BITREPACK((word), (value), 17,  1))
#define RAB_INSTR_PACK_bc_fmt(word, value)          (BITREPACK((word), (value), 16,  5))

#define RAB_INSTR_PACK_stype(word, value)           (BITREPACK((word), (value),  6,  5))


#define RAB_INSTR_FLAGS_GET_r5900DisasmAsData(self)                       (RabTrinaryValue)(SHIFTR((self)->flags,  0,  2))
#define RAB_INSTR_FLAGS_SET_r5900DisasmAsData(self, value)     ((self)->flags = BITREPACK((self)->flags, (value),  0,  2))

#define RAB_INSTR_FLAGS_GET_r5900UseDollar(self)                          (RabTrinaryValue)(SHIFTR((self)->flags,  2,  2))
#define RAB_INSTR_FLAGS_SET_r5900UseDollar(self, value)        ((self)->flags = BITREPACK((self)->flags, (value),  2,  2))


NON_NULL(1)
void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstruction_destroy(RabbitizerInstruction* self);


/* Process uniqueId */

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Normal(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Regimm(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor0_BC0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor0_Tlb(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor0(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1_BC1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1_FpuS(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1_FpuD(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1_FpuW(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1_FpuL(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor1(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstruction_processUniqueId_Coprocessor2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstruction_processUniqueId(RabbitizerInstruction *self);

/* Process uniqueId */


/* General getters */

NODISCARD NON_NULL(1) PURE
uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self);

NODISCARD NON_NULL(1) PURE
int32_t RabbitizerInstruction_getProcessedImmediate(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
uint32_t RabbitizerInstruction_getInstrIndexAsVram(const RabbitizerInstruction *self);

NODISCARD NON_NULL(1) PURE
int32_t RabbitizerInstruction_getBranchOffset(const RabbitizerInstruction *self);

//! @deprecated
NODISCARD NON_NULL(1) PURE
int32_t RabbitizerInstruction_getGenericBranchOffset(const RabbitizerInstruction *self, uint32_t currentVram);

NODISCARD NON_NULL(1) PURE
int32_t RabbitizerInstruction_getBranchOffsetGeneric(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
uint32_t RabbitizerInstruction_getBranchVramGeneric(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
int8_t RabbitizerInstruction_getDestinationGpr(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_outputsToGprZero(const RabbitizerInstruction *self);

/* General getters */

NON_NULL(1)
void RabbitizerInstruction_blankOut(RabbitizerInstruction *self);


/* Instruction examination */

NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isImplemented(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isLikelyHandwritten(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isNop(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isUnconditionalBranch(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isFunctionCall(const RabbitizerInstruction *self);

NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isReturn(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isJumptableJump(const RabbitizerInstruction *self);

//! @deprecated
#define RabbitizerInstruction_isJrRa RabbitizerInstruction_isReturn
//! @deprecated
#define RabbitizerInstruction_isJrNotRa RabbitizerInstruction_isJumptableJump

NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_hasDelaySlot(const RabbitizerInstruction *self);

//! @deprecated
NODISCARD NON_NULL(1) PURE
const char *RabbitizerInstruction_mapInstrToType(const RabbitizerInstruction *self);

NODISCARD NON_NULL(1, 2) PURE
bool RabbitizerInstruction_sameOpcode(const RabbitizerInstruction *self, const RabbitizerInstruction *other);
NODISCARD NON_NULL(1, 2) PURE
bool RabbitizerInstruction_sameOpcodeButDifferentArguments(const RabbitizerInstruction *self, const RabbitizerInstruction *other);

NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_hasOperand(const RabbitizerInstruction *self, RabbitizerOperandType operand);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_hasOperandAlias(const RabbitizerInstruction *self, RabbitizerOperandType operand);

NODISCARD NON_NULL(1) PURE
uint32_t RabbitizerInstruction_getValidBits(const RabbitizerInstruction *self);
NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_isValid(const RabbitizerInstruction *self);

/* Instruction examination */


/* Disassembly */

NODISCARD NON_NULL(1) PURE
bool RabbitizerInstruction_mustDisasmAsData(const RabbitizerInstruction *self);

NODISCARD NON_NULL(2) PURE
size_t RabbitizerOperandType_getBufferSize(RabbitizerOperandType operand, const RabbitizerInstruction *instr, size_t immOverrideLength);
NODISCARD NON_NULL(2, 3)
size_t RabbitizerOperandType_disassemble(RabbitizerOperandType operand, const RabbitizerInstruction *instr, char *dst, const char *immOverride, size_t immOverrideLength);

NODISCARD NON_NULL(1) PURE
size_t RabbitizerInstruction_getSizeForBufferOperandsDisasm(const RabbitizerInstruction *self, size_t immOverrideLength);
NON_NULL(1, 2)
size_t RabbitizerInstruction_disassembleOperands(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

NODISCARD NON_NULL(1) PURE
size_t RabbitizerInstruction_getSizeForBufferInstrDisasm(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust);
NON_NULL(1, 2)
size_t RabbitizerInstruction_disassembleInstruction(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

NODISCARD NON_NULL(1) PURE
size_t RabbitizerInstruction_getSizeForBufferDataDisasm(const RabbitizerInstruction *self, int extraLJust);
NON_NULL(1, 2)
size_t RabbitizerInstruction_disassembleAsData(const RabbitizerInstruction *self, char *dst, int extraLJust);

NODISCARD NON_NULL(1) PURE
size_t RabbitizerInstruction_getSizeForBuffer(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust);
NON_NULL(1, 2)
size_t RabbitizerInstruction_disassemble(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

/* Disassembly */


#ifdef __cplusplus
}
#endif

#endif
