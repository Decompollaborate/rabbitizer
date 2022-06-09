/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerInstrId.h"


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...)                   { "InstrId", #prefix "_" #name, RABBITIZER_INSTR_ID_##prefix##_##name, false, NULL }
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...)  { "InstrId", #prefix "_" #name, RABBITIZER_INSTR_ID_##prefix##_##name, false, NULL }

RabbitizerEnumMetadata rabbitizer_enum_InstrId_enumvalues[] = {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    RABBITIZER_DEF_INSTR_ID(cpu, MAX, ),

    #include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
    RABBITIZER_DEF_INSTR_ID(rsp, MAX, ),

    RABBITIZER_DEF_INSTR_ID(ALL, MAX, ),
    { 0 },
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


static PyModuleDef rabbitizer_enum_InstrId_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.InstrId",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_enum_InstrId_Init(void) {
    PyObject *submodule;

    if (PyType_Ready(&rabbitizer_type_Enum_TypeObject) < 0) {
        return NULL;
    }

    submodule = PyModule_Create(&rabbitizer_enum_InstrId_module);
    if (submodule == NULL) {
        return NULL;
    }

    if (rabbitizer_EnumMetadata_Initialize(submodule, rabbitizer_enum_InstrId_enumvalues) < 0) {
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
