/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"
#include "common/Utils.h"


typedef struct PyRabbitizerEnum {
    PyObject_HEAD
    PyObject *enumType;
    PyObject *name;
    int value;
} PyRabbitizerEnum;


static void rabbitizer_type_Enum_dealloc(PyRabbitizerEnum *self) {
    Py_XDECREF(self->enumType);
    Py_XDECREF(self->name);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

static PyObject *rabbitizer_type_Enum_new(PyTypeObject *type, UNUSED PyObject *args, UNUSED PyObject *kwds) {
    PyRabbitizerEnum *self = (PyRabbitizerEnum *) type->tp_alloc(type, 0);

    if (self == NULL) {
        return NULL;
    }

    self->enumType = PyUnicode_FromString("");
    if (self->enumType == NULL) {
        Py_DECREF(self);
        return NULL;
    }
    self->name = PyUnicode_FromString("");
    if (self->name == NULL) {
        Py_DECREF(self);
        return NULL;
    }
    self->value = 0;

    return (PyObject *) self;
}

static int rabbitizer_type_Enum_init(PyRabbitizerEnum *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "enumType", "name", "value", NULL };
    PyObject *enumType = NULL;
    PyObject *name = NULL;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "UUi", kwlist, &enumType, &name, &self->value)) {
        return -1;
    }

    if (enumType != NULL) {
        PyObject *tmp = NULL;

        tmp = self->enumType;
        Py_INCREF(enumType);
        self->enumType = enumType;
        Py_DECREF(tmp);
    }
    if (name) {
        PyObject *tmp = NULL;

        tmp = self->name;
        Py_INCREF(name);
        self->name = name;
        Py_DECREF(tmp);
    }

    return 0;
}


#define DEF_MEMBER_GET_INT(name) \
    static PyObject *rabbitizer_type_Enum_member_get_##name(PyRabbitizerEnum *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromLong(self->name); \
    }

#define DEF_MEMBER_GET_OBJREF(name) \
    static PyObject *rabbitizer_type_Enum_member_get_##name(PyRabbitizerEnum *self, PyObject *Py_UNUSED(ignored)) { \
        Py_INCREF(self->name); \
        return self->name; \
    }

DEF_MEMBER_GET_OBJREF(name)
DEF_MEMBER_GET_INT(value)


#define MEMBER_GET(name, docs, closure)      { #name, (getter) rabbitizer_type_Enum_member_get_##name, (setter) NULL,                                   PyDoc_STR(docs), closure }
#define MEMBER_SET(name, docs, closure)      { #name, (getter) NULL,                                   (setter) rabbitizer_type_Enum_member_set_##name, PyDoc_STR(docs), closure }
#define MEMBER_GET_SET(name, docs, closure)  { #name, (getter) rabbitizer_type_Enum_member_get_##name, (setter) rabbitizer_type_Enum_member_set_##name, PyDoc_STR(docs), closure }

static PyGetSetDef rabbitizer_type_Enum_getsetters[] = {
    MEMBER_GET(name, "", NULL),
    MEMBER_GET(value, "", NULL),

    { 0 },
};



PyObject *rabbitizer_type_Enum_richcompare(PyRabbitizerEnum *self, PyObject *other, int op) {
    int isInstance = PyObject_IsInstance(other, (PyObject*)&rabbitizer_type_Enum_TypeObject);
    int enumTypeCmp;

    if (isInstance < 0) {
        return NULL;
    }

    if (isInstance == 0) {
        Py_RETURN_FALSE;
    }

    enumTypeCmp = PyUnicode_Compare(self->enumType, ((PyRabbitizerEnum*)other)->enumType);
    if (enumTypeCmp < 0) {
        if (PyErr_Occurred() != NULL) {
            return NULL;
        }
        Py_RETURN_FALSE;
    }
    if (enumTypeCmp != 0) {
        Py_RETURN_FALSE;
    }

    Py_RETURN_RICHCOMPARE(self->value, ((PyRabbitizerEnum*)other)->value, op);
}


static PyObject *rabbitizer_type_Enum_repr(PyRabbitizerEnum *self) {
    return PyUnicode_FromFormat("<%U: %U (%i)>", self->enumType, self->name, self->value);
}

static PyObject *rabbitizer_type_Enum_str(PyRabbitizerEnum *self) {
    return rabbitizer_type_Enum_repr(self);
}

// TODO: implement hash and int

PyTypeObject rabbitizer_type_Enum_TypeObject = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.Enum",
    .tp_doc = PyDoc_STR("Enum"),
    .tp_basicsize = sizeof(PyRabbitizerEnum),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = rabbitizer_type_Enum_new,
    .tp_init = (initproc) rabbitizer_type_Enum_init,
    .tp_dealloc = (destructor) rabbitizer_type_Enum_dealloc,
    .tp_richcompare = (richcmpfunc) rabbitizer_type_Enum_richcompare,
    .tp_repr = (reprfunc) rabbitizer_type_Enum_repr,
    .tp_str = (reprfunc) rabbitizer_type_Enum_str,
    //.tp_members = rabbitizer_type_Enum_members,
    //.tp_methods = rabbitizer_type_Enum_methods,
    .tp_getset = rabbitizer_type_Enum_getsetters,
};