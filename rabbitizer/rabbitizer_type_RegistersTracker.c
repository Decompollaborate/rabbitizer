/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"

#include "analysis/RabbitizerRegistersTracker.h"


typedef struct PyRabbitizerRegistersTracker {
    PyObject_HEAD
    RabbitizerRegistersTracker tracker;
} PyRabbitizerRegistersTracker;


static void rabbitizer_type_RegistersTracker_dealloc(PyRabbitizerRegistersTracker *self) {
    RabbitizerRegistersTracker_destroy(&self->tracker);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

static int rabbitizer_type_RegistersTracker_init(PyRabbitizerRegistersTracker *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "other", NULL };
    PyRabbitizerRegistersTracker *pyOther = NULL;
    RabbitizerRegistersTracker *other = NULL;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "|O!", kwlist, &rabbitizer_type_RegistersTracker_TypeObject, &pyOther)) {
        return -1;
    }

    if (pyOther != NULL) {
        other = &pyOther->tracker;
    }

    RabbitizerRegistersTracker_init(&self->tracker, other);

    return 0;
}


static PyObject *rabbitizer_type_RegistersTracker_moveRegisters(PyRabbitizerRegistersTracker *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "instr", NULL };
    PyRabbitizerInstruction *instr;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!", kwlist, &rabbitizer_type_Instruction_TypeObject, &instr)) {
        return NULL;
    }

    if (RabbitizerRegistersTracker_moveRegisters(&self->tracker, &instr->instr)) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

static PyObject *rabbitizer_type_RegistersTracker_overwriteRegisters(PyRabbitizerRegistersTracker *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "instr", "instrOffset", NULL };
    PyRabbitizerInstruction *instr;
    int instrOffset;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!i", kwlist, &rabbitizer_type_Instruction_TypeObject, &instr, &instrOffset)) {
        return NULL;
    }

    RabbitizerRegistersTracker_overwriteRegisters(&self->tracker, &instr->instr, instrOffset);

    Py_RETURN_NONE;
}

static PyObject *rabbitizer_type_RegistersTracker_unsetRegistersAfterFuncCall(PyRabbitizerRegistersTracker *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "instr", "prevInstr", NULL };
    PyRabbitizerInstruction *instr;
    PyRabbitizerInstruction *prevInstr;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!O!", kwlist, &rabbitizer_type_Instruction_TypeObject, &instr, &rabbitizer_type_Instruction_TypeObject, &prevInstr)) {
        return NULL;
    }

    RabbitizerRegistersTracker_unsetRegistersAfterFuncCall(&self->tracker, &instr->instr, &prevInstr->instr);

    Py_RETURN_NONE;
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction)rabbitizer_type_RegistersTracker_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction)rabbitizer_type_RegistersTracker_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }

static PyMethodDef rabbitizer_type_RegistersTracker_methods[] = {
    METHOD_ARGS(moveRegisters, ""),
    METHOD_ARGS(overwriteRegisters, ""),
    METHOD_ARGS(unsetRegistersAfterFuncCall, ""),

    { 0 },
};


PyTypeObject rabbitizer_type_RegistersTracker_TypeObject = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.RegistersTracker",
    .tp_doc = PyDoc_STR("RegistersTracker"),
    .tp_basicsize = sizeof(PyRabbitizerRegistersTracker),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = PyType_GenericNew,
    .tp_init = (initproc) rabbitizer_type_RegistersTracker_init,
    .tp_dealloc = (destructor) rabbitizer_type_RegistersTracker_dealloc,
    // .tp_repr = (reprfunc) rabbitizer_type_RegistersTracker_repr,
    // .tp_str = (reprfunc) rabbitizer_type_RegistersTracker_str,
    .tp_methods = rabbitizer_type_RegistersTracker_methods,
    // .tp_getset = rabbitizer_type_RegistersTracker_getsetters,
};
