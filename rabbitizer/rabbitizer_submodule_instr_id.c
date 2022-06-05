/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/**
 * Wrapper to expose the enums of instructions/RabbitizerInstrId.h
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "instructions/RabbitizerInstrId.h"

static PyObject *instr_id__cpu_get_value(PyObject *self, void *closure) {
    (void)self;
    return Py_BuildValue("i", (RabbitizerInstrId)closure);
}

#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    {#name, (getter) instr_id__cpu_get_value, (setter) NULL, "", (void*)RABBITIZER_INSTR_##prefix##_##name}

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    {#name, (getter) instr_id__cpu_get_value, (setter) NULL, "", (void*)RABBITIZER_INSTR_##prefix##_##name}


static PyGetSetDef instr_id__cpu_getsetters[] = {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    {NULL}  /* Sentinel */
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


PyTypeObject rabbitizer_type_instr_id__cpu = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.instr_id.cpu",
    .tp_doc = PyDoc_STR(""),
    .tp_basicsize = sizeof(PyObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT,
    .tp_new = PyType_GenericNew,
    .tp_getset = instr_id__cpu_getsetters,
};


// TODO: RSP


static PyModuleDef rabbitizer_submodule_instr_id = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.instr_id",
    .m_doc = "",
    .m_size = -1,
};

PyObject *rabbitizer_submodule_instr_id_Init(void) {
    PyObject *submodule;

    submodule = PyModule_Create(&rabbitizer_submodule_instr_id);
    if (submodule == NULL) {
        return NULL;
    }

    if (PyType_Ready(&rabbitizer_type_instr_id__cpu) < 0) {
        return NULL;
    }

    PyObject *cpuIds = PyObject_CallObject((PyObject*)&rabbitizer_type_instr_id__cpu, NULL);

    if (PyModule_AddObject(submodule, "cpu", (PyObject *) cpuIds) < 0) {
        Py_DECREF(cpuIds);
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}
