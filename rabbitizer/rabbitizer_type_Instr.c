/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"

#include "instructions/RabbitizerInstr.h"


typedef struct PyRabbitizerInstr {
    PyObject_HEAD
    RabbitizerInstr instr;
} PyRabbitizerInstr;

static void Instr_dealloc(PyRabbitizerInstr *self) {
    RabbitizerInstr_Destroy(&self->instr);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

static int Instr_init(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = {"word", NULL};
    uint32_t word;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "I", kwlist, &word)) {
        return -1;
    }

    RabbitizerInstr_Init(&self->instr, word);
    RabbitizerInstr_ProcessUniqueId(&self->instr);

    return 0;
}

static PyMemberDef Instr_members[] = {
    {"vram", T_UINT, offsetof(PyRabbitizerInstr, instr.vram), 0, "vram description"},
    {NULL}  /* Sentinel */
};

static PyObject *Instr_get_uniqueId(PyRabbitizerInstr *self, void *closure) {
    return Py_BuildValue("i", self->instr.uniqueId);
}

static PyGetSetDef Instr_getsetters[] = {
    {"uniqueId", (getter) Instr_get_uniqueId, (setter) NULL, "", NULL},
    {NULL}  /* Sentinel */
};


static PyObject *Instr_Disassemble(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = {"immOverride", "extraLJust", NULL};
    const char *immOverride = NULL;
    size_t immOverrideLength = 0;
    int extraLJust = 0;
    size_t bufferSize;
    char *buffer;
    PyObject *ret;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "|zi", kwlist, &immOverride, &extraLJust)) {
        return NULL;
    }

    if (immOverride != NULL) {
        immOverrideLength = strlen(immOverride);
    }

    bufferSize = RabbitizerInstr_GetSizeForBuffer(&self->instr, immOverrideLength, extraLJust);

    buffer = malloc(bufferSize+1);
    if (buffer == NULL) {
        return NULL;
    }

    RabbitizerInstr_Disassemble(&self->instr, buffer, immOverride, immOverrideLength, extraLJust);

    ret = PyUnicode_FromString(buffer);
    free(buffer);
    return ret;
}

static PyMethodDef Instr_methods[] = {
    {"disassemble", (PyCFunction) Instr_Disassemble, METH_VARARGS | METH_KEYWORDS, "description"},
    {NULL}  /* Sentinel */
};

PyTypeObject rabbitizer_type_Instr = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.Instr",
    .tp_doc = PyDoc_STR("Instruction"),
    .tp_basicsize = sizeof(PyRabbitizerInstr),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = PyType_GenericNew,
    .tp_init = (initproc) Instr_init,
    .tp_dealloc = (destructor) Instr_dealloc,
    .tp_members = Instr_members,
    .tp_methods = Instr_methods,
    .tp_getset = Instr_getsetters,
};
