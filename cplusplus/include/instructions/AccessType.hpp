/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_ACCESS_TYPE_HPP
#define RABBITIZER_ACCESS_TYPE_HPP
#pragma once


namespace rabbitizer {
    #define RAB_DEF_ACCESSTYPE(name) name,

    enum class AccessType {
        #include "instructions/AccessType.inc"

        RAB_DEF_ACCESSTYPE(MAX)
    };

    #undef RAB_DEF_ACCESSTYPE
};


#endif
