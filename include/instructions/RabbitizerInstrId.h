/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRID_H
#define RABBITIZER_INSTRID_H
#pragma once


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    RABBITIZER_INSTR_ID_##prefix##_##name

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    RABBITIZER_INSTR_ID_##prefix##_##name

typedef enum RabbitizerInstrId {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    RABBITIZER_DEF_INSTR_ID(cpu, MAX, ),

    #include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
    RABBITIZER_DEF_INSTR_ID(rsp, MAX, ),

    RABBITIZER_DEF_INSTR_ID(ALL, MAX, ) = RABBITIZER_DEF_INSTR_ID(rsp, MAX, ),
} RabbitizerInstrId;

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


extern const char *RabbitizerInstrId_Names[];


const char *RabbitizerInstrId_getOpcodeName(RabbitizerInstrId uniqueId);

#endif
