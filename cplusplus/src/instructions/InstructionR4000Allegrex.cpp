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
