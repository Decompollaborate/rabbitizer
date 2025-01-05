/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums/enums_utils.h"
#include "instructions/RabbitizerRegister.h"


#define RABBITIZER_DEF_REG(prefix, name, numeric, ...) \
    { "RegCop1O32", #name, RABBITIZER_REG_##prefix##_##name, false, NULL },

#define RABBITIZER_DEF_REG_NODOLLAR(prefix, name, numeric, ...) \
    RABBITIZER_DEF_REG(prefix, name, numeric, __VARGS__)

RabbitizerEnumMetadata rabbitizer_enum_RegCop1O32_enumvalues[] = {
    #include "tables/registers/RabbitizerRegister_Cop1O32.inc"

    { 0 },
};

#undef RABBITIZER_DEF_REG
#undef RABBITIZER_DEF_REG_NODOLLAR


static PyMethodDef rabbitizer_enum_RegCop1O32_methods[] = {
    { 0 },
};

DEF_ENUM(RegCop1O32, "")
