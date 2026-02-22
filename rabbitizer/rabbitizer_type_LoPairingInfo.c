/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"


static void rabbitizer_type_LoPairingInfo_dealloc(PyRabbitizerLoPairingInfo *self) {

    freefunc tp_free = PyType_GetSlot(Py_TYPE(self), Py_tp_free);
    tp_free((PyObject *) self);
}

static int rabbitizer_type_LoPairingInfo_init(PyRabbitizerLoPairingInfo *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { NULL };

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "", kwlist)) {
        return -1;
    }

    RabbitizerLoPairingInfo_Init(&self->pairingInfo);

    return 0;
}


static PyMemberDef rabbitizer_type_LoPairingInfo_members[] = {
    { "shouldProcess", T_BOOL, offsetof(PyRabbitizerLoPairingInfo, pairingInfo.shouldProcess), 0, "" },
    { "isGpRel", T_BOOL, offsetof(PyRabbitizerLoPairingInfo, pairingInfo.isGpRel), 0, "" },
    { "isGpGot", T_BOOL, offsetof(PyRabbitizerLoPairingInfo, pairingInfo.isGpGot), 0, "" },

    { 0 }
};


#define DEF_MEMBER_GET_INT32(name) \
    static PyObject *rabbitizer_type_LoPairingInfo_member_get_##name(PyRabbitizerLoPairingInfo *self, UNUSED PyObject *closure) { \
        return PyLong_FromLong(self->pairingInfo.name); \
    }

#define DEF_MEMBER_GET_INT64(name) \
    static PyObject *rabbitizer_type_LoPairingInfo_member_get_##name(PyRabbitizerLoPairingInfo *self, UNUSED PyObject *closure) { \
        return PyLong_FromLongLong(self->pairingInfo.name); \
    }

DEF_MEMBER_GET_INT32(instrOffset)
DEF_MEMBER_GET_INT64(value)


#define MEMBER_GET(name, docs)      { #name, (getter) rabbitizer_type_LoPairingInfo_member_get_##name, (setter) NULL,                                            PyDoc_STR(docs), NULL }
#define MEMBER_SET(name, docs)      { #name, (getter) NULL,                                            (setter) rabbitizer_type_LoPairingInfo_member_set_##name, PyDoc_STR(docs), NULL }
#define MEMBER_GET_SET(name, docs)  { #name, (getter) rabbitizer_type_LoPairingInfo_member_get_##name, (setter) rabbitizer_type_LoPairingInfo_member_set_##name, PyDoc_STR(docs), NULL }

static PyGetSetDef rabbitizer_type_LoPairingInfo_getsetters[] = {
    MEMBER_GET(instrOffset, ""),
    MEMBER_GET(value, ""),

    { 0 }
};


DEF_RAB_TYPE(LoPairingInfo)


PyObject *rabbitizer_type_LoPairingInfo_TypeObject = NULL;

static PyType_Slot rabbitizer_type_LoPairingInfo_Slots[] = {
    {Py_tp_doc, PyDoc_STR("LoPairingInfo")},
    {Py_tp_new, PyType_GenericNew},
    {Py_tp_init, rabbitizer_type_LoPairingInfo_init},
    {Py_tp_dealloc, rabbitizer_type_LoPairingInfo_dealloc},
    // {Py_tp_repr, rabbitizer_type_LoPairingInfo_repr},
    // {Py_tp_str, rabbitizer_type_LoPairingInfo_str},
    {Py_tp_members, rabbitizer_type_LoPairingInfo_members},
    // {Py_tp_methods, rabbitizer_type_Instr_methods},
    {Py_tp_getset, rabbitizer_type_LoPairingInfo_getsetters},
    {0, NULL},
};

PyType_Spec rabbitizer_type_LoPairingInfo_Spec = {
    .name = "rabbitizer.LoPairingInfo",
    .basicsize = sizeof(PyRabbitizerLoPairingInfo),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = rabbitizer_type_LoPairingInfo_Slots,
};
