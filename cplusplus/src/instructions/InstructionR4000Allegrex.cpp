/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstructionR4000Allegrex.hpp"

#include <stdexcept>

#include "instructions/RabbitizerInstructionR4000Allegrex.h"

using namespace rabbitizer;

InstructionR4000Allegrex::InstructionR4000Allegrex(uint32_t word, uint32_t vram) : InstructionBase() {
    RabbitizerInstructionR4000Allegrex_init(&this->instr, word, vram);
    RabbitizerInstructionR4000Allegrex_processUniqueId(&this->instr);
}

InstructionR4000Allegrex::~InstructionR4000Allegrex() {
    RabbitizerInstructionR4000Allegrex_destroy(&this->instr);
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vt_imm() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vt_imm)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vt_imm' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vt_imm(&this->instr));
}

Registers::R4000Allegrex::S InstructionR4000Allegrex::GetR4000Allegrex_s_vd_imm() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vd_imm)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vd_imm' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::S>(RAB_INSTR_R4000ALLEGREX_GET_vd_imm(&this->instr));
}

Registers::R4000Allegrex::V2D InstructionR4000Allegrex::GetR4000Allegrex_p_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_p_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'p_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V2D>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::V2D InstructionR4000Allegrex::GetR4000Allegrex_p_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_s_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 's_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V2D>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::V2D InstructionR4000Allegrex::GetR4000Allegrex_p_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_p_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'p_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V2D>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::V3D InstructionR4000Allegrex::GetR4000Allegrex_t_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_t_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 't_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V3D>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::V3D InstructionR4000Allegrex::GetR4000Allegrex_t_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_t_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 't_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V3D>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::V3D InstructionR4000Allegrex::GetR4000Allegrex_t_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_t_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 't_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V3D>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::V4D InstructionR4000Allegrex::GetR4000Allegrex_q_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_q_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'q_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V4D>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::V4D InstructionR4000Allegrex::GetR4000Allegrex_q_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_q_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'q_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V4D>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::V4D InstructionR4000Allegrex::GetR4000Allegrex_q_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_q_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'q_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V4D>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::V4D InstructionR4000Allegrex::GetR4000Allegrex_q_vt_imm() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_q_vt_imm)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'q_vt_imm' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::V4D>(RAB_INSTR_R4000ALLEGREX_GET_vt_imm(&this->instr));
}

Registers::R4000Allegrex::M2x2 InstructionR4000Allegrex::GetR4000Allegrex_mp_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mp_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mp_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M2x2>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::M2x2 InstructionR4000Allegrex::GetR4000Allegrex_mp_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mp_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mp_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M2x2>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::M2x2 InstructionR4000Allegrex::GetR4000Allegrex_mp_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mp_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mp_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M2x2>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::M2x2 InstructionR4000Allegrex::GetR4000Allegrex_mp_vs_transpose() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mp_vs_transpose)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mp_vs_transpose' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M2x2>(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(&this->instr));
}

Registers::R4000Allegrex::M3x3 InstructionR4000Allegrex::GetR4000Allegrex_mt_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mt_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mt_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M3x3>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::M3x3 InstructionR4000Allegrex::GetR4000Allegrex_mt_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mt_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mt_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M3x3>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::M3x3 InstructionR4000Allegrex::GetR4000Allegrex_mt_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mt_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mt_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M3x3>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::M3x3 InstructionR4000Allegrex::GetR4000Allegrex_mt_vs_transpose() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mt_vs_transpose)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mt_vs_transpose' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M3x3>(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(&this->instr));
}

Registers::R4000Allegrex::M4x4 InstructionR4000Allegrex::GetR4000Allegrex_mq_vs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mq_vs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mq_vs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M4x4>(RAB_INSTR_R4000ALLEGREX_GET_vs(&this->instr));
}

Registers::R4000Allegrex::M4x4 InstructionR4000Allegrex::GetR4000Allegrex_mq_vt() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mq_vt)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mq_vt' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M4x4>(RAB_INSTR_R4000ALLEGREX_GET_vt(&this->instr));
}

Registers::R4000Allegrex::M4x4 InstructionR4000Allegrex::GetR4000Allegrex_mq_vd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mq_vd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mq_vd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M4x4>(RAB_INSTR_R4000ALLEGREX_GET_vd(&this->instr));
}

Registers::R4000Allegrex::M4x4 InstructionR4000Allegrex::GetR4000Allegrex_mq_vs_transpose() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_mq_vs_transpose)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'mq_vs_transpose' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::M4x4>(RAB_INSTR_R4000ALLEGREX_GET_vs_transpose(&this->instr));
}

Registers::R4000Allegrex::VfpuControl InstructionR4000Allegrex::GetR4000Allegrex_cop2cs() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_cop2cs)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'cop2cs' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::VfpuControl>(RAB_INSTR_R4000ALLEGREX_GET_cop2cs(&this->instr));
}

Registers::R4000Allegrex::VfpuControl InstructionR4000Allegrex::GetR4000Allegrex_cop2cd() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_cop2cd)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'cop2cd' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::VfpuControl>(RAB_INSTR_R4000ALLEGREX_GET_cop2cd(&this->instr));
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_pos() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_pos)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'pos' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_pos(&this->instr);
}
uint8_t InstructionR4000Allegrex::GetR4000Allegrex_size() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_size)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'size' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_size(&this->instr);
}
uint8_t InstructionR4000Allegrex::GetR4000Allegrex_size_plus_pos() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_size_plus_pos)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'size_plus_pos' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_size_plus_pos(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_imm3() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_imm3)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'imm3' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_imm3(&this->instr);
}

uint16_t InstructionR4000Allegrex::GetR4000Allegrex_offset14() const {
#if 0
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_offset14)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'offset14' operand.");
    }
#endif
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_offset14(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_vcmp_cond() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_vcmp_cond)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'vcmp_cond' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_vcmp_cond(&this->instr);
}

Registers::R4000Allegrex::VConstant InstructionR4000Allegrex::GetR4000Allegrex_vconstant() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_vconstant)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'vconstant' operand.");
    }
#endif

    return static_cast<Registers::R4000Allegrex::VConstant>(RAB_INSTR_R4000ALLEGREX_GET_vconstant(&this->instr));
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_power_of_two() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_power_of_two)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'power_of_two' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_power_of_two(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_vfpu_cc_bit() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_vfpu_cc_bit)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'vfpu_cc_bit' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_vfpu_cc_bit(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_bn() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_bn)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'bn' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_bn(&this->instr);
}

uint16_t InstructionR4000Allegrex::GetR4000Allegrex_intfloat16() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_int16) && !hasOperandAlias(OperandType::r4000allegrex_float16)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'intfloat16' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_intfloat16(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_vrot_code() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_p_vrot_code) &&
        !hasOperandAlias(OperandType::r4000allegrex_t_vrot_code) &&
        !hasOperandAlias(OperandType::r4000allegrex_q_vrot_code)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'vrot_code' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_vrot_code(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_rpx() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_rpx)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rpx' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_rpx(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_rpy() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_rpy)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rpy' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_rpy(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_rpz() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_rpz)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rpz' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_rpz(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_rpw() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_rpw)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'rpw' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_rpw(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_wpx() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_wpx)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'wpx' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_wpx(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_wpy() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_wpy)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'wpy' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_wpy(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_wpz() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_wpz)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'wpz' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_wpz(&this->instr);
}

uint8_t InstructionR4000Allegrex::GetR4000Allegrex_wpw() const {
#ifdef RAB_SANITY_CHECKS
    if (!hasOperandAlias(OperandType::r4000allegrex_wpw)) {
        // TODO: make a rabbitizer exception class
        throw std::runtime_error("Instruction '" + getOpcodeName() + "' does not have 'wpw' operand.");
    }
#endif

    return RAB_INSTR_R4000ALLEGREX_GET_wpw(&this->instr);
}
