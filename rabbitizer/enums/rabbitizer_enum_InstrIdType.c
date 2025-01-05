/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerInstrIdType.h"


#define RAB_DEF_OPCODE_CATEGORY(prefix, name) \
    { "InstrIdType", #prefix "_" #name, RAB_INSTR_ID_TYPE_##prefix##_##name, false, NULL },

RabbitizerEnumMetadata rabbitizer_enum_InstrIdType_enumvalues[] = {
    #include "tables/OpcodeCategory.inc"
    { 0 },
};

#undef RAB_DEF_OPCODE_CATEGORY

static PyMethodDef rabbitizer_enum_InstrIdType_methods[] = {
    { 0 },
};

DEF_ENUM(InstrIdType, "")
