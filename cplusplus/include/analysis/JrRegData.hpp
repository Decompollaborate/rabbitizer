/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_JR_REG_DATA_HPP
#define RABBITIZER_JR_REG_DATA_HPP
#pragma once

#include "analysis/RabbitizerJrRegData.h"

namespace rabbitizer {
    class JrRegData {
    protected:
        RabbitizerJrRegData regData;

    public:
        JrRegData();
        JrRegData(const RabbitizerJrRegData &other);

        /**
         * Returns a pointer to the inner RabbitizerJrRegData.
         * It is recommended to not mess with it unless you know what you are doing.
         */
        RabbitizerJrRegData *getCPtr();
        const RabbitizerJrRegData *getCPtr() const;

        bool hasInfo() const;

        int offset() const;
        uint32_t address() const;
        bool checkedForBranching() const;
        int lastBranchOffset() const;
    };
}

#endif
