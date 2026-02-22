/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"


static void rabbitizer_type_JrRegData_dealloc(PyRabbitizerJrRegData *self) {

    freefunc tp_free = PyType_GetSlot(Py_TYPE(self), Py_tp_free);
    tp_free((PyObject *) self);
}

static int rabbitizer_type_JrRegData_init(PyRabbitizerJrRegData *self, UNUSED PyObject *args, UNUSED PyObject *kwds) {
    RabbitizerJrRegData_init(&self->jrRegData);

    return 0;
}


static PyObject *rabbitizer_type_JrRegData_hasInfo(PyRabbitizerJrRegData *self, UNUSED PyObject *closure) {
    if (self->jrRegData.hasInfo) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

static PyObject *rabbitizer_type_JrRegData_offset(PyRabbitizerJrRegData *self, UNUSED PyObject *closure) {
    return PyLong_FromLong(self->jrRegData.offset);
}

static PyObject *rabbitizer_type_JrRegData_address(PyRabbitizerJrRegData *self, UNUSED PyObject *closure) {
    return PyLong_FromUnsignedLong(self->jrRegData.address);
}

static PyObject *rabbitizer_type_JrRegData_checkedForBranching(PyRabbitizerJrRegData *self, UNUSED PyObject *closure) {
    if (self->jrRegData.checkedForBranching) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

static PyObject *rabbitizer_type_JrRegData_lastBranchOffset(PyRabbitizerJrRegData *self, UNUSED PyObject *closure) {
    return PyLong_FromLong(self->jrRegData.lastBranchOffset);
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction) (void *) rabbitizer_type_JrRegData_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction) (void *) rabbitizer_type_JrRegData_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }

static PyMethodDef rabbitizer_type_JrRegData_methods[] = {
    METHOD_NO_ARGS(hasInfo, ""),
    METHOD_NO_ARGS(offset, ""),
    METHOD_NO_ARGS(address, ""),
    METHOD_NO_ARGS(checkedForBranching, ""),
    METHOD_NO_ARGS(lastBranchOffset, ""),

    { 0 },
};



DEF_RAB_TYPE(JrRegData)


PyObject *rabbitizer_type_JrRegData_TypeObject = NULL;

static PyType_Slot rabbitizer_type_JrRegData_Slots[] = {
    {Py_tp_doc, PyDoc_STR("JrRegData")},
    {Py_tp_new, PyType_GenericNew},
    {Py_tp_init, rabbitizer_type_JrRegData_init},
    {Py_tp_dealloc, rabbitizer_type_JrRegData_dealloc},
    // {Py_tp_repr, rabbitizer_type_JrRegData_repr},
    // {Py_tp_as_sequence, &rabbitizer_type_JrRegData_classSeqMethods},
    // {Py_tp_str, rabbitizer_type_JrRegData_str},
    {Py_tp_methods, rabbitizer_type_JrRegData_methods},
    // {Py_tp_getset, rabbitizer_type_JrRegData_getsetters},
    {0, NULL},
};

PyType_Spec rabbitizer_type_JrRegData_Spec = {
    .name = "rabbitizer.JrRegData",
    .basicsize = sizeof(PyRabbitizerJrRegData),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = rabbitizer_type_JrRegData_Slots,
};
