/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/InstrId.hpp"

#include "instructions/RabbitizerInstrId.h"

using namespace rabbitizer;


std::string getOpcodeName(InstrId::UniqueId uniqueId) {
    RabbitizerInstrId id = static_cast<RabbitizerInstrId>(uniqueId);

    return std::string(RabbitizerInstrId_getOpcodeName(id));
}
