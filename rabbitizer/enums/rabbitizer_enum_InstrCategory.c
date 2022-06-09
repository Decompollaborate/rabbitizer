/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"
#include "common/Utils.h"
#include "instructions/RabbitizerInstruction.h"


typedef struct RabbitizerEnumMetadata {
    const char *enumType;
    const char *name;
    int value;
    bool isInstanced;
    PyObject *instance;
} RabbitizerEnumMetadata;

#define RABBITIZER_DEF_INSTR_CATEGORY(name) { "InstrCategory", #name, RABBITIZER_INSTRCAT_##name, false, NULL }

static RabbitizerEnumMetadata rabbitizer_enum_InstrCategory_enumvalues[] = {
    #include "instructions/InstrCategory.inc"
    RABBITIZER_DEF_INSTR_CATEGORY(MAX),
};

#undef RABBITIZER_DEF_INSTR_CATEGORY


int rabbitizer_enum_InstrCategory_enumvalues_Initialize(PyObject *submodule) {
    for (size_t i = 0; i < ARRAY_COUNT(rabbitizer_enum_InstrCategory_enumvalues); i++) {
        PyObject *args;

        args = Py_BuildValue("ssi", rabbitizer_enum_InstrCategory_enumvalues[i].enumType, rabbitizer_enum_InstrCategory_enumvalues[i].name, rabbitizer_enum_InstrCategory_enumvalues[i].value);
        if (args == NULL) {
            goto error;
        }

        rabbitizer_enum_InstrCategory_enumvalues[i].instance = PyObject_CallObject((PyObject*)&rabbitizer_type_Enum_TypeObject, args);
        Py_DECREF(args);
        if (rabbitizer_enum_InstrCategory_enumvalues[i].instance == NULL) {
            goto error;
        }

        rabbitizer_enum_InstrCategory_enumvalues[i].isInstanced = true;
        if (PyModule_AddObject(submodule, rabbitizer_enum_InstrCategory_enumvalues[i].name, rabbitizer_enum_InstrCategory_enumvalues[i].instance) < 0) {
            goto error;
        }
    }
    return 0;

error:
    for (size_t i = 0; i < ARRAY_COUNT(rabbitizer_enum_InstrCategory_enumvalues); i++) {
        if (rabbitizer_enum_InstrCategory_enumvalues[i].isInstanced) {
            Py_DECREF(rabbitizer_enum_InstrCategory_enumvalues[i].instance);
        }
        rabbitizer_enum_InstrCategory_enumvalues[i].isInstanced = false;
    }

    return -1;
}


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

    if (rabbitizer_enum_InstrCategory_enumvalues_Initialize(submodule) < 0) {
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
