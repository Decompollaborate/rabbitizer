/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_ALLEGREX_HPP
#define RABBITIZER_INSTRUCTION_ALLEGREX_HPP
#pragma once

#include "InstructionBase.hpp"


namespace rabbitizer {
    class InstructionAllegrex : public InstructionBase {
    public:
        InstructionAllegrex(uint32_t word, uint32_t vram);
        virtual ~InstructionAllegrex();
    };
};


#endif
