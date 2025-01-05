/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
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

        Registers::R4000Allegrex::V2D GetR4000Allegrex_p_vs() const;
        Registers::R4000Allegrex::V2D GetR4000Allegrex_p_vt() const;
        Registers::R4000Allegrex::V2D GetR4000Allegrex_p_vd() const;

        Registers::R4000Allegrex::V3D GetR4000Allegrex_t_vs() const;
        Registers::R4000Allegrex::V3D GetR4000Allegrex_t_vt() const;
        Registers::R4000Allegrex::V3D GetR4000Allegrex_t_vd() const;

        Registers::R4000Allegrex::V4D GetR4000Allegrex_q_vs() const;
        Registers::R4000Allegrex::V4D GetR4000Allegrex_q_vt() const;
        Registers::R4000Allegrex::V4D GetR4000Allegrex_q_vd() const;
        Registers::R4000Allegrex::V4D GetR4000Allegrex_q_vt_imm() const;

        Registers::R4000Allegrex::M2x2 GetR4000Allegrex_mp_vs() const;
        Registers::R4000Allegrex::M2x2 GetR4000Allegrex_mp_vt() const;
        Registers::R4000Allegrex::M2x2 GetR4000Allegrex_mp_vd() const;
        Registers::R4000Allegrex::M2x2 GetR4000Allegrex_mp_vs_transpose() const;

        Registers::R4000Allegrex::M3x3 GetR4000Allegrex_mt_vs() const;
        Registers::R4000Allegrex::M3x3 GetR4000Allegrex_mt_vt() const;
        Registers::R4000Allegrex::M3x3 GetR4000Allegrex_mt_vd() const;
        Registers::R4000Allegrex::M3x3 GetR4000Allegrex_mt_vs_transpose() const;

        Registers::R4000Allegrex::M4x4 GetR4000Allegrex_mq_vs() const;
        Registers::R4000Allegrex::M4x4 GetR4000Allegrex_mq_vt() const;
        Registers::R4000Allegrex::M4x4 GetR4000Allegrex_mq_vd() const;
        Registers::R4000Allegrex::M4x4 GetR4000Allegrex_mq_vs_transpose() const;

        Registers::R4000Allegrex::VfpuControl GetR4000Allegrex_cop2cs() const;
        Registers::R4000Allegrex::VfpuControl GetR4000Allegrex_cop2cd() const;

        uint8_t GetR4000Allegrex_pos() const;
        uint8_t GetR4000Allegrex_size() const;
        uint8_t GetR4000Allegrex_size_plus_pos() const;

        uint8_t GetR4000Allegrex_imm3() const;

        uint16_t GetR4000Allegrex_offset14() const;

        uint8_t GetR4000Allegrex_vcmp_cond() const;

        Registers::R4000Allegrex::VConstant GetR4000Allegrex_vconstant() const;

        uint8_t GetR4000Allegrex_power_of_two() const;
        uint8_t GetR4000Allegrex_vfpu_cc_bit() const;
        uint8_t GetR4000Allegrex_bn() const;

        uint16_t GetR4000Allegrex_intfloat16() const;
        uint8_t GetR4000Allegrex_vrot_code() const;

        uint8_t GetR4000Allegrex_rpx() const;
        uint8_t GetR4000Allegrex_rpy() const;
        uint8_t GetR4000Allegrex_rpz() const;
        uint8_t GetR4000Allegrex_rpw() const;

        uint8_t GetR4000Allegrex_wpx() const;
        uint8_t GetR4000Allegrex_wpy() const;
        uint8_t GetR4000Allegrex_wpz() const;
        uint8_t GetR4000Allegrex_wpw() const;
    };
};

#endif
