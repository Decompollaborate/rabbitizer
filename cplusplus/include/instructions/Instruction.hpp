/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_HPP
#define RABBITIZER_INSTRUCTION_HPP
#pragma once

#include <string>
#include <string_view>

#include "instructions/RabbitizerInstruction.h"

#include "instructions/Registers.hpp"
#include "instructions/OperandType.hpp"
#include "instructions/InstrId.hpp"


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

        uint8_t Get_opcode() const;
        uint8_t Get_sa() const;
        uint8_t Get_function() const;

        Registers::Cpu::GprO32 GetO32_rs() const;
        Registers::Cpu::GprO32 GetO32_rt() const;
        Registers::Cpu::GprO32 GetO32_rd() const;

        Registers::Cpu::GprN32 GetN32_rs() const;
        Registers::Cpu::GprN32 GetN32_rt() const;
        Registers::Cpu::GprN32 GetN32_rd() const;

        Registers::Cpu::Cop0 Get_cop0d() const;

        uint32_t Get_instr_index() const;
        uint16_t Get_immediate() const;

        Registers::Cpu::Cop1O32 GetO32_fs() const;
        Registers::Cpu::Cop1O32 GetO32_ft() const;
        Registers::Cpu::Cop1O32 GetO32_fd() const;

        Registers::Cpu::Cop1N32 GetN32_fs() const;
        Registers::Cpu::Cop1N32 GetN32_ft() const;
        Registers::Cpu::Cop1N32 GetN32_fd() const;

        Registers::Cpu::Cop1N64 GetN64_fs() const;
        Registers::Cpu::Cop1N64 GetN64_ft() const;
        Registers::Cpu::Cop1N64 GetN64_fd() const;

        Registers::Cpu::Cop1Control Get_cop1cs() const;

        uint8_t Get_op() const;

        uint32_t Get_code() const;
        uint32_t Get_code_upper() const;
        uint32_t Get_code_lower() const;

        uint32_t Get_copraw() const;

        uint8_t Get_fmt() const;
        uint8_t Get_fc() const;
        uint8_t Get_cond() const;

        Registers::Cpu::Cop2 Get_cop2t() const;

        uint8_t Get_tf() const;
        uint8_t Get_nd() const;
        uint8_t Get_bc_fmt() const;

        uint8_t Get_stype() const;

        /* getters */


        /* more getters */

        uint32_t getRaw() const;

        InstrId::UniqueId getUniqueId() const;
        uint32_t getVram() const;
        bool isInHandwrittenFunction() const;

        int32_t getProcessedImmediate() const;
        uint32_t getInstrIndexAsVram() const;

        int32_t getBranchOffset() const;
        int32_t getGenericBranchOffset(uint32_t currentVram) const;


        std::string getOpcodeName() const;

        /* more getters */


        /* */

        void blankOut();


        /* Instruction examination */

        bool isImplemented() const;
        bool isLikelyHandwritten() const;
        bool isNop() const;
        bool isUnconditionalBranch() const;
        bool isJrRa() const;
        bool isJrNotRa() const;
        bool hasDelaySlot() const;

        std::string mapInstrToType() const;

        bool sameOpcode(const InstructionBase &other) const;
        bool sameOpcodeButDifferentArguments(const InstructionBase &other) const;

        bool hasOperand(OperandType operand) const;
        bool hasOperandAlias(OperandType operand) const;

        uint32_t getValidBits() const;
        bool isValid() const;

        /* Instruction examination */


        /* Instruction descriptor */

        bool isUnknownType() const;
        bool isJType() const;
        bool isIType() const;
        bool isRType() const;
        bool isRegimmType() const;

        // TODO
        // RabbitizerInstrSuffix instrSuffix() const;

        bool isBranch() const;
        bool isBranchLikely() const;
        bool isJump() const;
        bool isTrap() const;

        bool isFloat() const;
        bool isDouble() const;

        bool isUnsigned() const;

        bool modifiesRt() const;
        bool modifiesRd() const;

        bool notEmitedByCompilers() const;

        bool canBeHi() const;
        bool canBeLo() const;
        bool doesLink() const;
        bool doesDereference() const;
        bool doesLoad() const;
        bool doesStore() const;
        bool maybeIsMove() const;

        bool isPseudo() const;

        /* Instruction descriptor */


        /* Disassembly */

        bool mustDisasmAsData() const;

        #if 0
        size_t RabbitizerInstruction_disassembleOperands(char *dst, const char *immOverride, size_t immOverrideLength) const;

        size_t RabbitizerInstruction_disassembleInstruction(char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) const;

        size_t RabbitizerInstruction_disassembleAsData(char *dst, int extraLJust) const;
        #endif

        std::string disassemble(bool useImmOverride, std::string_view immOverride, int extraLJust) const;

        /* Disassembly */
    };


    class InstructionCpu : public InstructionBase {
    public:
        InstructionCpu(uint32_t word, uint32_t vram);
        virtual ~InstructionCpu();
    };

    class InstructionRsp : public InstructionBase {
    public:
        InstructionRsp(uint32_t word, uint32_t vram);
        virtual ~InstructionRsp();

        // TODO: more methods
    };

    class InstructionR5900 : public InstructionBase {
    public:
        InstructionR5900(uint32_t word, uint32_t vram);
        virtual ~InstructionR5900();

        // TODO: more methods
    };
};

#endif
