/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_MODULE_H
#define RABBITIZER_MODULE_H
#pragma once

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"

#include "enums/enums_utils.h"

// TODO: clean up this...

extern RabbitizerEnumMetadata rabbitizer_enum_Abi_enumvalues[];
extern RabbitizerEnumMetadata rabbitizer_enum_InstrId_enumvalues[];

int rabbitizer_enum_Abi_Check(PyObject *o);


PyObject *rabbitizer_submodule_Utils_Init(void);

extern PyTypeObject rabbitizer_global_config_TypeObject;

extern PyTypeObject rabbitizer_type_Enum_TypeObject;
extern PyTypeObject rabbitizer_type_Instruction_TypeObject;

PyObject *rabbitizer_enum_Abi_Init(void);
PyObject *rabbitizer_enum_InstrCategory_Init(void);
PyObject *rabbitizer_enum_InstrId_Init(void);

#endif
