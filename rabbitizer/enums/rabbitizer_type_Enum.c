/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"
#include "enums_utils.h"
#include "common/Utils.h"


static void rabbitizer_type_Enum_dealloc(PyRabbitizerEnum *self) {
    Py_XDECREF(self->enumType);
    Py_XDECREF(self->name);

    freefunc tp_free = PyType_GetSlot(Py_TYPE(self), Py_tp_free);
    tp_free((PyObject *) self);
}

static PyObject *rabbitizer_type_Enum_new(PyTypeObject *type, UNUSED PyObject *args, UNUSED PyObject *kwds) {
    allocfunc tp_alloc = PyType_GetSlot(type, Py_tp_alloc);
    if (tp_alloc == NULL) {
        return NULL;
    }
    PyRabbitizerEnum *self = (void *) tp_alloc(type, 0);

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


Py_hash_t rabbitizer_type_Enum_hash(PyRabbitizerEnum *self) {
    PyObject *tuple = PyTuple_Pack(2, self->enumType, PyLong_FromLong(self->value));
    Py_hash_t hash;

    if (tuple == NULL) {
        return -1;
    }

    hash = PyObject_Hash(tuple);

    if (hash == -1) {
        return -1;
    }

    return hash;
}

// Checks for the 6 basic comparisons (==, !=, <, <=, >, >=)
PyObject *rabbitizer_type_Enum_richcompare(PyRabbitizerEnum *self, PyObject *other, int op) {
    int isInstance = PyObject_IsInstance(other, rabbitizer_type_Enum_TypeObject);
    int enumTypeCmp;
    int otherValue;

    if (isInstance < 0) {
        // An error happened
        // PyObject_IsInstance already sets an exception, so nothing else to do here
        return NULL;
    }

    if (isInstance == 0) {
        // `other` isn't an instance of the Enum type
        Py_RETURN_FALSE;
    }

    // Check if both enums have the same `enumType`
    enumTypeCmp = PyUnicode_Compare(self->enumType, ((PyRabbitizerEnum*)other)->enumType);
    if (enumTypeCmp < 0) {
        // Negative could mean it isn't equal or an error occurred, so we need to check for errors
        if (PyErr_Occurred() != NULL) {
            return NULL;
        }
        Py_RETURN_FALSE;
    }
    if (enumTypeCmp != 0) {
        Py_RETURN_FALSE;
    }

    otherValue = ((PyRabbitizerEnum*)other)->value;

    // Fails to build on ci builds for some reason
    // Py_RETURN_RICHCOMPARE(self->value, ((PyRabbitizerEnum*)other)->value, op);

    switch (op) {
        case Py_EQ: if ((self->value) == (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        case Py_NE: if ((self->value) != (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        case Py_LT: if ((self->value) < (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        case Py_GT: if ((self->value) > (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        case Py_LE: if ((self->value) <= (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        case Py_GE: if ((self->value) >= (otherValue)) Py_RETURN_TRUE; Py_RETURN_FALSE;
        default:
            //Py_UNREACHABLE();
            UNREACHABLE;
    }
}


// To allow piclking the object
static PyObject *rabbitizer_type_Enum___reduce__(PyRabbitizerEnum *self, UNUSED PyObject *closure) {
    PyObject *args;
    PyObject *enumType;
    PyObject *name;
    PyObject *value;

    enumType = self->enumType;
    Py_INCREF(enumType);
    name = self->name;
    Py_INCREF(name);
    value = PyLong_FromLong(self->value);


    args = PyTuple_Pack(3, enumType, name, value);

    return PyTuple_Pack(2, rabbitizer_type_Enum_TypeObject, args);
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction) (void *) rabbitizer_type_Enum_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction) (void *) rabbitizer_type_Enum_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }


static PyMethodDef rabbitizer_type_Enum_methods[] = {
    METHOD_ARGS(__reduce__, ""),

    { 0 },
};

static PyObject *rabbitizer_type_Enum_repr(PyRabbitizerEnum *self) {
    return PyUnicode_FromFormat("<%U: %U (%i)>", self->enumType, self->name, self->value);
}

static PyObject *rabbitizer_type_Enum_str(PyRabbitizerEnum *self) {
    return rabbitizer_type_Enum_repr(self);
}

// TODO: implement hash and int

PyObject *rabbitizer_type_Enum_TypeObject = NULL;

static PyType_Slot rabbitizer_type_Enum_Slots[] = {
    {Py_tp_doc, PyDoc_STR("Enum")},
    {Py_tp_new, rabbitizer_type_Enum_new},
    {Py_tp_init, rabbitizer_type_Enum_init},
    {Py_tp_dealloc, rabbitizer_type_Enum_dealloc},
    {Py_tp_hash, rabbitizer_type_Enum_hash},
    {Py_tp_richcompare, rabbitizer_type_Enum_richcompare},
    {Py_tp_repr, rabbitizer_type_Enum_repr},
    {Py_tp_str, rabbitizer_type_Enum_str},
    // {Py_tp_members, rabbitizer_type_Enum_members},
    {Py_tp_methods, rabbitizer_type_Enum_methods},
    {Py_tp_getset, rabbitizer_type_Enum_getsetters},
    {0, NULL}
};

PyType_Spec rabbitizer_type_Enum_Spec = {
    .name = "rabbitizer.Enum",
    .basicsize = sizeof(PyRabbitizerEnum),
    .itemsize = 0,
    .flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .slots = rabbitizer_type_Enum_Slots,
};
