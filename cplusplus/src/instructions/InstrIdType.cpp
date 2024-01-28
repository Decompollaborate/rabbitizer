/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstrIdType.hpp"

#include "instructions/RabbitizerInstrIdType.h"

using namespace rabbitizer;

std::string InstrIdType::getName(IdType idType) {
    RabInstrIdType id = static_cast<RabInstrIdType>(idType);

    return std::string(RabInstrIdType_getName(id));
}
