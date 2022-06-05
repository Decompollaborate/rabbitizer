/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"
#include "instructions/RabbitizerInstrId.h"


static PyModuleDef rabbitizer_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer",
    .m_doc = "",
    .m_size = -1,
};

PyMODINIT_FUNC PyInit_rabbitizer(void) {
    PyObject *m;
    if (PyType_Ready(&rabbitizer_type_Instr) < 0) {
        return NULL;
    }

    m = PyModule_Create(&rabbitizer_module);
    if (m == NULL) {
        return NULL;
    }

    Py_INCREF(&rabbitizer_type_Instr);
    if (PyModule_AddObject(m, "Instr", (PyObject *) &rabbitizer_type_Instr) < 0) {
        Py_DECREF(&rabbitizer_type_Instr);
        Py_DECREF(m);
        return NULL;
    }

    PyObject *submodule = rabbitizer_submodule_instr_id_Init();
    if (submodule == NULL) {
        Py_DECREF(&rabbitizer_type_Instr);
        Py_DECREF(m);
        return NULL;
    }
    if (PyModule_AddObject(m, "instr_id", (PyObject *) submodule) < 0) {
        Py_DECREF(submodule);
        Py_DECREF(&rabbitizer_type_Instr);
        Py_DECREF(m);
        return NULL;
    }

    return m;
}
