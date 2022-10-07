/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionRsp.hpp"

#include "instructions/RabbitizerInstructionRsp.h"

using namespace rabbitizer;


InstructionRsp::InstructionRsp(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionRsp_init(&this->instr, word, vram);
    RabbitizerInstructionRsp_processUniqueId(&this->instr);
}

InstructionRsp::~InstructionRsp() {
    RabbitizerInstructionRsp_destroy(&this->instr);
}
