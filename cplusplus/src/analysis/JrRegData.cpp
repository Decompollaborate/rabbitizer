/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/JrRegData.hpp"

using namespace rabbitizer;

JrRegData::JrRegData() {
    RabbitizerJrRegData_init(&this->regData);
}
JrRegData::JrRegData(const RabbitizerJrRegData &other) {
    RabbitizerJrRegData_copy(&this->regData, &other);
}

RabbitizerJrRegData *JrRegData::getCPtr() {
    return &this->regData;
}
const RabbitizerJrRegData *JrRegData::getCPtr() const {
    return &this->regData;
}

bool JrRegData::hasInfo() const {
    return this->regData.hasInfo;
}

int JrRegData::offset() const {
    return this->regData.offset;
}
uint32_t JrRegData::address() const {
    return this->regData.address;
}
bool JrRegData::checkedForBranching() const {
    return this->regData.checkedForBranching;
}
int JrRegData::lastBranchOffset() const {
    return this->regData.lastBranchOffset;
}
