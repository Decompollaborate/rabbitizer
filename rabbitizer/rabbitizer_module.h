/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include "structmember.h"


extern PyTypeObject rabbitizer_type_Instr_TypeObject;

extern PyTypeObject rabbitizer_global_config_TypeObject;

PyObject *rabbitizer_submodule_instr_id_Init(void);
