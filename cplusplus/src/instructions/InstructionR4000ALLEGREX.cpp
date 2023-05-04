/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionR4000ALLEGREX.hpp"

#include "instructions/RabbitizerInstructionR4000ALLEGREX.h"

using namespace rabbitizer;

InstructionR4000ALLEGREX::InstructionR4000ALLEGREX(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionR4000ALLEGREX_init(&this->instr, word, vram);
    RabbitizerInstructionR4000ALLEGREX_processUniqueId(&this->instr);
}

InstructionR4000ALLEGREX::~InstructionR4000ALLEGREX() {
    RabbitizerInstructionR4000ALLEGREX_destroy(&this->instr);
}
