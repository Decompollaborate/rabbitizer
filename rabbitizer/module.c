#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "instructions/RabbitizerInstr.h"

typedef struct PyRabbitizerInstr {
    PyObject_HEAD
    //PyObject *first; /* first name */
    //PyObject *last;  /* last name */
    //int number;
    RabbitizerInstr instr;
} PyRabbitizerInstr;

static void
Instr_dealloc(PyRabbitizerInstr *self)
{
    RabbitizerInstr_Destroy(&self->instr);
    Py_TYPE(self)->tp_free((PyObject *) self);
}

#if 0
static PyObject *
Instr_new(PyTypeObject *type, PyObject *args, PyObject *kwds)
{
    PyRabbitizerInstr *self;
    self = (PyRabbitizerInstr *) type->tp_alloc(type, 0);
    if (self != NULL) {
        self->first = PyUnicode_FromString("");
        if (self->first == NULL) {
            Py_DECREF(self);
            return NULL;
        }
        self->last = PyUnicode_FromString("");
        if (self->last == NULL) {
            Py_DECREF(self);
            return NULL;
        }
        self->number = 0;
    }
    return (PyObject *) self;
}
#endif

static int
Instr_init(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds)
{
    static char *kwlist[] = {"word", NULL};
    uint32_t word;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "I", kwlist, &word))
        return -1;

    RabbitizerInstr_Init(&self->instr, word);
    RabbitizerInstr_ProcessUniqueId(&self->instr);

    return 0;
}

static PyMemberDef Instr_members[] = {
    {"extraLjustWidthOpcode", T_INT, offsetof(PyRabbitizerInstr, instr.extraLjustWidthOpcode), 0,
     "extraLjustWidthOpcode description"},
    {"vram", T_UINT, offsetof(PyRabbitizerInstr, instr.vram), 0,
     "vram description"},
    {NULL}  /* Sentinel */
};

#if 0
static PyObject *
Instr_getfirst(PyRabbitizerInstr *self, void *closure)
{
    Py_INCREF(self->first);
    return self->first;
}

static int
Instr_setfirst(PyRabbitizerInstr *self, PyObject *value, void *closure)
{
    PyObject *tmp;
    if (value == NULL) {
        PyErr_SetString(PyExc_TypeError, "Cannot delete the first attribute");
        return -1;
    }
    if (!PyUnicode_Check(value)) {
        PyErr_SetString(PyExc_TypeError,
                        "The first attribute value must be a string");
        return -1;
    }
    tmp = self->first;
    Py_INCREF(value);
    self->first = value;
    Py_DECREF(tmp);
    return 0;
}

static PyObject *
Instr_getlast(PyRabbitizerInstr *self, void *closure)
{
    Py_INCREF(self->last);
    return self->last;
}

static int
Instr_setlast(PyRabbitizerInstr *self, PyObject *value, void *closure)
{
    PyObject *tmp;
    if (value == NULL) {
        PyErr_SetString(PyExc_TypeError, "Cannot delete the last attribute");
        return -1;
    }
    if (!PyUnicode_Check(value)) {
        PyErr_SetString(PyExc_TypeError,
                        "The last attribute value must be a string");
        return -1;
    }
    tmp = self->last;
    Py_INCREF(value);
    self->last = value;
    Py_DECREF(tmp);
    return 0;
}
#endif

static PyGetSetDef Instr_getsetters[] = {
#if 0
    {"first", (getter) Instr_getfirst, (setter) Instr_setfirst,
     "first name", NULL},
    {"last", (getter) Instr_getlast, (setter) Instr_setlast,
     "last name", NULL},
#endif
    {NULL}  /* Sentinel */
};

#if 0
static PyObject *
Instr_name(PyRabbitizerInstr *self, PyObject *Py_UNUSED(ignored))
{
    return PyUnicode_FromFormat("%S %S", self->first, self->last);
}
#endif


static PyObject *
Instr_Disassemble(PyRabbitizerInstr *self, PyObject *args, PyObject *kwds)
{
    static char *kwlist[] = {"immOverride", NULL};
    const char *immOverride = NULL;
    size_t immOverrideLength = 0;
    size_t bufferSize;
    char *buffer;
    PyObject *ret;

    if (!PyArg_ParseTupleAndKeywords(args, kwds, "|z", kwlist, &immOverride)) {
        return NULL;
    }

    if (immOverride != NULL) {
        immOverrideLength = strlen(immOverride);
    }

    bufferSize = RabbitizerInstr_GetSizeForBuffer(&self->instr, immOverrideLength);

    buffer = malloc(bufferSize+1);
    if (buffer == NULL) {
        return NULL;
    }

    RabbitizerInstr_Disassemble(&self->instr, buffer, immOverride, immOverrideLength);

    ret = PyUnicode_FromString(buffer);

    free(buffer);

    return ret;
}

static PyMethodDef Instr_methods[] = {
#if 0
    {"name", (PyCFunction) Instr_name, METH_NOARGS,
     "Return the name, combining the first and last name"
    },
#endif
    {"disassemble", (PyCFunction) Instr_Disassemble, METH_VARARGS | METH_KEYWORDS, "description"},
    {NULL}  /* Sentinel */
};

static PyTypeObject InstrType = {
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

static PyModuleDef rabbitizer_module = {
    PyModuleDef_HEAD_INIT,
    .m_name = "rabbitizer",
    .m_doc = "",
    .m_size = -1,
};

PyMODINIT_FUNC
PyInit_rabbitizer(void)
{
    PyObject *m;
    if (PyType_Ready(&InstrType) < 0)
        return NULL;

    m = PyModule_Create(&rabbitizer_module);
    if (m == NULL)
        return NULL;

    Py_INCREF(&InstrType);
    if (PyModule_AddObject(m, "Instr", (PyObject *) &InstrType) < 0) {
        Py_DECREF(&InstrType);
        Py_DECREF(m);
        return NULL;
    }

    return m;
}
