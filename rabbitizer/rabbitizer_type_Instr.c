/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"

#include "instructions/RabbitizerInstr.h"


typedef struct PyRabbitizerInstr {
    PyObject_HEAD
    RabbitizerInstr instr;
} PyRabbitizerInstr;

extern PyTypeObject rabbitizer_type_Instr_TypeObject;


static void Instr_dealloc(PyRabbitizerInstr *self) {
    RabbitizerInstr_destroy(&self->instr);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

static int Instr_init(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = {"word", NULL};
    uint32_t word;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "I", kwlist, &word)) {
        return -1;
    }

    RabbitizerInstr_init(&self->instr, word);
    RabbitizerInstr_processUniqueId(&self->instr);

    return 0;
}

static PyMemberDef Instr_members[] = {
    {"vram", T_UINT, offsetof(PyRabbitizerInstr, instr.vram), 0, "vram description"},
    {"inHandwrittenFunction", T_BOOL, offsetof(PyRabbitizerInstr, instr.inHandwrittenFunction), 0, "inHandwrittenFunction description"},
    {NULL}  /* Sentinel */
};


#define DEF_MEMBER_GET_UINT(name) \
    static PyObject *Instr_member_get_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromUnsignedLong(self->instr.name); \
    }

DEF_MEMBER_GET_UINT(opcode)
DEF_MEMBER_GET_UINT(rs)
DEF_MEMBER_GET_UINT(rt)
DEF_MEMBER_GET_UINT(rd)
DEF_MEMBER_GET_UINT(sa)
DEF_MEMBER_GET_UINT(function)
DEF_MEMBER_GET_UINT(uniqueId)


static PyObject *Instr_instr(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
    return PyLong_FromUnsignedLong(RabbitizerInstr_getRaw(&self->instr)); \
}
static PyObject *Instr_immediate(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
    return PyLong_FromUnsignedLong(RabbitizerInstr_getImmediate(&self->instr)); \
}

#define MEMBER_GET(name, docs, closure)  { #name, (getter) Instr_member_get_##name, (setter) NULL, PyDoc_STR(docs), closure }
#define MEMBER_SET(name, docs, closure)  { #name, (getter) NULL, (setter) Instr_member_set_##name, PyDoc_STR(docs), closure }
#define MEMBER_GET_SET(name, docs, closure)  { #name, (getter) Instr_member_get_##name, (setter) Instr_member_set_##name, PyDoc_STR(docs), closure }


static PyGetSetDef Instr_getsetters[] = {
    MEMBER_GET(opcode, "", NULL),
    MEMBER_GET(rs, "", NULL),
    MEMBER_GET(rt, "", NULL),
    MEMBER_GET(rd, "", NULL),
    MEMBER_GET(sa, "", NULL),
    MEMBER_GET(function, "", NULL),
    MEMBER_GET(uniqueId, "", NULL),
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

DEF_METHOD_GET_UINT(getRaw)
DEF_METHOD_GET_UINT(getImmediate)
DEF_METHOD_GET_UINT(getInstrIndexAsVram)
DEF_METHOD_GET_INT(getBranchOffset)


static PyObject *Instr_blankOut(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) {
    RabbitizerInstr_blankOut(&self->instr);
    Py_RETURN_NONE;
}


#define DEF_METHOD_BOOL(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstr_##name(&self->instr)) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

DEF_METHOD_BOOL(isImplemented)
DEF_METHOD_BOOL(isLikelyHandwritten)
DEF_METHOD_BOOL(isNop)
DEF_METHOD_BOOL(isUnconditionalBranch)
DEF_METHOD_BOOL(isJrRa)
DEF_METHOD_BOOL(isJrNotRa)

static PyObject *Instr_mapInstrToType(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) {
    const char *type = RabbitizerInstr_mapInstrToType(&self->instr);

    if (type != NULL) {
        return PyUnicode_FromString(type);
    }
    Py_RETURN_NONE;
}

static PyObject *Instr_sameOpcode(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = {"other", NULL};
    PyRabbitizerInstr *other;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!", kwlist, &rabbitizer_type_Instr_TypeObject, &other)) {
        return NULL;
    }

    if (RabbitizerInstr_sameOpcode(&self->instr, &other->instr)) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

static PyObject *Instr_sameOpcodeButDifferentArguments(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = {"other", NULL};
    PyRabbitizerInstr *other;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!", kwlist, &rabbitizer_type_Instr_TypeObject, &other)) {
        return NULL;
    }

    if (RabbitizerInstr_sameOpcodeButDifferentArguments(&self->instr, &other->instr)) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

#define DEF_DESCRIPTOR_METHOD_BOOL(name) \
    static PyObject *Instr_##name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstrDescriptor_##name(self->instr.descriptor)) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

DEF_DESCRIPTOR_METHOD_BOOL(isJType)
DEF_DESCRIPTOR_METHOD_BOOL(isIType)
DEF_DESCRIPTOR_METHOD_BOOL(isRType)
DEF_DESCRIPTOR_METHOD_BOOL(isBranch)
DEF_DESCRIPTOR_METHOD_BOOL(isBranchLikely)
DEF_DESCRIPTOR_METHOD_BOOL(isJump)
DEF_DESCRIPTOR_METHOD_BOOL(isTrap)
DEF_DESCRIPTOR_METHOD_BOOL(isFloat)
DEF_DESCRIPTOR_METHOD_BOOL(isDouble)
DEF_DESCRIPTOR_METHOD_BOOL(isUnsigned)
DEF_DESCRIPTOR_METHOD_BOOL(modifiesRt)
DEF_DESCRIPTOR_METHOD_BOOL(modifiesRd)


static PyObject *Instr_disassemble(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds) {
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

    bufferSize = RabbitizerInstr_getSizeForBuffer(&self->instr, immOverrideLength, extraLJust);

    buffer = malloc(bufferSize+1);
    if (buffer == NULL) {
        // TODO: signal an exception?
        return NULL;
    }

    RabbitizerInstr_disassemble(&self->instr, buffer, immOverride, immOverrideLength, extraLJust);

    ret = PyUnicode_FromString(buffer);
    free(buffer);
    return ret;
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction)Instr_##name, METH_NOARGS, PyDoc_STR(docs) }


static PyMethodDef Instr_methods[] = {
    METHOD_NO_ARGS(getRaw, ""),
    METHOD_NO_ARGS(getImmediate, ""),
    METHOD_NO_ARGS(getInstrIndexAsVram, ""),
    METHOD_NO_ARGS(getBranchOffset, ""),

    METHOD_NO_ARGS(blankOut, ""),

    METHOD_NO_ARGS(isImplemented, ""),
    METHOD_NO_ARGS(isLikelyHandwritten, ""),
    METHOD_NO_ARGS(isNop, ""),
    METHOD_NO_ARGS(isUnconditionalBranch, ""),
    METHOD_NO_ARGS(isJrRa, ""),
    METHOD_NO_ARGS(isJrNotRa, ""),
    METHOD_NO_ARGS(mapInstrToType, ""),

    {"sameOpcode", (PyCFunction) Instr_sameOpcode, METH_VARARGS | METH_KEYWORDS, "description"},
    {"sameOpcodeButDifferentArguments", (PyCFunction)Instr_sameOpcodeButDifferentArguments, METH_VARARGS | METH_KEYWORDS, "description"},

    METHOD_NO_ARGS(isJType, ""),
    METHOD_NO_ARGS(isIType, ""),
    METHOD_NO_ARGS(isRType, ""),
    METHOD_NO_ARGS(isBranch, ""),
    METHOD_NO_ARGS(isBranchLikely, ""),
    METHOD_NO_ARGS(isJump, ""),
    METHOD_NO_ARGS(isTrap, ""),
    METHOD_NO_ARGS(isFloat, ""),
    {"isFloatInstruction", (PyCFunction)Instr_isFloat, METH_NOARGS, ""}, // TODO: deprecate
    METHOD_NO_ARGS(isDouble, ""),
    {"isDoubleInstruction", (PyCFunction)Instr_isDouble, METH_NOARGS, ""}, // TODO: deprecate
    METHOD_NO_ARGS(isUnsigned, ""),
    METHOD_NO_ARGS(modifiesRt, ""),
    METHOD_NO_ARGS(modifiesRd, ""),

    {"disassemble", (PyCFunction)Instr_disassemble, METH_VARARGS | METH_KEYWORDS, "description"},

    {NULL}  /* Sentinel */
};


static PyObject *Instr_repr(PyObject *selfx) {
    PyRabbitizerInstr *self = (PyRabbitizerInstr *)selfx;
    PyObject *ret;
    size_t disasmBufferSize;
    char *bufferStart;
    char *buffer;
    size_t typeNameLength;
    size_t extraSize = 3 + 8 + 4; // "(0x" + 32bits hex + ") # "
    int len;

    // self->ob_base.ob_type->tp_name is the type name (rabbitizer.Instr)
    typeNameLength = strlen(self->ob_base.ob_type->tp_name);

    disasmBufferSize = RabbitizerInstr_getSizeForBuffer(&self->instr, 0, 0);

    buffer = bufferStart = malloc(disasmBufferSize+1 + typeNameLength + extraSize);
    if (buffer == NULL) {
        // TODO: signal an exception?
        return NULL;
    }

    memcpy(buffer, self->ob_base.ob_type->tp_name, typeNameLength);
    buffer += typeNameLength;

    len = sprintf(buffer, "(0x%08X) # ", RabbitizerInstr_getRaw(&self->instr));
    if (len != 15) {
        // bad stuff
        // TODO: exception?
    }
    assert(len == 15);
    buffer += len;

    RabbitizerInstr_disassemble(&self->instr, buffer, NULL, 0, 0);

    ret = PyUnicode_FromString(bufferStart);
    free(bufferStart);
    return ret;
}

static PyObject *Instr_str(PyObject *selfx) {
    PyRabbitizerInstr *self = (PyRabbitizerInstr *)selfx;

    return Instr_disassemble(self, Py_BuildValue("()"), Py_BuildValue("{}"));
}

PyTypeObject rabbitizer_type_Instr_TypeObject = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.Instr",
    .tp_doc = PyDoc_STR("Instruction"),
    .tp_basicsize = sizeof(PyRabbitizerInstr),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = PyType_GenericNew,
    .tp_init = (initproc) Instr_init,
    .tp_dealloc = (destructor) Instr_dealloc,
    .tp_repr = Instr_repr,
    .tp_str = Instr_str,
    .tp_members = Instr_members,
    .tp_methods = Instr_methods,
    .tp_getset = Instr_getsetters,
};
