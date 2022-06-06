/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/**
 * Wrapper to expose the enums of instructions/RabbitizerInstrId.h
 */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "instructions/RabbitizerInstrId.h"

static PyObject *rabbitizer_global_instr_id_get_closure_id(PyObject *self, RabbitizerInstrId closure) {
    (void)self;
    return PyLong_FromLong(closure);
}

#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    { #prefix "_" #name, (getter) (void*) rabbitizer_global_instr_id_get_closure_id, (setter) NULL, "", (void*)RABBITIZER_INSTR_ID_##prefix##_##name }

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    { #prefix "_" #name, (getter) (void*) rabbitizer_global_instr_id_get_closure_id, (setter) NULL, "", (void*)RABBITIZER_INSTR_ID_##prefix##_##name }


static PyGetSetDef rabbitizer_global_instr_id_GetSets[] = {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    { NULL }
};

PyTypeObject rabbitizer_global_instr_id_TypeObject = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.instr_id",
    .tp_doc = PyDoc_STR(""),
    .tp_basicsize = sizeof(PyObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT,
    .tp_new = PyType_GenericNew,
    .tp_getset = rabbitizer_global_instr_id_GetSets,
};
