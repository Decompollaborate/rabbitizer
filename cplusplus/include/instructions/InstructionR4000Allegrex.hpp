/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R4000ALLEGREX_HPP
#define RABBITIZER_INSTRUCTION_R4000ALLEGREX_HPP
#pragma once

#include "InstructionBase.hpp"


namespace rabbitizer {
    class InstructionR4000Allegrex : public InstructionBase {
    public:
        InstructionR4000Allegrex(uint32_t word, uint32_t vram);
        virtual ~InstructionR4000Allegrex();

        uint8_t GetR4000Allegrex_pos() const;
        uint8_t GetR4000Allegrex_size() const;
        uint8_t GetR4000Allegrex_size_plus_pos() const;
    };
};

#endif
