/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/RabbitizerRegistersTracker.h"

#include <assert.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"


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
        if (RabbitizerInstrDescriptor_isHiPair(instr->descriptor)) {
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
    RabbitizerTrackedRegisterState *state = NULL;

    if (!RabbitizerInstrDescriptor_doesLink(prevInstr->descriptor)) {
        return;
    }

    if (RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_O32 || RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_NUMERIC) {
        for (size_t reg = 0; reg < ARRAY_COUNT(self->registers); reg++) {
            switch (reg) {
                case RABBITIZER_REG_GPR_O32_at:
                case RABBITIZER_REG_GPR_O32_v0:
                case RABBITIZER_REG_GPR_O32_v1:
                case RABBITIZER_REG_GPR_O32_a0:
                case RABBITIZER_REG_GPR_O32_a1:
                case RABBITIZER_REG_GPR_O32_a2:
                case RABBITIZER_REG_GPR_O32_a3:
                case RABBITIZER_REG_GPR_O32_t0:
                case RABBITIZER_REG_GPR_O32_t1:
                case RABBITIZER_REG_GPR_O32_t2:
                case RABBITIZER_REG_GPR_O32_t3:
                case RABBITIZER_REG_GPR_O32_t4:
                case RABBITIZER_REG_GPR_O32_t5:
                case RABBITIZER_REG_GPR_O32_t6:
                case RABBITIZER_REG_GPR_O32_t7:
                case RABBITIZER_REG_GPR_O32_t8:
                case RABBITIZER_REG_GPR_O32_t9:
                case RABBITIZER_REG_GPR_O32_ra:
                    state = &self->registers[reg];
                    #if 0
                    if (state.hasLuiValue) {
                        self->_printDebugInfo_clearRegister(instr, reg)
                    }
                    #endif
                    RabbitizerTrackedRegisterState_clear(state);
                    break;

                default:
                    break;
            }
        }
    } else if (RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_N32 || RabbitizerConfig_Cfg.regNames.gprAbiNames == RABBITIZER_ABI_N64) {
        for (size_t reg = 0; reg < ARRAY_COUNT(self->registers); reg++) {
            switch (reg) {
                case RABBITIZER_REG_GPR_N32_at:
                case RABBITIZER_REG_GPR_N32_v0:
                case RABBITIZER_REG_GPR_N32_v1:
                case RABBITIZER_REG_GPR_N32_a0:
                case RABBITIZER_REG_GPR_N32_a1:
                case RABBITIZER_REG_GPR_N32_a2:
                case RABBITIZER_REG_GPR_N32_a3:
                case RABBITIZER_REG_GPR_N32_a4:
                case RABBITIZER_REG_GPR_N32_a5:
                case RABBITIZER_REG_GPR_N32_a6:
                case RABBITIZER_REG_GPR_N32_a7:
                case RABBITIZER_REG_GPR_N32_t0:
                case RABBITIZER_REG_GPR_N32_t1:
                case RABBITIZER_REG_GPR_N32_t2:
                case RABBITIZER_REG_GPR_N32_t3:
                case RABBITIZER_REG_GPR_N32_t8:
                case RABBITIZER_REG_GPR_N32_t9:
                case RABBITIZER_REG_GPR_N32_ra:
                    state = &self->registers[reg];
                    #if 0
                    if (state.hasLuiValue) {
                        self->_printDebugInfo_clearRegister(instr, reg)
                    }
                    #endif
                    RabbitizerTrackedRegisterState_clear(state);
                    break;

                default:
                    break;
            }
        }
    }
}

bool RabbitizerRegistersTracker_getAddressIfCanSetType(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, int *dstAddress) {
    RabbitizerTrackedRegisterState *state = &self->registers[instr->rs];

    if (!state->hasLoValue) {
        return false;
    }

    if (!state->dereferenced || instrOffset == state->dereferenceOffset) {
        *dstAddress = state->value;
        return true;
    }

    return false;
}

bool RabbitizerRegistersTracker_getJrInfo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset, int *dstAddress) {
    RabbitizerTrackedRegisterState *state = &self->registers[instr->rs];

    if (!state->hasLoValue || !state->dereferenced) {
        return false;
    }

    *dstOffset = state->loOffset;
    *dstAddress = state->value;
    return true;
}


// prevInstr can be NULL
void RabbitizerRegistersTracker_processLui(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, const RabbitizerInstruction *prevInstr, int instrOffset) {
    RabbitizerTrackedRegisterState *state = NULL;

    assert(RabbitizerInstrDescriptor_isHiPair(instr->descriptor));

    state = &self->registers[instr->rt];
    RabbitizerTrackedRegisterState_clear(state);
    RabbitizerTrackedRegisterState_setHi(state, RabbitizerInstruction_getImmediate(instr), instrOffset);

    if (prevInstr != NULL) {
        // If the previous instructions is a branch likely, then nulify
        // the effects of this instruction for future analysis
        state->luiSetOnBranchLikely = RabbitizerInstrDescriptor_isBranchLikely(prevInstr->descriptor) || RabbitizerInstruction_isUnconditionalBranch(prevInstr);
    }
}

bool RabbitizerRegistersTracker_getLuiOffsetForConstant(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset) {
    RabbitizerTrackedRegisterState *state = &self->registers[instr->rs];

    if (!state->hasLuiValue) {
        return false;
    }

    *dstOffset = state->luiOffset;
    return true;
}

void RabbitizerRegistersTracker_processConstant(RabbitizerRegistersTracker *self, int value, const RabbitizerInstruction *instr, int offset) {
    RabbitizerTrackedRegisterState *stateDst = &self->registers[instr->rt];

    RabbitizerTrackedRegisterState_setLo(stateDst, value, offset);
}

bool RabbitizerRegistersTracker_getLuiOffsetForLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, int *dstOffset, bool *dstIsGp) {
    RabbitizerTrackedRegisterState *state = &self->registers[instr->rs];

    if (state->hasLuiValue && !state->luiSetOnBranchLikely) {
        *dstOffset = state->luiOffset;
        *dstIsGp = false;
        return true;
    }

    // TODO: abi
    if (instr->rs == 28) { // $gp
        *dstOffset = 0;
        *dstIsGp = true;
        return true;
    }

    if (RabbitizerInstrDescriptor_modifiesRt(instr->descriptor) && RabbitizerInstrDescriptor_doesDereference(instr->descriptor)) {
        if (state->hasLoValue && !state->dereferenced) {
            // Simulate a dereference
            RabbitizerTrackedRegisterState_dereferenceState(&self->registers[instr->rt], state, instrOffset);
        }
    }

    return false;
}

void RabbitizerRegistersTracker_processLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int value, int offset) {
    RabbitizerTrackedRegisterState *stateDst;

    if (!RabbitizerInstrDescriptor_modifiesRt(instr)) {
        return;
    }

    stateDst = &self->registers[instr->rt];
    RabbitizerTrackedRegisterState_setLo(stateDst, value, offset);
    if (RabbitizerInstrDescriptor_doesDereference(instr->descriptor)) {
        RabbitizerTrackedRegisterState_deref(stateDst, offset);
    }
    if (instr->rt == instr->rs) {
        RabbitizerTrackedRegisterState_clearHi(stateDst);
    }
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
