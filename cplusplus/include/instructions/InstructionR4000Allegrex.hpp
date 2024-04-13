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

        Registers::R4000Allegrex::S GetR4000Allegrex_s_vs() const;
        Registers::R4000Allegrex::S GetR4000Allegrex_s_vt() const;
        Registers::R4000Allegrex::S GetR4000Allegrex_s_vd() const;
        Registers::R4000Allegrex::S GetR4000Allegrex_s_vt_imm() const;
        Registers::R4000Allegrex::S GetR4000Allegrex_s_vd_imm() const;

        Registers::R4000Allegrex::V2D GetR4000Allegrex_q_vs() const;
        Registers::R4000Allegrex::V2D GetR4000Allegrex_q_vt() const;
        Registers::R4000Allegrex::V2D GetR4000Allegrex_q_vd() const;
        Registers::R4000Allegrex::V2D GetR4000Allegrex_q_vt_imm() const;

        uint8_t GetR4000Allegrex_pos() const;
        uint8_t GetR4000Allegrex_size() const;
        uint8_t GetR4000Allegrex_size_plus_pos() const;

        uint8_t GetR4000Allegrex_imm3() const;
    };
};

#endif
