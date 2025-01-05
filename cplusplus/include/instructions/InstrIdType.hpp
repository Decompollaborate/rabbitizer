/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTR_ID_TYPE_HPP
#define RABBITIZER_INSTR_ID_TYPE_HPP
#pragma once

#include <string>


namespace rabbitizer {
    namespace InstrIdType {
#include "generated/opcode_category_enum_class.hpp"

        std::string getName(IdType idType);
    };
};


#endif
