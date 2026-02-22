/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"


static void rabbitizer_type_TrackedRegisterState_dealloc(PyRabbitizerTrackedRegisterState *self) {
    RabbitizerTrackedRegisterState_destroy(&self->registerState);

    freefunc tp_free = PyType_GetSlot(Py_TYPE(self), Py_tp_free);
    tp_free((PyObject *) self);
}

static int rabbitizer_type_TrackedRegisterState_init(PyRabbitizerTrackedRegisterState *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "registerNum", NULL };
    int registerNum;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "i", kwlist, &registerNum)) {
        return -1;
    }

    RabbitizerTrackedRegisterState_init(&self->registerState, registerNum);

    return 0;
}


#define DEF_MEMBER_GET_BOOL(name) \
    static PyObject *rabbitizer_type_TrackedRegisterState_member_get_##name(PyRabbitizerTrackedRegisterState *self, UNUSED PyObject *closure) { \
        if (self->registerState.name) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

#define DEF_MEMBER_GET_INT(name) \
    static PyObject *rabbitizer_type_TrackedRegisterState_member_get_##name(PyRabbitizerTrackedRegisterState *self, UNUSED PyObject *closure) { \
        return PyLong_FromLong(self->registerState.name); \
    }

#define DEF_MEMBER_GET_UINT(name) \
    static PyObject *rabbitizer_type_TrackedRegisterState_member_get_##name(PyRabbitizerTrackedRegisterState *self, UNUSED PyObject *closure) { \
        return PyLong_FromUnsignedLong(self->registerState.name); \
    }


DEF_MEMBER_GET_INT(registerNum)

DEF_MEMBER_GET_BOOL(hasLuiValue)
DEF_MEMBER_GET_INT(luiOffset)
DEF_MEMBER_GET_BOOL(luiSetOnBranchLikely)

DEF_MEMBER_GET_BOOL(hasGpGot)
DEF_MEMBER_GET_INT(gpGotOffset)

DEF_MEMBER_GET_BOOL(hasLoValue)
DEF_MEMBER_GET_INT(loOffset)
DEF_MEMBER_GET_BOOL(dereferenced)
DEF_MEMBER_GET_INT(dereferenceOffset)

DEF_MEMBER_GET_BOOL(checkedForBranching)
DEF_MEMBER_GET_INT(lastBranchOffset)

DEF_MEMBER_GET_UINT(value)


#define MEMBER_GET(name, docs, closure)      { #name, (getter) rabbitizer_type_TrackedRegisterState_member_get_##name, (setter) NULL,                                                   PyDoc_STR(docs), closure }
#define MEMBER_SET(name, docs, closure)      { #name, (getter) NULL,                                                   (setter) rabbitizer_type_TrackedRegisterState_member_set_##name, PyDoc_STR(docs), closure }
#define MEMBER_GET_SET(name, docs, closure)  { #name, (getter) rabbitizer_type_TrackedRegisterState_member_get_##name, (setter) rabbitizer_type_TrackedRegisterState_member_set_##name, PyDoc_STR(docs), closure }


static PyGetSetDef rabbitizer_type_TrackedRegisterState_getsetters[] = {
    MEMBER_GET(registerNum, "", NULL),

    MEMBER_GET(hasLuiValue, "", NULL),
    MEMBER_GET(luiOffset, "", NULL),
    MEMBER_GET(luiSetOnBranchLikely, "", NULL),

    MEMBER_GET(hasGpGot, "", NULL),
    MEMBER_GET(gpGotOffset, "", NULL),

    MEMBER_GET(hasLoValue, "", NULL),
    MEMBER_GET(loOffset, "", NULL),
    MEMBER_GET(dereferenced, "", NULL),
    MEMBER_GET(dereferenceOffset, "", NULL),

    MEMBER_GET(checkedForBranching, "", NULL),
    MEMBER_GET(lastBranchOffset, "", NULL),

    MEMBER_GET(value, "", NULL),

    { 0 }
};


DEF_RAB_TYPE(TrackedRegisterState)


PyObject *rabbitizer_type_TrackedRegisterState_TypeObject = NULL;

static PyType_Slot rabbitizer_type_TrackedRegisterState_Slots[] = {
    {Py_tp_doc, PyDoc_STR("TrackedRegisterState")},
    {Py_tp_new, PyType_GenericNew},
    {Py_tp_init, rabbitizer_type_TrackedRegisterState_init},
    {Py_tp_dealloc, rabbitizer_type_TrackedRegisterState_dealloc},
    // {Py_tp_repr, rabbitizer_type_TrackedRegisterState_repr},
    // {Py_tp_str, rabbitizer_type_TrackedRegisterState_str},
    // {Py_tp_methods, rabbitizer_type_TrackedRegisterState_methods},
    {Py_tp_getset, rabbitizer_type_TrackedRegisterState_getsetters},
    {0, NULL},
};

PyType_Spec rabbitizer_type_TrackedRegisterState_Spec = {
    .name = "rabbitizer.TrackedRegisterState",
    .basicsize = sizeof(PyRabbitizerTrackedRegisterState),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = rabbitizer_type_TrackedRegisterState_Slots,
};
