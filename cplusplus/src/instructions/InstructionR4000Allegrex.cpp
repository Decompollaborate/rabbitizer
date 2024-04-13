/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionR4000Allegrex.hpp"

#include <stdexcept>

#include "instructions/RabbitizerInstructionR4000Allegrex.h"

using namespace rabbitizer;

InstructionR4000Allegrex::InstructionR4000Allegrex(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionR4000Allegrex_init(&this->instr, word, vram);
    RabbitizerInstructionR4000Allegrex_processUniqueId(&this->instr);
}

InstructionR4000Allegrex::~InstructionR4000Allegrex() {
    RabbitizerInstructionR4000Allegrex_destroy(&this->instr);
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vt_imm() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vt_imm)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vt_imm' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vt_imm(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vd_imm() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vd_imm)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vd_imm' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vd_imm(&this->instr));
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_pos() const {
    return RAB_INSTR_R4000ALLEGREX_GET_pos(&this->instr);
}
uint8_t InstructionR4000Allegrex::GetR4000Allegrex_size() const {
    return RAB_INSTR_R4000ALLEGREX_GET_size(&this->instr);
}
uint8_t InstructionR4000Allegrex::GetR4000Allegrex_size_plus_pos() const {
    return RAB_INSTR_R4000ALLEGREX_GET_size_plus_pos(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_imm3() const {
    return RAB_INSTR_R4000ALLEGREX_GET_imm3(&this->instr);
}
