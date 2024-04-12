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
