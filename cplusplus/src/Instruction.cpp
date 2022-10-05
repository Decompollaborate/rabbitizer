/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/Instruction.hpp"

#include <stdexcept>

#include "instructions/RabbitizerInstruction.h"
#include "instructions/RabbitizerInstructionRsp.h"
#include "instructions/RabbitizerInstructionR5900.h"


using namespace rabbitizer;

InstructionBase::InstructionBase() {
}
InstructionBase::~InstructionBase() {
}

RabbitizerInstruction &InstructionBase::getCInstr() {
    return this->instr;
}

/* getters */

constexpr uint8_t InstructionBase::Get_opcode() const {
    return RAB_INSTR_GET_opcode(&this->instr);
}
constexpr uint8_t InstructionBase::Get_sa() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_sa)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'sa' operand.");
    }
    #endif

    return RAB_INSTR_GET_sa(&this->instr);
}
constexpr uint8_t InstructionBase::Get_function() const {
    /*
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_function)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'function' operand.");
    }
    #endif
    */

    return RAB_INSTR_GET_function(&this->instr);
}

constexpr Registers::Cpu::GprO32 InstructionBase::GetO32_rs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprO32>(RAB_INSTR_GET_rs(&this->instr));
}
constexpr Registers::Cpu::GprO32 InstructionBase::GetO32_rt() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rt' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprO32>(RAB_INSTR_GET_rt(&this->instr));
}
constexpr Registers::Cpu::GprO32 InstructionBase::GetO32_rd() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rd' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprO32>(RAB_INSTR_GET_rd(&this->instr));
}

constexpr Registers::Cpu::GprN32 InstructionBase::GetN32_rs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprN32>(RAB_INSTR_GET_rs(&this->instr));
}
constexpr Registers::Cpu::GprN32 InstructionBase::GetN32_rt() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rt' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprN32>(RAB_INSTR_GET_rt(&this->instr));
}
constexpr Registers::Cpu::GprN32 InstructionBase::GetN32_rd() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_rd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rd' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::GprN32>(RAB_INSTR_GET_rd(&this->instr));
}

constexpr Registers::Cpu::Cop0 InstructionBase::Get_cop0d() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_cop0d)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'cop0d' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop0>(RAB_INSTR_GET_cop0d(&this->instr));
}

constexpr uint32_t InstructionBase::Get_instr_index() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_label)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'label' operand.");
    }
    #endif

    return RAB_INSTR_GET_instr_index(&this->instr);
}
constexpr uint16_t InstructionBase::Get_immediate() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_immediate)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'immediate' operand.");
    }
    #endif

    return RAB_INSTR_GET_immediate(&this->instr);
}

constexpr Registers::Cpu::Cop1O32 InstructionBase::GetO32_fs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1O32>(RAB_INSTR_GET_fs(&this->instr));
}
constexpr Registers::Cpu::Cop1O32 InstructionBase::GetO32_ft() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_ft)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'ft' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1O32>(RAB_INSTR_GET_ft(&this->instr));
}
constexpr Registers::Cpu::Cop1O32 InstructionBase::GetO32_fd() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fd' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1O32>(RAB_INSTR_GET_fd(&this->instr));
}

constexpr Registers::Cpu::Cop1N32 InstructionBase::GetN32_fs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N32>(RAB_INSTR_GET_fs(&this->instr));
}
constexpr Registers::Cpu::Cop1N32 InstructionBase::GetN32_ft() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_ft)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'ft' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N32>(RAB_INSTR_GET_ft(&this->instr));
}
constexpr Registers::Cpu::Cop1N32 InstructionBase::GetN32_fd() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fd' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N32>(RAB_INSTR_GET_fd(&this->instr));
}

constexpr Registers::Cpu::Cop1N64 InstructionBase::GetN64_fs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N64>(RAB_INSTR_GET_fs(&this->instr));
}
constexpr Registers::Cpu::Cop1N64 InstructionBase::GetN64_ft() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_ft)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'ft' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N64>(RAB_INSTR_GET_ft(&this->instr));
}
constexpr Registers::Cpu::Cop1N64 InstructionBase::GetN64_fd() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_fd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'fd' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1N64>(RAB_INSTR_GET_fd(&this->instr));
}

constexpr Registers::Cpu::Cop1Control InstructionBase::Get_cop1cs() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_cop1cs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'cop1cs' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop1Control>(RAB_INSTR_GET_cop1cs(&this->instr));
}

constexpr uint8_t InstructionBase::Get_op() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_op)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'op' operand.");
    }
    #endif

    return RAB_INSTR_GET_op(&this->instr);
}

constexpr uint32_t InstructionBase::Get_code() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_code)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'code' operand.");
    }
    #endif

    return RAB_INSTR_GET_code(&this->instr);
}
constexpr uint32_t InstructionBase::Get_code_upper() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_code)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'code' operand.");
    }
    #endif

    return RAB_INSTR_GET_code_upper(&this->instr);
}
constexpr uint32_t InstructionBase::Get_code_lower() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_code)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'code' operand.");
    }
    #endif

    return RAB_INSTR_GET_code_lower(&this->instr);
}

constexpr uint32_t InstructionBase::Get_copraw() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_copraw)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'copraw' operand.");
    }
    #endif

    return RAB_INSTR_GET_copraw(&this->instr);
}

constexpr uint8_t InstructionBase::Get_fmt() const {
    return RAB_INSTR_GET_fmt(&this->instr);
}
constexpr uint8_t InstructionBase::Get_fc() const {
    return RAB_INSTR_GET_fc(&this->instr);
}
constexpr uint8_t InstructionBase::Get_cond() const {
    return RAB_INSTR_GET_cond(&this->instr);
}

constexpr Registers::Cpu::Cop2 InstructionBase::Get_cop2t() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_cop2t)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'cop2t' operand.");
    }
    #endif

    return static_cast<Registers::Cpu::Cop2>(RAB_INSTR_GET_cop2t(&this->instr));
}

constexpr uint8_t InstructionBase::Get_tf() const {
    return RAB_INSTR_GET_tf(&this->instr);
}
constexpr uint8_t InstructionBase::Get_nd() const {
    return RAB_INSTR_GET_nd(&this->instr);
}
constexpr uint8_t InstructionBase::Get_bc_fmt() const {
    return RAB_INSTR_GET_fmt(&this->instr);
}

constexpr uint8_t InstructionBase::Get_stype() const {
    return RAB_INSTR_GET_stype(&this->instr);
}

/* getters */


/* more getters */

constexpr uint32_t InstructionBase::getRaw() const {
    return RabbitizerInstruction_getRaw(&this->instr);
}

constexpr InstrId::UniqueId InstructionBase::getUniqueId() const {
    return static_cast<InstrId::UniqueId>(this->instr.uniqueId);
}
constexpr uint32_t InstructionBase::getVram() const {
    return this->instr.vram;
}
constexpr bool InstructionBase::isInHandwrittenFunction() const {
    return this->instr.inHandwrittenFunction;
}

constexpr int32_t InstructionBase::getProcessedImmediate() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_immediate)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'immediate' operand.");
    }
    #endif

    return RabbitizerInstruction_getProcessedImmediate(&this->instr);
}
constexpr uint32_t InstructionBase::getInstrIndexAsVram() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_label)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'label' operand.");
    }
    #endif

    return RabbitizerInstruction_getInstrIndexAsVram(&this->instr);
}

constexpr int32_t InstructionBase::getBranchOffset() const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_branch_target_label)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'branch_target_label' operand.");
    }
    #endif

    return RabbitizerInstruction_getBranchOffset(&this->instr);
}
constexpr int32_t InstructionBase::getGenericBranchOffset(uint32_t currentVram) const {
    #ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::cpu_branch_target_label) && !hasOperandAlias(OperandType::cpu_label)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have either 'branch_target_label' or 'label' operands.");
    }
    #endif

    return RabbitizerInstruction_getGenericBranchOffset(&this->instr, currentVram);
}


std::string InstructionBase::getOpcodeName() const {
    return InstrId::getOpcodeName(getUniqueId());
}

/* more getters */


/* */

void InstructionBase::blankOut() {
    RabbitizerInstruction_blankOut(&this->instr);
}


/* Instruction examination */

constexpr bool InstructionBase::isImplemented() const {
    return RabbitizerInstruction_isImplemented(&this->instr);
}
constexpr bool InstructionBase::isLikelyHandwritten() const {
    return RabbitizerInstruction_isLikelyHandwritten(&this->instr);
}
constexpr bool InstructionBase::isNop() const {
    return RabbitizerInstruction_isNop(&this->instr);
}
constexpr bool InstructionBase::isUnconditionalBranch() const {
    return RabbitizerInstruction_isUnconditionalBranch(&this->instr);
}
constexpr bool InstructionBase::isJrRa() const {
    return RabbitizerInstruction_isJrRa(&this->instr);
}
constexpr bool InstructionBase::isJrNotRa() const {
    return RabbitizerInstruction_isJrNotRa(&this->instr);
}
constexpr bool InstructionBase::hasDelaySlot() const {
    return RabbitizerInstruction_hasDelaySlot(&this->instr);
}

std::string InstructionBase::mapInstrToType() const {
    return std::string(RabbitizerInstruction_mapInstrToType(&this->instr));
}

constexpr bool InstructionBase::sameOpcode(const InstructionBase &other) const {
    return RabbitizerInstruction_sameOpcode(&this->instr, &other.instr);
}
constexpr bool InstructionBase::sameOpcodeButDifferentArguments(const InstructionBase &other) const {
    return RabbitizerInstruction_sameOpcodeButDifferentArguments(&this->instr, &other.instr);
}

constexpr bool InstructionBase::hasOperand(OperandType operand) const {
    return RabbitizerInstruction_hasOperand(&this->instr, static_cast<RabbitizerOperandType>(operand));
}
constexpr bool InstructionBase::hasOperandAlias(OperandType operand) const {
    return RabbitizerInstruction_hasOperandAlias(&this->instr, static_cast<RabbitizerOperandType>(operand));
}

constexpr uint32_t InstructionBase::getValidBits() const {
    return RabbitizerInstruction_getValidBits(&this->instr);
}
constexpr bool InstructionBase::isValid() const {
    return RabbitizerInstruction_isValid(&this->instr);
}

/* Instruction examination */


/* Instruction descriptor */

constexpr bool InstructionBase::isUnknownType() const {
    return RabbitizerInstrDescriptor_isUnknownType(this->instr.descriptor);
}
constexpr bool InstructionBase::isJType() const {
    return RabbitizerInstrDescriptor_isJType(this->instr.descriptor);
}
constexpr bool InstructionBase::isIType() const {
    return RabbitizerInstrDescriptor_isIType(this->instr.descriptor);
}
constexpr bool InstructionBase::isRType() const {
    return RabbitizerInstrDescriptor_isRType(this->instr.descriptor);
}
constexpr bool InstructionBase::isRegimmType() const {
    return RabbitizerInstrDescriptor_isRegimmType(this->instr.descriptor);
}

// TODO
// constexpr RabbitizerInstrSuffix instrSuffix() const;

constexpr bool InstructionBase::isBranch() const {
    return RabbitizerInstrDescriptor_isBranch(this->instr.descriptor);
}
constexpr bool InstructionBase::isBranchLikely() const {
    return RabbitizerInstrDescriptor_isBranchLikely(this->instr.descriptor);
}
constexpr bool InstructionBase::isJump() const {
    return RabbitizerInstrDescriptor_isJump(this->instr.descriptor);
}
constexpr bool InstructionBase::isTrap() const {
    return RabbitizerInstrDescriptor_isTrap(this->instr.descriptor);
}

constexpr bool InstructionBase::isFloat() const {
    return RabbitizerInstrDescriptor_isFloat(this->instr.descriptor);
}
constexpr bool InstructionBase::isDouble() const {
    return RabbitizerInstrDescriptor_isDouble(this->instr.descriptor);
}

constexpr bool InstructionBase::isUnsigned() const {
    return RabbitizerInstrDescriptor_isUnsigned(this->instr.descriptor);
}

constexpr bool InstructionBase::modifiesRt() const {
    return RabbitizerInstrDescriptor_modifiesRt(this->instr.descriptor);
}
constexpr bool InstructionBase::modifiesRd() const {
    return RabbitizerInstrDescriptor_modifiesRd(this->instr.descriptor);
}

constexpr bool InstructionBase::notEmitedByCompilers() const {
    return RabbitizerInstrDescriptor_notEmitedByCompilers(this->instr.descriptor);
}

constexpr bool InstructionBase::canBeHi() const {
    return RabbitizerInstrDescriptor_canBeHi(this->instr.descriptor);
}
constexpr bool InstructionBase::canBeLo() const {
    return RabbitizerInstrDescriptor_canBeLo(this->instr.descriptor);
}
constexpr bool InstructionBase::doesLink() const {
    return RabbitizerInstrDescriptor_doesLink(this->instr.descriptor);
}
constexpr bool InstructionBase::doesDereference() const {
    return RabbitizerInstrDescriptor_doesDereference(this->instr.descriptor);
}
constexpr bool InstructionBase::doesLoad() const {
    return RabbitizerInstrDescriptor_doesLoad(this->instr.descriptor);
}
constexpr bool InstructionBase::doesStore() const {
    return RabbitizerInstrDescriptor_doesStore(this->instr.descriptor);
}
constexpr bool InstructionBase::maybeIsMove() const {
    return RabbitizerInstrDescriptor_maybeIsMove(this->instr.descriptor);
}

constexpr bool InstructionBase::isPseudo() const {
    return RabbitizerInstrDescriptor_isPseudo(this->instr.descriptor);
}

/* Instruction descriptor */


/* Disassembly */

constexpr bool InstructionBase::mustDisasmAsData() const {
    return RabbitizerInstruction_mustDisasmAsData(&this->instr);
}

#if 0
constexpr size_t RabbitizerInstruction_disassembleOperands(char *dst, const char *immOverride, size_t immOverrideLength) const {
}

constexpr size_t RabbitizerInstruction_disassembleInstruction(char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) const {
}

constexpr size_t RabbitizerInstruction_disassembleAsData(char *dst, int extraLJust) const {
}

constexpr size_t RabbitizerInstruction_disassemble(char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust) const {
}
#endif




InstructionCpu::InstructionCpu(uint32_t word, uint32_t vram)
    : InstructionBase() {
    RabbitizerInstruction_init(&this->instr, word, vram);
    RabbitizerInstruction_processUniqueId(&this->instr);
}

InstructionCpu::~InstructionCpu() {
    RabbitizerInstruction_destroy(&this->instr);
}


InstructionRsp::InstructionRsp(uint32_t word, uint32_t vram)
    : InstructionBase() {
    RabbitizerInstructionRsp_init(&this->instr, word, vram);
    RabbitizerInstructionRsp_processUniqueId(&this->instr);
}

InstructionRsp::~InstructionRsp() {
    RabbitizerInstructionRsp_destroy(&this->instr);
}


InstructionR5900::InstructionR5900(uint32_t word, uint32_t vram)
    : InstructionBase() {
    RabbitizerInstructionR5900_init(&this->instr, word, vram);
    RabbitizerInstructionR5900_processUniqueId(&this->instr);
}

InstructionR5900::~InstructionR5900() {
    RabbitizerInstructionR5900_destroy(&this->instr);
}

