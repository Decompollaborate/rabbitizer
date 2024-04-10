/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionAllegrex.hpp"

#include <stdexcept>

#include "instructions/RabbitizerInstructionAllegrex.h"

using namespace rabbitizer;

InstructionAllegrex::InstructionAllegrex(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionAllegrex_init(&this->instr, word, vram);
    RabbitizerInstructionAllegrex_processUniqueId(&this->instr);
}

InstructionAllegrex::~InstructionAllegrex() {
    RabbitizerInstructionAllegrex_destroy(&this->instr);
}
