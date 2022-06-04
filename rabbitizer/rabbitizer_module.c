/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"


static PyModuleDef rabbitizer_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer",
    .m_doc = "",
    .m_size = -1,
};

PyMODINIT_FUNC
PyInit_rabbitizer(void)
{
    PyObject *m;
    if (PyType_Ready(&rabbitizer_Instr_Type) < 0)
        return NULL;

    m = PyModule_Create(&rabbitizer_module);
    if (m == NULL)
        return NULL;

    Py_INCREF(&rabbitizer_Instr_Type);
    if (PyModule_AddObject(m, "Instr", (PyObject *) &rabbitizer_Instr_Type) < 0) {
        Py_DECREF(&rabbitizer_Instr_Type);
        Py_DECREF(m);
        return NULL;
    }

    return m;
}
