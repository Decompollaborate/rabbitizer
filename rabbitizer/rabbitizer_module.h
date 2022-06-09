/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"


extern PyTypeObject rabbitizer_global_config_TypeObject;
extern PyTypeObject rabbitizer_global_instr_id_TypeObject;

extern PyTypeObject rabbitizer_type_Enum_TypeObject;
extern PyTypeObject rabbitizer_type_Instruction_TypeObject;

PyObject *rabbitizer_enum_InstrCategory_Init(void);
