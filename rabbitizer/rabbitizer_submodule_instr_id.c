/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/**
 * Wrapper to expose the enums of instructions/RabbitizerInstrId.h
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "instructions/RabbitizerInstrId.h"


static PyModuleDef rabbitizer_submodule_instr_id__cpu = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.instr_id.cpu",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_submodule_instr_id__cpu_Init(void) {
    PyObject *submodule;
    RabbitizerInstrId instrId;

    submodule = PyModule_Create(&rabbitizer_submodule_instr_id__cpu);
    if (submodule == NULL) {
        return NULL;
    }

    for (instrId = RABBITIZER_INSTR_CPU_ID_INVALID; instrId < RABBITIZER_INSTR_CPU_ID_MAX; instrId++) {
        if (PyModule_AddIntConstant(submodule, RabbitizerInstrId_Names[instrId], instrId) < 0) {
            Py_DECREF(submodule);
            return NULL;
        }
    }

    return submodule;
}

// TODO: RSP


static PyModuleDef rabbitizer_submodule_instr_id = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.instr_id",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_submodule_instr_id_Init(void) {
    PyObject *submodule;
    PyObject *cpuSubmodule;

    submodule = PyModule_Create(&rabbitizer_submodule_instr_id);
    if (submodule == NULL) {
        return NULL;
    }

    cpuSubmodule = rabbitizer_submodule_instr_id__cpu_Init();
    if (cpuSubmodule == NULL) {
        Py_DECREF(submodule);
        return NULL;
    }

    if (PyModule_AddObject(submodule, "cpu", (PyObject *) cpuSubmodule) < 0) {
        Py_DECREF(cpuSubmodule);
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
