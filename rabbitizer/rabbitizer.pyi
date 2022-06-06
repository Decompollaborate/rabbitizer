#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations


class Instr:
    opcode: int
    rs: int
    rt: int
    rd: int
    sa: int
    function: int

    uniqueId: int

    vram: int = 0
    inHandwrittenFunction: bool = False

    instr: int
    immediate: int

    def __init__(self, word: int) -> None: ...

    def getRaw(self) -> int: ...
    def getImmediate(self) -> int: ...
    def getInstrIndexAsVram(self) -> int: ...
    def getBranchOffset(self) -> int: ...

    def blankOut(self) -> None: ...

    def isImplemented(self) -> bool: ...
    def isLikelyHandwritten(self) -> bool: ...
    def isNop(self) -> bool: ...
    def isUnconditionalBranch(self) -> bool: ...
    def isJrRa(self) -> bool: ...
    def isJrNotRa(self) -> bool: ...
    def mapInstrToType(self) -> str|None: ...

    def sameOpcode(self, other: Instr) -> bool: ...
    def sameOpcodeButDifferentArguments(self, other: Instr) -> bool: ...

    def isJType(self) -> bool: ...
    def isIType(self) -> bool: ...
    def isRType(self) -> bool: ...
    def isBranch(self) -> bool: ...
    def isBranchLikely(self) -> bool: ...
    def isJump(self) -> bool: ...
    def isTrap(self) -> bool: ...
    def isFloat(self) -> bool: ...
    def isFloatInstruction(self) -> bool: ...
    def isDouble(self) -> bool: ...
    def isDoubleInstruction(self) -> bool: ...
    def isUnsigned(self) -> bool: ...
    def modifiesRt(self) -> bool: ...
    def modifiesRd(self) -> bool: ...

    def disassemble(self, immOverride: str|None=None, extraLJust: int=0) -> str: ...


class RabbitizerConfig:
    regNames_namedRegisters: bool = True
    regNames_gprAbiNames: int
    regNames_fprAbiNames: int
    regNames_userFpcCsr: bool = True
    regNames_vr4300Cop0NamedRegisters: bool = True
    regNames_vr4300RspCop0NamedRegisters: bool = True

    pseudos_enablePseudos: bool = True
    pseudos_pseudoBeqz: bool = True
    pseudos_pseudoBnez: bool = True
    pseudos_pseudoB: bool = True
    pseudos_pseudoMove: bool = True
    pseudos_pseudoNot: bool = True
    pseudos_pseudoNegu: bool = True

    toolchainTweaks_sn64DivFix: bool = False
    toolchainTweaks_treatJAsUnconditionalBranch: bool = False

    misc_opcodeLJust: int = 11
    misc_unknownInstrComment: bool = True

config: RabbitizerConfig


class InstrId:
    cpu_INVALID: int
    cpu_abs_d: int
    cpu_abs_s: int
    cpu_add: int
    cpu_add_d: int
    cpu_add_s: int
    cpu_addi: int
    cpu_addiu: int
    cpu_addu: int
    cpu_and: int
    cpu_andi: int
    cpu_b: int
    cpu_bc0f: int
    cpu_bc0fl: int
    cpu_bc0t: int
    cpu_bc0tl: int
    cpu_bc1f: int
    cpu_bc1fl: int
    cpu_bc1t: int
    cpu_bc1tl: int
    cpu_beq: int
    cpu_beql: int
    cpu_beqz: int
    cpu_bgez: int
    cpu_bgezal: int
    cpu_bgezall: int
    cpu_bgezl: int
    cpu_bgtz: int
    cpu_bgtzl: int
    cpu_blez: int
    cpu_blezl: int
    cpu_bltz: int
    cpu_bltzal: int
    cpu_bltzall: int
    cpu_bltzl: int
    cpu_bne: int
    cpu_bnel: int
    cpu_bnez: int
    cpu_break: int
    cpu_c_eq_d: int
    cpu_c_eq_s: int
    cpu_c_f_d: int
    cpu_c_f_s: int
    cpu_c_le_d: int
    cpu_c_le_s: int
    cpu_c_lt_d: int
    cpu_c_lt_s: int
    cpu_c_nge_d: int
    cpu_c_nge_s: int
    cpu_c_ngl_d: int
    cpu_c_ngl_s: int
    cpu_c_ngle_d: int
    cpu_c_ngle_s: int
    cpu_c_ngt_d: int
    cpu_c_ngt_s: int
    cpu_c_ole_d: int
    cpu_c_ole_s: int
    cpu_c_olt_d: int
    cpu_c_olt_s: int
    cpu_c_seq_d: int
    cpu_c_seq_s: int
    cpu_c_sf_d: int
    cpu_c_sf_s: int
    cpu_c_ueq_d: int
    cpu_c_ueq_s: int
    cpu_c_ule_d: int
    cpu_c_ule_s: int
    cpu_c_ult_d: int
    cpu_c_ult_s: int
    cpu_c_un_d: int
    cpu_c_un_s: int
    cpu_cache: int
    cpu_ceil_l_d: int
    cpu_ceil_l_s: int
    cpu_ceil_w_d: int
    cpu_ceil_w_s: int
    cpu_cfc0: int
    cpu_cfc1: int
    cpu_ctc0: int
    cpu_ctc1: int
    cpu_cvt_d_l: int
    cpu_cvt_d_s: int
    cpu_cvt_d_w: int
    cpu_cvt_l_d: int
    cpu_cvt_l_s: int
    cpu_cvt_s_d: int
    cpu_cvt_s_l: int
    cpu_cvt_s_w: int
    cpu_cvt_w_d: int
    cpu_cvt_w_s: int
    cpu_dadd: int
    cpu_daddi: int
    cpu_daddiu: int
    cpu_daddu: int
    cpu_ddiv: int
    cpu_ddivu: int
    cpu_div: int
    cpu_div_d: int
    cpu_div_s: int
    cpu_divu: int
    cpu_dmfc0: int
    cpu_dmfc1: int
    cpu_dmtc0: int
    cpu_dmtc1: int
    cpu_dmult: int
    cpu_dmultu: int
    cpu_dsll: int
    cpu_dsll32: int
    cpu_dsllv: int
    cpu_dsra: int
    cpu_dsra32: int
    cpu_dsrav: int
    cpu_dsrl: int
    cpu_dsrl32: int
    cpu_dsrlv: int
    cpu_dsub: int
    cpu_dsubu: int
    cpu_eret: int
    cpu_floor_l_d: int
    cpu_floor_l_s: int
    cpu_floor_w_d: int
    cpu_floor_w_s: int
    cpu_j: int
    cpu_jal: int
    cpu_jalr: int
    cpu_jalr_rd: int
    cpu_jr: int
    cpu_lb: int
    cpu_lbu: int
    cpu_ld: int
    cpu_ldc1: int
    cpu_ldc2: int
    cpu_ldl: int
    cpu_ldr: int
    cpu_lh: int
    cpu_lhu: int
    cpu_ll: int
    cpu_lld: int
    cpu_lui: int
    cpu_lw: int
    cpu_lwc1: int
    cpu_lwc2: int
    cpu_lwl: int
    cpu_lwr: int
    cpu_lwu: int
    cpu_mfc0: int
    cpu_mfc1: int
    cpu_mfhi: int
    cpu_mflo: int
    cpu_mov_d: int
    cpu_mov_s: int
    cpu_move: int
    cpu_movn: int
    cpu_movz: int
    cpu_mtc0: int
    cpu_mtc1: int
    cpu_mthi: int
    cpu_mtlo: int
    cpu_mul_d: int
    cpu_mul_s: int
    cpu_mult: int
    cpu_multu: int
    cpu_neg_d: int
    cpu_neg_s: int
    cpu_negu: int
    cpu_nop: int
    cpu_nor: int
    cpu_not: int
    cpu_or: int
    cpu_ori: int
    cpu_pref: int
    cpu_round_l_d: int
    cpu_round_l_s: int
    cpu_round_w_d: int
    cpu_round_w_s: int
    cpu_sb: int
    cpu_sc: int
    cpu_scd: int
    cpu_sd: int
    cpu_sdc1: int
    cpu_sdc2: int
    cpu_sdl: int
    cpu_sdr: int
    cpu_sh: int
    cpu_sll: int
    cpu_sllv: int
    cpu_slt: int
    cpu_slti: int
    cpu_sltiu: int
    cpu_sltu: int
    cpu_sn64_div: int
    cpu_sn64_divu: int
    cpu_sqrt_d: int
    cpu_sqrt_s: int
    cpu_sra: int
    cpu_srav: int
    cpu_srl: int
    cpu_srlv: int
    cpu_sub: int
    cpu_sub_d: int
    cpu_sub_s: int
    cpu_subu: int
    cpu_sw: int
    cpu_swc1: int
    cpu_swc2: int
    cpu_swl: int
    cpu_swr: int
    cpu_sync: int
    cpu_syscall: int
    cpu_teq: int
    cpu_teqi: int
    cpu_tge: int
    cpu_tgei: int
    cpu_tgeiu: int
    cpu_tgeu: int
    cpu_tlbp: int
    cpu_tlbr: int
    cpu_tlbwi: int
    cpu_tlbwr: int
    cpu_tlt: int
    cpu_tlti: int
    cpu_tltiu: int
    cpu_tltu: int
    cpu_tne: int
    cpu_tnei: int
    cpu_trunc_l_d: int
    cpu_trunc_l_s: int
    cpu_trunc_w_d: int
    cpu_trunc_w_s: int
    cpu_xor: int
    cpu_xori: int

instr_id: InstrId
