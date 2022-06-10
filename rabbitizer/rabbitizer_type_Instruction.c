/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer_module.h"

#include "instructions/RabbitizerInstruction.h"


typedef struct PyRabbitizerInstruction {
    PyObject_HEAD
    RabbitizerInstruction instr;
} PyRabbitizerInstruction;

extern PyTypeObject rabbitizer_type_Instruction_TypeObject;


static void rabbitizer_type_Instruction_dealloc(PyRabbitizerInstruction *self) {
    RabbitizerInstruction_destroy(&self->instr);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

static int rabbitizer_type_Instruction_init(PyRabbitizerInstruction *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "word", NULL };
    uint32_t word;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "I", kwlist, &word)) {
        return -1;
    }

    RabbitizerInstruction_init(&self->instr, word);
    RabbitizerInstruction_processUniqueId(&self->instr);

    return 0;
}

static PyMemberDef rabbitizer_type_Instruction_members[] = {
    { "vram", T_UINT, offsetof(PyRabbitizerInstruction, instr.vram), 0, "vram description" },
    { "inHandwrittenFunction", T_BOOL, offsetof(PyRabbitizerInstruction, instr.inHandwrittenFunction), 0, "inHandwrittenFunction description" },
    { 0 }
};


#define DEF_MEMBER_GET_UINT(name) \
    static PyObject *rabbitizer_type_Instruction_member_get_##name(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromUnsignedLong(self->instr.name); \
    }

DEF_MEMBER_GET_UINT(rs)
DEF_MEMBER_GET_UINT(rt)
DEF_MEMBER_GET_UINT(rd)
DEF_MEMBER_GET_UINT(sa)

static PyObject *rabbitizer_type_Instruction_member_get_uniqueId(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) {
    PyObject *enumInstance = rabbitizer_enum_InstrId_enumvalues[self->instr.uniqueId].instance;

    if (enumInstance == NULL) {
        PyErr_SetString(PyExc_RuntimeError, "Internal error: invalid uniqueId enum value");
        return NULL;
    }

    Py_INCREF(enumInstance);
    return enumInstance;
}

#define MEMBER_GET(name, docs, closure)      { #name, (getter) rabbitizer_type_Instruction_member_get_##name, (setter) NULL,                                          PyDoc_STR(docs), closure }
#define MEMBER_SET(name, docs, closure)      { #name, (getter) NULL,                                          (setter) rabbitizer_type_Instruction_member_set_##name, PyDoc_STR(docs), closure }
#define MEMBER_GET_SET(name, docs, closure)  { #name, (getter) rabbitizer_type_Instruction_member_get_##name, (setter) rabbitizer_type_Instruction_member_set_##name, PyDoc_STR(docs), closure }


static PyGetSetDef Instr_getsetters[] = {
    MEMBER_GET(rs, "", NULL), // todo: deprecate
    MEMBER_GET(rt, "", NULL), // todo: deprecate
    MEMBER_GET(rd, "", NULL), // todo: deprecate
    MEMBER_GET(sa, "", NULL), // todo: deprecate
    MEMBER_GET(uniqueId, "", NULL),

    { 0 }
};


#define DEF_METHOD_GET_UINT(name) \
    static PyObject *rabbitizer_type_Instruction_##name(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromUnsignedLong(RabbitizerInstruction_##name(&self->instr)); \
    }

#define DEF_METHOD_GET_INT(name) \
    static PyObject *rabbitizer_type_Instruction_##name(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) { \
        return PyLong_FromLong(RabbitizerInstruction_##name(&self->instr)); \
    }

DEF_METHOD_GET_UINT(getRaw)
DEF_METHOD_GET_UINT(getImmediate)
DEF_METHOD_GET_UINT(getInstrIndexAsVram)
DEF_METHOD_GET_INT(getBranchOffset)

static PyObject *rabbitizer_type_Instruction_getGenericBranchOffset(PyRabbitizerInstruction *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "currentVram", NULL };
    uint32_t currentVram;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "I", kwlist, &currentVram)) {
        return NULL;
    }

    return PyLong_FromLong(RabbitizerInstruction_getGenericBranchOffset(&self->instr, currentVram));
}

static PyObject *rabbitizer_type_Instruction_blankOut(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) {
    RabbitizerInstruction_blankOut(&self->instr);
    Py_RETURN_NONE;
}


#define DEF_METHOD_BOOL(name) \
    static PyObject *rabbitizer_type_Instruction_##name(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstruction_##name(&self->instr)) { \
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

static PyObject *rabbitizer_type_Instruction_mapInstrToType(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) {
    const char *type = RabbitizerInstruction_mapInstrToType(&self->instr);

    if (type != NULL) {
        return PyUnicode_FromString(type);
    }
    Py_RETURN_NONE;
}

static PyObject *rabbitizer_type_Instruction_sameOpcode(PyRabbitizerInstruction *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "other", NULL };
    PyRabbitizerInstruction *other;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!", kwlist, &rabbitizer_type_Instruction_TypeObject, &other)) {
        return NULL;
    }

    if (RabbitizerInstruction_sameOpcode(&self->instr, &other->instr)) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

static PyObject *rabbitizer_type_Instruction_sameOpcodeButDifferentArguments(PyRabbitizerInstruction *self, PyObject *args, PyObject *kwds) {
    static char *kwlist[] = { "other", NULL };
    PyRabbitizerInstruction *other;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "O!", kwlist, &rabbitizer_type_Instruction_TypeObject, &other)) {
        return NULL;
    }

    if (RabbitizerInstruction_sameOpcodeButDifferentArguments(&self->instr, &other->instr)) {
        Py_RETURN_TRUE;
    }
    Py_RETURN_FALSE;
}

#define DEF_DESCRIPTOR_METHOD_BOOL(name) \
    static PyObject *rabbitizer_type_Instruction_##name(PyRabbitizerInstruction *self, PyObject *Py_UNUSED(ignored)) { \
        if (RabbitizerInstrDescriptor_##name(self->instr.descriptor)) { \
            Py_RETURN_TRUE; \
        } \
        Py_RETURN_FALSE; \
    }

DEF_DESCRIPTOR_METHOD_BOOL(isUnknownType)
DEF_DESCRIPTOR_METHOD_BOOL(isJType)
DEF_DESCRIPTOR_METHOD_BOOL(isIType)
DEF_DESCRIPTOR_METHOD_BOOL(isRType)
DEF_DESCRIPTOR_METHOD_BOOL(isRegimmType)
DEF_DESCRIPTOR_METHOD_BOOL(isBranch)
DEF_DESCRIPTOR_METHOD_BOOL(isBranchLikely)
DEF_DESCRIPTOR_METHOD_BOOL(isJump)
DEF_DESCRIPTOR_METHOD_BOOL(isTrap)
DEF_DESCRIPTOR_METHOD_BOOL(isFloat)
DEF_DESCRIPTOR_METHOD_BOOL(isDouble)
DEF_DESCRIPTOR_METHOD_BOOL(isUnsigned)
DEF_DESCRIPTOR_METHOD_BOOL(modifiesRt)
DEF_DESCRIPTOR_METHOD_BOOL(modifiesRd)
DEF_DESCRIPTOR_METHOD_BOOL(notEmitedByCompilers)
DEF_DESCRIPTOR_METHOD_BOOL(isHiPair)
DEF_DESCRIPTOR_METHOD_BOOL(isLoPair)
DEF_DESCRIPTOR_METHOD_BOOL(doesLink)
DEF_DESCRIPTOR_METHOD_BOOL(doesDereference)


static PyObject *rabbitizer_type_Instruction_disassemble(PyRabbitizerInstruction *self, PyObject *args, PyObject *kwds) {
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

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&self->instr, immOverrideLength, extraLJust);

    buffer = malloc(bufferSize+1);
    if (buffer == NULL) {
        // TODO: signal an exception?
        return NULL;
    }

    RabbitizerInstruction_disassemble(&self->instr, buffer, immOverride, immOverrideLength, extraLJust);

    ret = PyUnicode_FromString(buffer);
    free(buffer);
    return ret;
}


#define METHOD_NO_ARGS(name, docs)  { #name, (PyCFunction)rabbitizer_type_Instruction_##name, METH_NOARGS,                  PyDoc_STR(docs) }
#define METHOD_ARGS(name, docs)     { #name, (PyCFunction)rabbitizer_type_Instruction_##name, METH_VARARGS | METH_KEYWORDS, PyDoc_STR(docs) }


static PyMethodDef Instr_methods[] = {
    METHOD_NO_ARGS(getRaw, ""),
    METHOD_NO_ARGS(getImmediate, ""),
    METHOD_NO_ARGS(getInstrIndexAsVram, ""),
    METHOD_NO_ARGS(getBranchOffset, ""),
    METHOD_ARGS(getGenericBranchOffset, ""),

    METHOD_NO_ARGS(blankOut, ""),

    METHOD_NO_ARGS(isImplemented, ""),
    METHOD_NO_ARGS(isLikelyHandwritten, ""),
    METHOD_NO_ARGS(isNop, ""),
    METHOD_NO_ARGS(isUnconditionalBranch, ""),
    METHOD_NO_ARGS(isJrRa, ""),
    METHOD_NO_ARGS(isJrNotRa, ""),
    METHOD_NO_ARGS(mapInstrToType, ""),

    METHOD_ARGS(sameOpcode, "description"),
    METHOD_ARGS(sameOpcodeButDifferentArguments, "description"),

    METHOD_NO_ARGS(isUnknownType, ""),
    METHOD_NO_ARGS(isJType, ""),
    METHOD_NO_ARGS(isIType, ""),
    METHOD_NO_ARGS(isRType, ""),
    METHOD_NO_ARGS(isRegimmType, ""),
    METHOD_NO_ARGS(isBranch, ""),
    METHOD_NO_ARGS(isBranchLikely, ""),
    METHOD_NO_ARGS(isJump, ""),
    METHOD_NO_ARGS(isTrap, ""),
    METHOD_NO_ARGS(isFloat, ""),
    METHOD_NO_ARGS(isDouble, ""),
    METHOD_NO_ARGS(isUnsigned, ""),
    METHOD_NO_ARGS(modifiesRt, ""),
    METHOD_NO_ARGS(modifiesRd, ""),
    METHOD_NO_ARGS(notEmitedByCompilers, ""),
    METHOD_NO_ARGS(isHiPair, ""),
    METHOD_NO_ARGS(isLoPair, ""),
    METHOD_NO_ARGS(doesLink, ""),
    METHOD_NO_ARGS(doesDereference, ""),

    METHOD_ARGS(disassemble, "description"),

    { 0 },
};


static PyObject *rabbitizer_type_Instruction_repr(PyRabbitizerInstruction *self) {
    PyObject *ret;
    size_t disasmBufferSize;
    char *bufferStart;
    char *buffer;
    size_t typeNameLength;
    size_t extraSize = 3 + 8 + 4; // "(0x" + 32bits hex + ") # "
    int len;

    typeNameLength = strlen("rabbitizer.Instruction");

    disasmBufferSize = RabbitizerInstruction_getSizeForBuffer(&self->instr, 0, 0);

    buffer = bufferStart = malloc(disasmBufferSize+1 + typeNameLength + extraSize);
    if (buffer == NULL) {
        // TODO: signal an exception?
        return NULL;
    }

    memcpy(buffer, "rabbitizer.Instruction", typeNameLength);
    buffer += typeNameLength;

    len = sprintf(buffer, "(0x%08X) # ", RabbitizerInstruction_getRaw(&self->instr));
    if (len != 15) {
        // bad stuff
        // TODO: exception?
    }
    assert(len == 15);
    buffer += len;

    RabbitizerInstruction_disassemble(&self->instr, buffer, NULL, 0, 0);

    ret = PyUnicode_FromString(bufferStart);
    free(bufferStart);
    return ret;
}

static PyObject *rabbitizer_type_Instruction_str(PyRabbitizerInstruction *self) {
    return rabbitizer_type_Instruction_disassemble(self, Py_BuildValue("()"), Py_BuildValue("{}"));
}


PyTypeObject rabbitizer_type_Instruction_TypeObject = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "rabbitizer.Instruction",
    .tp_doc = PyDoc_STR("Instruction"),
    .tp_basicsize = sizeof(PyRabbitizerInstruction),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = PyType_GenericNew,
    .tp_init = (initproc) rabbitizer_type_Instruction_init,
    .tp_dealloc = (destructor) rabbitizer_type_Instruction_dealloc,
    .tp_repr = (reprfunc) rabbitizer_type_Instruction_repr,
    .tp_str = (reprfunc) rabbitizer_type_Instruction_str,
    .tp_members = rabbitizer_type_Instruction_members,
    .tp_methods = Instr_methods,
    .tp_getset = Instr_getsetters,
};
