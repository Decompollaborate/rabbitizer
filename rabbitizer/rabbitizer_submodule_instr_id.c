/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "instructions/RabbitizerInstrId.h"


static PyModuleDef rabbitizer_instr_id_submodule = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.instr_id",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_submodule_instr_id_Init(void) {
    PyObject *submodule;
    submodule = PyModule_Create(&rabbitizer_instr_id_submodule);
    if (submodule == NULL) {
        return NULL;
    }

    if (PyModule_AddIntConstant(submodule, "RABBITIZER_INSTR_CPU_ID_nop", RABBITIZER_INSTR_CPU_ID_nop) < 0) {
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
