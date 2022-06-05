/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRID_H
#define RABBITIZER_INSTRID_H
#pragma once


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    RABBITIZER_INSTR_##prefix##_##name

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    RABBITIZER_INSTR_##prefix##_##name

typedef enum RabbitizerInstrId {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    RABBITIZER_DEF_INSTR_ID(CPU_ID, MAX),

    #include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
    RABBITIZER_DEF_INSTR_ID(RSP_ID, MAX),

    RABBITIZER_DEF_INSTR_ID(ID, MAX) = RABBITIZER_DEF_INSTR_ID(RSP_ID, MAX),
} RabbitizerInstrId;

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


extern const char *RabbitizerInstrId_Names[];
extern const RabbitizerInstrId RabbitizerInstrId_NotEmitedByCompilers[];


const char *RabbitizerInstrId_getOpcodeName(RabbitizerInstrId uniqueId);

#endif
