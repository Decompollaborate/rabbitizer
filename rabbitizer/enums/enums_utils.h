/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_ENUMS_UTILS_H
#define RABBITIZER_ENUMS_UTILS_H
#pragma once


#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include <stdbool.h>


extern PyTypeObject rabbitizer_type_Enum_TypeObject;


typedef struct PyRabbitizerEnum {
    PyObject_HEAD
    PyObject *enumType;
    PyObject *name;
    int value;
} PyRabbitizerEnum;

typedef struct RabbitizerEnumMetadata {
    const char *enumType;
    const char *name;
    int value;
    bool isInstanced;
    PyObject *instance;
} RabbitizerEnumMetadata;


int rabbitizer_EnumMetadata_Initialize(PyObject *submodule, RabbitizerEnumMetadata *enumValues);


#endif
