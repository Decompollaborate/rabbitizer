/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/LoPairingInfo.hpp"

using namespace rabbitizer;


LoPairingInfo::LoPairingInfo() {

}
LoPairingInfo::LoPairingInfo(const RabbitizerLoPairingInfo &pairingInfo) {
    this->pairingInfo = pairingInfo;
}

RabbitizerLoPairingInfo *LoPairingInfo::getCPtr() {
    return &this->pairingInfo;
}
const RabbitizerLoPairingInfo *LoPairingInfo::getCPtr() const{
    return &this->pairingInfo;
}

int LoPairingInfo::getInstrOffset() const {
    return this->pairingInfo.instrOffset;
}
int64_t LoPairingInfo::getValue() const {
    return this->pairingInfo.value;
}
bool LoPairingInfo::shouldProcess() const {
    return this->pairingInfo.shouldProcess;
}
bool LoPairingInfo::isGpRel() const {
    return this->pairingInfo.isGpRel;
}
bool LoPairingInfo::isGpGot() const {
    return this->pairingInfo.isGpGot;
}
