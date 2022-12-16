/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRID_HPP
#define RABBITIZER_INSTRID_HPP
#pragma once

#include <string>


namespace rabbitizer {
    namespace InstrId {
#include "UniqueId_enum_class.table.h"

        std::string getOpcodeName(UniqueId uniqueId);
    };
};


#endif
