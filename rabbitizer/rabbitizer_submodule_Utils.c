/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"

#include "common/Utils.h"


static PyObject *rabbitizer_submodule_Utils_from2Complement(UNUSED PyObject *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "number", "bits", NULL };
    uint32_t number = 0;
    int bits = 0;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "Ii", kwlist, &number, &bits)) {
        return NULL;
    }

    return PyLong_FromLong(RabbitizerUtils_From2Complement(number, bits));
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction) rabbitizer_submodule_Utils_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction) rabbitizer_submodule_Utils_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }

static PyMethodDef rabbitizer_submodule_Utils_methods[] = {
    METHOD_ARGS(from2Complement, ""),

    { 0 },
};


static PyModuleDef rabbitizer_submodule_Utils_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer.Utils",
    .m_doc = "",
    .m_size = -1,
    .m_methods = rabbitizer_submodule_Utils_methods,
};

PyObject *rabbitizer_submodule_Utils_Init(void) {
    PyObject *submodule;

    submodule = PyModule_Create(&rabbitizer_submodule_Utils_module);
    if (submodule == NULL) {
        return NULL;
    }

    return submodule;
}
