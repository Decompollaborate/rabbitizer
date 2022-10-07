/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_RSP_HPP
#define RABBITIZER_INSTRUCTION_RSP_HPP
#pragma once

#include "InstructionBase.hpp"


namespace rabbitizer {
    class InstructionRsp : public InstructionBase {
    public:
        InstructionRsp(uint32_t word, uint32_t vram);
        virtual ~InstructionRsp();
    };
};


#endif
