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
    {"inHandwrittenFunction", T_BOOL, offsetof(PyRabbitizerInstr, instr.inHandwrittenFunction), 0, "vram description"},
    {NULL}  /* Sentinel */
};


#define DEF_MEMBER_GET_UINT(name) \
    static PyObject *Instr_member_get_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromUnsignedLong((&self->instr)->name); \
    }

DEF_MEMBER_GET_UINT(opcode)
DEF_MEMBER_GET_UINT(rs)
DEF_MEMBER_GET_UINT(rt)
DEF_MEMBER_GET_UINT(rd)
DEF_MEMBER_GET_UINT(sa)
DEF_MEMBER_GET_UINT(function)
DEF_MEMBER_GET_UINT(uniqueId)


static PyObject *Instr_instr(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
    return PyLong_FromUnsignedLong(RabbitizerInstr_GetRaw(&self->instr)); \
}
static PyObject *Instr_immediate(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
    return PyLong_FromUnsignedLong(RabbitizerInstr_GetImmediate(&self->instr)); \
}


static PyGetSetDef Instr_getsetters[] = {
    { "opcode", (getter) Instr_member_get_opcode, (setter) NULL, "", NULL },
    { "rs", (getter) Instr_member_get_rs, (setter) NULL, "", NULL },
    { "rt", (getter) Instr_member_get_rt, (setter) NULL, "", NULL },
    { "rd", (getter) Instr_member_get_rd, (setter) NULL, "", NULL },
    { "sa", (getter) Instr_member_get_sa, (setter) NULL, "", NULL },
    { "function", (getter) Instr_member_get_function, (setter) NULL, "", NULL },
    { "uniqueId", (getter) Instr_member_get_uniqueId, (setter) NULL, "", NULL },
    { "instr", (getter) Instr_instr, (setter) NULL, "", NULL }, // todo: deprecate
    { "immediate", (getter) Instr_immediate, (setter) NULL, "", NULL }, // todo: deprecate
    {NULL}  /* Sentinel */
};


#define DEF_METHOD_GET_UINT(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromUnsignedLong(RabbitizerInstr_##name(&self->instr)); \
    }

#define DEF_METHOD_GET_INT(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromLong(RabbitizerInstr_##name(&self->instr)); \
    }

DEF_METHOD_GET_UINT(GetRaw)
DEF_METHOD_GET_UINT(GetImmediate)
DEF_METHOD_GET_UINT(GetInstrIndexAsVram)
DEF_METHOD_GET_INT(GetBranchOffset)


#define DEF_METHOD_BOOL(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstr_##name(&self->instr)) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

DEF_METHOD_BOOL(IsImplemented)
DEF_METHOD_BOOL(IsLikelyHandwritten)
DEF_METHOD_BOOL(IsNop)
DEF_METHOD_BOOL(IsUnconditionalBranch)
DEF_METHOD_BOOL(IsJrRa)
DEF_METHOD_BOOL(IsJrNotRa)


#define DEF_DESCRIPTOR_METHOD_BOOL(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstrDescriptor_##name(self->instr.descriptor)) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

DEF_DESCRIPTOR_METHOD_BOOL(IsJType)
DEF_DESCRIPTOR_METHOD_BOOL(IsIType)
DEF_DESCRIPTOR_METHOD_BOOL(IsRType)
DEF_DESCRIPTOR_METHOD_BOOL(IsBranch)
DEF_DESCRIPTOR_METHOD_BOOL(IsBranchLikely)
DEF_DESCRIPTOR_METHOD_BOOL(IsJump)
DEF_DESCRIPTOR_METHOD_BOOL(IsTrap)
DEF_DESCRIPTOR_METHOD_BOOL(IsFloat)
DEF_DESCRIPTOR_METHOD_BOOL(IsDouble)
DEF_DESCRIPTOR_METHOD_BOOL(IsUnsigned)
DEF_DESCRIPTOR_METHOD_BOOL(ModifiesRt)
DEF_DESCRIPTOR_METHOD_BOOL(ModifiesRd)


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

static PyObject *Instr_MapInstrToType(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) {
    Py_RETURN_NONE;
}

static PyMethodDef Instr_methods[] = {
    {"getRaw", (PyCFunction)Instr_GetRaw, METH_NOARGS, ""},
    {"getImmediate", (PyCFunction)Instr_GetImmediate, METH_NOARGS, ""},
    {"getInstrIndexAsVram", (PyCFunction)Instr_GetInstrIndexAsVram, METH_NOARGS, ""},
    {"getBranchOffset", (PyCFunction)Instr_GetBranchOffset, METH_NOARGS, ""},

    {"isImplemented", (PyCFunction)Instr_IsImplemented, METH_NOARGS, ""},
    {"isLikelyHandwritten", (PyCFunction)Instr_IsLikelyHandwritten, METH_NOARGS, ""},
    {"isNop", (PyCFunction)Instr_IsNop, METH_NOARGS, ""},
    {"isUnconditionalBranch", (PyCFunction)Instr_IsUnconditionalBranch, METH_NOARGS, ""},
    {"isJrRa", (PyCFunction)Instr_IsJrRa, METH_NOARGS, ""},
    {"isJrNotRa", (PyCFunction)Instr_IsJrNotRa, METH_NOARGS, ""},

    {"isJType", (PyCFunction)Instr_IsJType, METH_NOARGS, ""},
    {"isIType", (PyCFunction)Instr_IsIType, METH_NOARGS, ""},
    {"isRType", (PyCFunction)Instr_IsRType, METH_NOARGS, ""},
    {"isBranch", (PyCFunction)Instr_IsBranch, METH_NOARGS, ""},
    {"isBranchLikely", (PyCFunction)Instr_IsBranchLikely, METH_NOARGS, ""},
    {"isJump", (PyCFunction)Instr_IsJump, METH_NOARGS, ""},
    {"isTrap", (PyCFunction)Instr_IsTrap, METH_NOARGS, ""},
    {"isFloat", (PyCFunction)Instr_IsFloat, METH_NOARGS, ""},
    {"isFloatInstruction", (PyCFunction)Instr_IsFloat, METH_NOARGS, ""}, // TODO: deprecate
    {"isDouble", (PyCFunction)Instr_IsDouble, METH_NOARGS, ""},
    {"isDoubleInstruction", (PyCFunction)Instr_IsDouble, METH_NOARGS, ""}, // TODO: deprecate
    {"isUnsigned", (PyCFunction)Instr_IsUnsigned, METH_NOARGS, ""},
    {"modifiesRt", (PyCFunction)Instr_ModifiesRt, METH_NOARGS, ""},
    {"modifiesRd", (PyCFunction)Instr_ModifiesRd, METH_NOARGS, ""},

    {"disassemble", (PyCFunction) Instr_Disassemble, METH_VARARGS | METH_KEYWORDS, "description"},
    {"mapInstrToType", (PyCFunction) Instr_MapInstrToType, METH_VARARGS | METH_KEYWORDS, "description"},
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
