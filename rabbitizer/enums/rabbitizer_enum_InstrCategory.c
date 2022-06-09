/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerInstruction.h"


#define RABBITIZER_DEF_INSTR_CATEGORY(name) { "InstrCategory", #name, RABBITIZER_INSTRCAT_##name, false, NULL }

RabbitizerEnumMetadata rabbitizer_enum_InstrCategory_enumvalues[] = {
    #include "instructions/InstrCategory.inc"
    RABBITIZER_DEF_INSTR_CATEGORY(MAX),

    { 0 },
};

#undef RABBITIZER_DEF_INSTR_CATEGORY


static PyModuleDef rabbitizer_enum_InstrCategory_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.InstrCategory",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_enum_InstrCategory_Init(void) {
    PyObject *submodule;

    if (PyType_Ready(&rabbitizer_type_Enum_TypeObject) < 0) {
        return NULL;
    }

    submodule = PyModule_Create(&rabbitizer_enum_InstrCategory_module);
    if (submodule == NULL) {
        return NULL;
    }

    if (rabbitizer_EnumMetadata_Initialize(submodule, rabbitizer_enum_InstrCategory_enumvalues) < 0) {
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
