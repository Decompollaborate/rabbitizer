/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R5900_HPP
#define RABBITIZER_INSTRUCTION_R5900_HPP
#pragma once

#include "InstructionBase.hpp"


namespace rabbitizer {
    class InstructionR5900 : public InstructionBase {
    public:
        InstructionR5900(uint32_t word, uint32_t vram);
        virtual ~InstructionR5900();
    };
};


#endif
