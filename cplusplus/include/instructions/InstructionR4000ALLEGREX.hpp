/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R4000ALLEGREX_HPP
#define RABBITIZER_INSTRUCTION_R4000ALLEGREX_HPP
#pragma once

#include "InstructionBase.hpp"


namespace rabbitizer {
    class InstructionR4000ALLEGREX : public InstructionBase {
    public:
        InstructionR4000ALLEGREX(uint32_t word, uint32_t vram);
        virtual ~InstructionR4000ALLEGREX();
    };
};


#endif
