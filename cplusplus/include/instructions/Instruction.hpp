/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_HPP
#define RABBITIZER_INSTRUCTION_HPP
#pragma once

#include <string>

#include "instructions/RabbitizerInstruction.h"

#include "instructions/Registers.hpp"
#include "instructions/OperandType.hpp"


namespace rabbitizer {
    class InstructionBase {
    protected:
        RabbitizerInstruction instr;

        InstructionBase();
        virtual ~InstructionBase();

    public:
        /**
         * Returns a reference to the inner RabbitizerInstruction.
         * It is recommended to not mess with it unless you know what you are doing.
         */
        RabbitizerInstruction &getCInstr();

        /* getters */

        constexpr uint8_t Get_opcode() const;
        constexpr uint8_t Get_sa() const;
        constexpr uint8_t Get_function() const;

        constexpr Registers::Cpu::GprO32 GetO32_rs() const;
        constexpr Registers::Cpu::GprO32 GetO32_rt() const;
        constexpr Registers::Cpu::GprO32 GetO32_rd() const;

        constexpr Registers::Cpu::GprN32 GetN32_rs() const;
        constexpr Registers::Cpu::GprN32 GetN32_rt() const;
        constexpr Registers::Cpu::GprN32 GetN32_rd() const;

        constexpr Registers::Cpu::Cop0 Get_cop0d() const;

        constexpr uint32_t Get_instr_index() const;
        constexpr uint16_t Get_immediate() const;

        constexpr Registers::Cpu::Cop1O32 GetO32_fs() const;
        constexpr Registers::Cpu::Cop1O32 GetO32_ft() const;
        constexpr Registers::Cpu::Cop1O32 GetO32_fd() const;

        constexpr Registers::Cpu::Cop1N32 GetN32_fs() const;
        constexpr Registers::Cpu::Cop1N32 GetN32_ft() const;
        constexpr Registers::Cpu::Cop1N32 GetN32_fd() const;

        constexpr Registers::Cpu::Cop1N64 GetN64_fs() const;
        constexpr Registers::Cpu::Cop1N64 GetN64_ft() const;
        constexpr Registers::Cpu::Cop1N64 GetN64_fd() const;

        constexpr Registers::Cpu::Cop1Control Get_cop1cs() const;

        constexpr uint8_t Get_op() const;

        constexpr uint32_t Get_code() const;
        constexpr uint32_t Get_code_upper() const;
        constexpr uint32_t Get_code_lower() const;

        constexpr uint32_t Get_copraw() const;

        constexpr uint8_t Get_fmt() const;
        constexpr uint8_t Get_fc() const;
        constexpr uint8_t Get_cond() const;

        constexpr Registers::Cpu::Cop2 Get_cop2t() const;

        constexpr uint8_t Get_tf() const;
        constexpr uint8_t Get_nd() const;
        constexpr uint8_t Get_bc_fmt() const;

        constexpr uint8_t Get_stype() const;

        /* getters */


        /* more getters */

        constexpr uint32_t getRaw() const;

        constexpr InstrId::UniqueId getUniqueId() const;
        constexpr uint32_t getVram() const;
        constexpr bool isInHandwrittenFunction() const;

        constexpr int32_t getProcessedImmediate() const;
        constexpr uint32_t getInstrIndexAsVram() const;

        constexpr int32_t getBranchOffset() const;
        constexpr int32_t getGenericBranchOffset(uint32_t currentVram) const;


        std::string getOpcodeName() const;

        /* more getters */


        /* */

        void blankOut();


        /* Instruction examination */

        constexpr bool isImplemented() const;
        constexpr bool isLikelyHandwritten() const;
        constexpr bool isNop() const;
        constexpr bool isUnconditionalBranch() const;
        constexpr bool isJrRa() const;
        constexpr bool isJrNotRa() const;
        constexpr bool hasDelaySlot() const;

        std::string mapInstrToType() const;

        constexpr bool sameOpcode(const InstructionBase &other) const;
        constexpr bool sameOpcodeButDifferentArguments(const InstructionBase &other) const;

        constexpr bool hasOperand(OperandType operand) const;
        constexpr bool hasOperandAlias(OperandType operand) const;

        constexpr uint32_t getValidBits() const;
        constexpr bool isValid() const;

        /* Instruction examination */


        /* Instruction descriptor */

        constexpr bool isUnknownType() const;
        constexpr bool isJType() const;
        constexpr bool isIType() const;
        constexpr bool isRType() const;
        constexpr bool isRegimmType() const;

        // TODO
        // constexpr RabbitizerInstrSuffix instrSuffix() const;

        constexpr bool isBranch() const;
        constexpr bool isBranchLikely() const;
        constexpr bool isJump() const;
        constexpr bool isTrap() const;

        constexpr bool isFloat() const;
        constexpr bool isDouble() const;

        constexpr bool isUnsigned() const;

        constexpr bool modifiesRt() const;
        constexpr bool modifiesRd() const;

        constexpr bool notEmitedByCompilers() const;

        constexpr bool canBeHi() const;
        constexpr bool canBeLo() const;
        constexpr bool doesLink() const;
        constexpr bool doesDereference() const;
        constexpr bool doesLoad() const;
        constexpr bool doesStore() const;
        constexpr bool maybeIsMove() const;

        constexpr bool isPseudo() const;

        /* Instruction descriptor */


        /* Disassembly */

        constexpr bool mustDisasmAsData() const;

        #if 0
        constexpr size_t RabbitizerInstruction_disassembleOperands(char *dst, const char *immOverride, size_t immOverrideLength) const;

        constexpr size_t RabbitizerInstruction_disassembleInstruction(char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) const;

        constexpr size_t RabbitizerInstruction_disassembleAsData(char *dst, int extraLJust) const;

        constexpr size_t RabbitizerInstruction_disassemble(char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) const;
        #endif

        /* Disassembly */

    };


    class InstructionCpu : InstructionBase {
    public:
        InstructionCpu(uint32_t word, uint32_t vram);
        virtual ~InstructionCpu();
    };

    class InstructionRsp : InstructionBase {
    public:
        InstructionRsp(uint32_t word, uint32_t vram);
        virtual ~InstructionRsp();

        // TODO: more methods
    };

    class InstructionR5900 : InstructionBase {
    public:
        InstructionR5900(uint32_t word, uint32_t vram);
        virtual ~InstructionR5900();

        // TODO: more methods
    };
};

#endif
