/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums/enums_utils.h"
#include "instructions/RabbitizerRegister.h"


#define RABBITIZER_DEF_REG(prefix, name, numeric, ...) \
    { "RegCop1N32", #name, RABBITIZER_REG_##prefix##_##name, false, NULL },

#define RABBITIZER_DEF_REG_NODOLLAR(prefix, name, numeric, ...) \
    RABBITIZER_DEF_REG(prefix, name, numeric, __VARGS__)

#define RABBITIZER_DEF_REG_NONODOLLAR(prefix, name, numeric, ...) \
    RABBITIZER_DEF_REG(prefix, name, numeric, __VARGS__)

RabbitizerEnumMetadata rabbitizer_enum_RegCop1N32_enumvalues[] = {
    #include "tables/registers/RabbitizerRegister_Cop1N32.inc"

    { 0 },
};

#undef RABBITIZER_DEF_REG
#undef RABBITIZER_DEF_REG_NODOLLAR
#undef RABBITIZER_DEF_REG_NONODOLLAR


static PyMethodDef rabbitizer_enum_RegCop1N32_methods[] = {
    { 0 },
};

DEF_ENUM(RegCop1N32, "")
