/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/RabbitizerRegistersTracker.h"

#include "common/Utils.h"


// TODO: abi checks

void RabbitizerRegistersTracker_init(RabbitizerRegistersTracker *self, const RabbitizerRegistersTracker *other) {
    for (size_t i = 0; i < ARRAY_COUNT(self->registers); i++) {
        RabbitizerTrackedRegisterState_init(&self->registers[i], i);
        if (other != NULL) {
            RabbitizerTrackedRegisterState_copyState(&self->registers[i], &other->registers[i]);
        }
    }
}

void RabbitizerRegistersTracker_destroy(RabbitizerRegistersTracker *self) {
    for (size_t i = 0; i < ARRAY_COUNT(self->registers); i++) {
        RabbitizerTrackedRegisterState_destroy(&self->registers[i]);
    }
}


bool RabbitizerRegistersTracker_moveRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr) {
    RabbitizerTrackedRegisterState *dstState;
    RabbitizerTrackedRegisterState *srcState;
    uint8_t reg;

    if (!RabbitizerInstrDescriptor_maybeIsMove(instr->descriptor)) {
        return false;
    }
    if (instr->rt == 0 && instr->rs == 0) {
        // ?
        // RabbitizerTrackedRegisterState_clear(dstState);
        return false;
    }

    if (instr->rt == 0) {
        reg = instr->rs;
    } else if (instr->rs == 0) {
        reg = instr->rt;
    } else {
        // Check stuff like  `addu   $3, $3, $2`
        if (instr->rd == instr->rs) {
            reg = instr->rt;
            if (self->registers[instr->rs].hasLuiValue) {
                reg = instr->rs;
            }
        } else if (instr->rd == instr->rt) {
            reg = instr->rs;
            if (self->registers[instr->rt].hasLuiValue) {
                reg = instr->rt;
            }
        } else {
            // ?
            // RabbitizerTrackedRegisterState_clear(dstState);
            return false;
        }

        srcState = &self->registers[reg];
        RabbitizerTrackedRegisterState_copyState(&self->registers[instr->rd], srcState);
        return true;
    }

    srcState = &self->registers[reg];
    dstState = &self->registers[instr->rd];

    if (srcState->hasLoValue || srcState->hasLuiValue) {
        RabbitizerTrackedRegisterState_copyState(dstState, srcState);
        return true;
    }

    RabbitizerTrackedRegisterState_clear(dstState);
    return false;
}

void RabbitizerRegistersTracker_overwriteRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset) {
    bool shouldRemove = false;
    uint8_t reg = 0;
    RabbitizerTrackedRegisterState *state = NULL;

    if (RabbitizerRegistersTracker_moveRegisters(self, instr)) {
        return;
    }

    if (RabbitizerInstrDescriptor_isFloat(instr->descriptor)) {
        switch (instr->uniqueId) {
            case RABBITIZER_INSTR_ID_cpu_mtc1:
            case RABBITIZER_INSTR_ID_cpu_dmtc1:
            case RABBITIZER_INSTR_ID_cpu_ctc1:
                // IDO usually use a reg as a temp when loading a constant value
                // into the float coprocessor, after that IDO never re-uses the value
                // in that reg for anything else
                shouldRemove = true;
                reg = instr->rt;
                break;

            default:
                break;
        }
    } else if (RabbitizerInstrDescriptor_isRType(instr->descriptor) || (RabbitizerInstrDescriptor_isBranch(instr->descriptor) && RabbitizerInstrDescriptor_isIType(instr->descriptor))) {
        // $at usually is a one-use reg
        uint8_t at = 0;

        if (instr->rs == 1) {
            at = instr->rs;
        } else if (instr->rt == 1) {
            at = instr->rt;
        }

        state = &self->registers[at];
        if (state->hasLoValue || state->hasLuiValue) {
            shouldRemove = true;
            reg = at;
        }
    }

    if (RabbitizerInstrDescriptor_modifiesRt(instr->descriptor)) {
        shouldRemove = true;
        reg = instr->rt;
        if (RabbitizerInstrDescriptor_isHiPair(instr)) {
            RabbitizerTrackedRegisterState_clearLo(&self->registers[instr->rt]);
            shouldRemove = false;
        }
    }
    if (RabbitizerInstrDescriptor_modifiesRd(instr->descriptor)) {
        shouldRemove = true;
        reg = instr->rd;
    }

    if (shouldRemove) {
        state = &self->registers[reg];
        #if 0
        if (state->hasLuiValue) {
            self->_printDebugInfo_clearRegister(instr, reg)
        }
        #endif

        RabbitizerTrackedRegisterState_clearHi(state);
        if (!RabbitizerTrackedRegisterState_wasSetInCurrentOffset(state, instrOffset)) {
            RabbitizerTrackedRegisterState_clearLo(state);
        }
    }
}

void RabbitizerRegistersTracker_unsetRegistersAfterFuncCall(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, const RabbitizerInstruction *prevInstr) {
    #if 0
    if not previnstr->doesLink():
        return

    // It happen that every reg that we want to clear on a function call have the same raw values for both o32 and n32 ABIs, so no need to worry about it for now...
    registersToInvalidate = (
        1,  // $at
        2,  // $v0
        3,  // $v1
        4,  // $a0
        5,  // $a1
        6,  // $a2
        7,  // $a3
        8,  // $t0 / $a4
        9,  // $t1 / $a5
        10, // $t2 / $a6
        11, // $t3 / $a7
        12, // $t4 / $t0
        13, // $t5 / $t1
        14, // $t6 / $t2
        15, // $t7 / $t3
        24, // $t8 / $t8
        25, // $t9 / $t9
        31, // $ra
    )

    for reg in registersToInvalidate:
        state = self->registers[reg]
        if state.hasLuiValue:
            self->_printDebugInfo_clearRegister(instr, reg, currentVram)
        state.clear()
    #endif
}

bool RabbitizerRegistersTracker_getAddressIfCanSetType(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, int *dstAddress) {
    #if 0
    state = self->registers[instr->rs]
    if not state.hasLoValue:
        return None

    if not state.dereferenced or instrOffset == state.dereferenceOffset:
        return state.value

    return None
    #endif
    return false;
}

#if 0
def getJrInfo(self, instr: rabbitizer.Instruction) -> tuple[int, int]|None:
    state = self->registers[instr->rs]
    if state.hasLoValue and state.dereferenced:
        return state.loOffset, state.value
    return None
#endif


void RabbitizerRegistersTracker_processLui(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, const RabbitizerInstruction *prevInstr, int instrOffset) {
    #if 0
    assert instr->isHiPair()

    state = self->registers[instr->rt]
    state.clear()
    state.setHi(instr->getImmediate(), instrOffset)

    if prevInstr is not None:
        // If the previous instructions is a branch likely, then nulify
        // the effects of this instruction for future analysis
        state.luiSetOnBranchLikely = previnstr->isBranchLikely() or previnstr->isUnconditionalBranch()
    #endif
}

#if 0
def getLuiOffsetForConstant(self, instr: rabbitizer.Instruction) -> int|None:
    stateSrc = self->registers[instr->rs]
    if stateSrc.hasLuiValue:
        return stateSrc.luiOffset
    return None
#endif

void RabbitizerRegistersTracker_processConstant(RabbitizerRegistersTracker *self, int value, const RabbitizerInstruction *instr, int offset) {
    #if 0
    stateDst = self->registers[instr->rt]
    stateDst.setLo(value, offset)
    #endif
}

#if 0
def getLuiOffsetForLo(self, instr: rabbitizer.Instruction, instrOffset: int) -> tuple[int|None, bool]:
    stateSrc = self->registers[instr->rs]
    if stateSrc.hasLuiValue and not stateSrc.luiSetOnBranchLikely:
        return stateSrc.luiOffset, True

    if instr->rs == 28: // $gp
        return None, True

    if instr->modifiesRt() and instr->doesDereference():
        if stateSrc.hasLoValue and not stateSrc.dereferenced:
            // Simulate a dereference
            self->registers[instr->rt].dereferenceState(stateSrc, instrOffset)
    return None, False
#endif

void RabbitizerRegistersTracker_processLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int value, int offset) {
    #if 0
    if not instr->modifiesRt():
        return

    stateDst = self->registers[instr->rt]
    stateDst.setLo(value, offset)
    if instr->doesDereference():
        stateDst.deref(offset)
    if instr->rt == instr->rs:
        stateDst.clearHi()
    #endif
}


#if 0
def _printDebugInfo_clearRegister(self, instr: rabbitizer.Instruction, reg: int, currentVram: int|None=None) -> None:
    if not common.GlobalConfig.PRINT_SYMBOL_FINDER_DEBUG_INFO:
        return

    if currentVram is None:
        return

    print("Clearing reg:")
    // print()
    print(f"vram: {currentVram:X}")
    print(instr)
    print(self->registers)
    // TODO
    // print(f"deleting {reg} / {instr->getRegisterName(reg)}")
    print()
#endif
