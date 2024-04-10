/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionAllegrex.h"
#include "common/RabbitizerConfig.h"

#define RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, ...)    \
    case (caseBits):                                            \
        self->uniqueId = RABBITIZER_INSTR_ID_##prefix##_##name; \
        break;
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, caseBits, name, altname, ...) \
    RABBITIZER_DEF_INSTR_ID(prefix, caseBits, name, __VA_ARGS__)

void RabbitizerInstructionAllegrex_processUniqueId_Normal(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_NORMAL;

    switch (opcode) {
#include "tables/instr_id/allegrex/allegrex_normal.inc"

        default:
            RabbitizerInstruction_processUniqueId_Normal(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL;

    switch (function) {
#include "tables/instr_id/allegrex/allegrex_special.inc"

        case 0x02:
            RabbitizerInstructionAllegrex_processUniqueId_Special_Rs(self);
            fetchDescriptor = false;
            break;

        case 0x06:
            RabbitizerInstructionAllegrex_processUniqueId_Special_Sa(self);
            fetchDescriptor = false;
            break;

        default:
            RabbitizerInstruction_processUniqueId_Special(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special_Rs(RabbitizerInstruction *self) {
    uint32_t rs = RAB_INSTR_GET_rs(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rs(self->_mandatorybits, rs);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL_RS;

    switch (rs) {
#include "tables/instr_id/allegrex/allegrex_special_rs.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special_Sa(RabbitizerInstruction *self) {
    uint32_t sa = RAB_INSTR_GET_sa(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_sa(self->_mandatorybits, sa);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL_SA;

    switch (sa) {
#include "tables/instr_id/allegrex/allegrex_special_sa.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Regimm(RabbitizerInstruction *self) {
    uint32_t rt = RAB_INSTR_GET_rt(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_rt(self->_mandatorybits, rt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_REGIMM;

    switch (rt) {
#include "tables/instr_id/allegrex/allegrex_regimm.inc"

        default:
            RabbitizerInstruction_processUniqueId_Regimm(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special2(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL2;

    switch (function) {
#include "tables/instr_id/allegrex/allegrex_special2.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special3(RabbitizerInstruction *self) {
    uint32_t function = RAB_INSTR_GET_function(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_function(self->_mandatorybits, function);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL3;

    switch (function) {
#include "tables/instr_id/allegrex/allegrex_special3.inc"

        case 0x20:
            RabbitizerInstructionAllegrex_processUniqueId_Special3_Bshfl(self);
            fetchDescriptor = false;
            break;
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Special3_Bshfl(RabbitizerInstruction *self) {
    uint32_t sa = RAB_INSTR_GET_sa(self);
    bool fetchDescriptor = true;

    self->_mandatorybits = RAB_INSTR_PACK_sa(self->_mandatorybits, sa);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_SPECIAL3_BSHFL;

    switch (sa) {
#include "tables/instr_id/allegrex/allegrex_special3_bshfl.inc"
    }

    if (fetchDescriptor) {
        self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
    }
}

void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor0(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_COP0;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_cop0.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor1(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_COP1;

    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_cop0.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor2(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_COP2;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_cop2.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor3(RabbitizerInstruction *self) {
    uint32_t fmt = RAB_INSTR_GET_fmt(self);

    self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_COP3;
    self->_handwrittenCategory = true;

    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_cop3.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu0(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU0;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu0.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu1(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU1;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu1.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu3(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU3;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu3.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu4(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU4;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu4.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu5(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU5;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu5.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu6(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU6;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu6.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

void RabbitizerInstructionAllegrex_processUniqueId_Vfpu7(RabbitizerInstruction *self) {
    //! TODO
    // uint32_t fmt = RAB_INSTR_GET_fmt(self);

    // self->_mandatorybits = RAB_INSTR_PACK_fmt(self->_mandatorybits, fmt);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_VFPU7;

#if 0
    switch (fmt) {
#include "tables/instr_id/allegrex/allegrex_vfpu7.inc"
    }

    self->descriptor = &RabbitizerInstrDescriptor_Descriptors[self->uniqueId];
#endif
}

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

void RabbitizerInstructionAllegrex_processUniqueId(RabbitizerInstruction *self) {
    uint32_t opcode = RAB_INSTR_GET_opcode(self);

    self->_mandatorybits = RAB_INSTR_PACK_opcode(self->_mandatorybits, opcode);
    self->instrIdType = RAB_INSTR_ID_TYPE_ALLEGREX_INVALID;

    switch (opcode) {
        default:
            RabbitizerInstructionAllegrex_processUniqueId_Normal(self);
            break;
        case 0x00:
            RabbitizerInstructionAllegrex_processUniqueId_Special(self);
            break;
        case 0x01:
            RabbitizerInstructionAllegrex_processUniqueId_Regimm(self);
            break;

        case 0x1C:
            RabbitizerInstructionAllegrex_processUniqueId_Special2(self);
            break;
        case 0x1F:
            RabbitizerInstructionAllegrex_processUniqueId_Special3(self);
            break;

        case 0x10:
            RabbitizerInstructionAllegrex_processUniqueId_Coprocessor0(self);
            break;
        case 0x11:
            RabbitizerInstructionAllegrex_processUniqueId_Coprocessor1(self);
            break;
        case 0x12:
            RabbitizerInstructionAllegrex_processUniqueId_Coprocessor2(self);
            break;
        case 0x13:
            RabbitizerInstructionAllegrex_processUniqueId_Coprocessor3(self);
            break;

        case 0x18:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu0(self);
            break;
        case 0x19:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu1(self);
            break;
        case 0x1B:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu3(self);
            break;
        case 0x34:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu4(self);
            break;
        case 0x37:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu5(self);
            break;
        case 0x3C:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu6(self);
            break;
        case 0x3F:
            RabbitizerInstructionAllegrex_processUniqueId_Vfpu7(self);
            break;
    }
}
