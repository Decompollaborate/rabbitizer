/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionR5900.hpp"

#include "instructions/RabbitizerInstructionR5900.h"

using namespace rabbitizer;


InstructionR5900::InstructionR5900(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionR5900_init(&this->instr, word, vram);
    RabbitizerInstructionR5900_processUniqueId(&this->instr);
}

InstructionR5900::~InstructionR5900() {
    RabbitizerInstructionR5900_destroy(&this->instr);
}
