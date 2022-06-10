/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "common/RabbitizerConfig.h"
#include "common/Utils.h"


#define RABBITIZER_DEF_ABI(name) { "Abi", #name, RABBITIZER_ABI_##name, false, NULL }

RabbitizerEnumMetadata rabbitizer_enum_Abi_enumvalues[] = {
    #include "common/Abi.inc"
    RABBITIZER_DEF_ABI(MAX),

    { 0 },
};

#undef RABBITIZER_DEF_ABI


static PyObject *rabbitizer_enum_Abi_fromStr(UNUSED PyObject *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "name", NULL };
    const char *name = NULL;
    RabbitizerAbi abi;
    PyObject *ret;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "s", kwlist, &name)) {
        return NULL;
    }

    abi = RabbitizerAbi_fromStr(name);

    ret = rabbitizer_enum_Abi_enumvalues[abi].instance;
    Py_INCREF(ret);
    return ret;
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction) rabbitizer_enum_Abi_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction) rabbitizer_enum_Abi_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }

static PyMethodDef rabbitizer_enum_Abi_methods[] = {
    METHOD_ARGS(fromStr, ""),

    { 0 },
};


static PyModuleDef rabbitizer_enum_Abi_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.Abi",
    .m_doc = "",
    .m_size = -1,
    .m_methods = rabbitizer_enum_Abi_methods,
};

PyObject *rabbitizer_enum_Abi_Init(void) {
    PyObject *submodule;

    if (PyType_Ready(&rabbitizer_type_Enum_TypeObject) < 0) {
        return NULL;
    }

    submodule = PyModule_Create(&rabbitizer_enum_Abi_module);
    if (submodule == NULL) {
        return NULL;
    }

    if (rabbitizer_EnumMetadata_Initialize(submodule, rabbitizer_enum_Abi_enumvalues) < 0) {
        Py_DECREF(submodule);
        return NULL;
    }

    return submodule;
}

// Return true if o is of this enum type
int rabbitizer_enum_Abi_Check(PyObject *o) {
    int isInstance = PyObject_IsInstance(o, (PyObject*)&rabbitizer_type_Enum_TypeObject);
    int enumTypeCmp;

    if (isInstance < 0) {
        // An error happened
        // PyObject_IsInstance already sets an exception, so nothing else to do here
        return -1;
    }

    if (isInstance == 0) {
        // `other` isn't an instance of the Enum type
        return 0;
    }

    // Check if both enums have the same `enumType`
    enumTypeCmp = PyUnicode_CompareWithASCIIString(((PyRabbitizerEnum*)o)->enumType, "Abi");
    return enumTypeCmp == 0;
}
