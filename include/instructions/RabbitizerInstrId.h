/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRID_H
#define RABBITIZER_INSTRID_H
#pragma once

typedef enum RabbitizerInstrCpuId {
    RABBITIZER_INSTR_CPU_ID_INVALID,
#if 0

    SLL       = enum.auto() # Shift word Left Logical

    SRL       = enum.auto() # Shift word Right Logical
    SRA       = enum.auto() # Shift word Right Arithmetic
    SLLV      = enum.auto() # Shift word Left Logical Variable

    SRLV      = enum.auto() # Shift word Right Logical Variable
    SRAV      = enum.auto() # Shift word Right Arithmetic Variable

    JR        = enum.auto() # Jump Register
    JALR      = enum.auto() # Jump And Link Register
    JALR_RD   = enum.auto() # Jump And Link Register # Special case for rd != 31
    MOVZ      = enum.auto() # MOVe conditional on Zero
    MOVN      = enum.auto() # MOVe conditional on Not zero
    SYSCALL   = enum.auto() # SYStem CALL
    BREAK     = enum.auto() # Break

    SYNC      = enum.auto() # Sync

    MFHI      = enum.auto() # Move From HI register
    MTHI      = enum.auto() # Move To HI register
    MFLO      = enum.auto() # Move From LO register
    MTLO      = enum.auto() # Move To LO register
    DSLLV     = enum.auto() # Doubleword Shift Left Logical Variable

    DSRLV     = enum.auto() # Doubleword Shift Right Logical Variable
    DSRAV     = enum.auto() # Doubleword Shift Right Arithmetic Variable

    MULT      = enum.auto() # MULTtiply word
    MULTU     = enum.auto() # MULTtiply Unsigned word
    DIV       = enum.auto() # DIVide word
    DIVU      = enum.auto() # DIVide Unsigned word
    DMULT     = enum.auto() # Doubleword MULTiply
    DMULTU    = enum.auto() # Doubleword MULTiply Unsigned
    DDIV      = enum.auto() # Doubleword DIVide
    DDIVU     = enum.auto() # Doubleword DIVide Unsigned

    SN64_DIV  = enum.auto() # DIVide word
    SN64_DIVU = enum.auto() # DIVide Unsigned word

    ADD       = enum.auto() # ADD word
    ADDU      = enum.auto() # ADD Unsigned word
    SUB       = enum.auto() # Subtract word
    SUBU      = enum.auto() # SUBtract Unsigned word
    AND       = enum.auto() # AND
    OR        = enum.auto() # OR
    XOR       = enum.auto() # eXclusive OR
    NOR       = enum.auto() # Not OR

    SLT       = enum.auto() # Set on Less Than
    SLTU      = enum.auto() # Set on Less Than Unsigned
    DADD      = enum.auto() # Doubleword Add
    DADDU     = enum.auto() # Doubleword Add Unsigned
    DSUB      = enum.auto() # Doubleword SUBtract
    DSUBU     = enum.auto() # Doubleword SUBtract Unsigned

    TGE       = enum.auto() # Trap if Greater or Equal
    TGEU      = enum.auto() # Trap if Greater or Equal Unsigned
    TLT       = enum.auto() # Trap if Less Than
    TLTU      = enum.auto() # Trap if Less Than Unsigned
    TEQ       = enum.auto() # Trap if EQual

    TNE       = enum.auto() # Trap if Not Equal

    DSLL      = enum.auto() # Doubleword Shift Left Logical

    DSRL      = enum.auto() # Doubleword Shift Right Logical
    DSRA      = enum.auto() # Doubleword Shift Right Arithmetic
    DSLL32    = enum.auto() # Doubleword Shift Left Logical plus 32

    DSRL32    = enum.auto() # Doubleword Shift Right Logical plus 32
    DSRA32    = enum.auto() # Doubleword Shift Right Arithmetic plus 32

    BLTZ      = enum.auto() # Branch on Less Than Zero
    BGEZ      = enum.auto() # Branch on Greater than or Equal to Zero
    BLTZL     = enum.auto() # Branch on Less Than Zero Likely
    BGEZL     = enum.auto() # Branch on Greater than or Equal to Zero Likely

    TGEI      = enum.auto()
    TGEIU     = enum.auto()
    TLTI      = enum.auto()
    TLTIU     = enum.auto()

    BLTZAL    = enum.auto()
    BGEZAL    = enum.auto()
    BLTZALL   = enum.auto()
    BGEZALL   = enum.auto()

    TEQI      = enum.auto()
    TNEI      = enum.auto()

    J         = enum.auto() # Jump
    JAL       = enum.auto() # Jump And Link
    BEQ       = enum.auto() # Branch on EQual
    BNE       = enum.auto() # Branch on Not Equal
    BLEZ      = enum.auto() # Branch on Less than or Equal to Zero
    BGTZ      = enum.auto() # Branch on Greater Than Zero

    ADDI      = enum.auto() # Add Immediate
    ADDIU     = enum.auto() # Add Immediate Unsigned Word
    SLTI      = enum.auto() # Set on Less Than Immediate
    SLTIU     = enum.auto() # Set on Less Than Immediate Unsigned
    ANDI      = enum.auto() # And Immediate
    ORI       = enum.auto() # Or Immediate
    XORI      = enum.auto() # eXclusive OR Immediate
    LUI       = enum.auto() # Load Upper Immediate

    MFC0      = enum.auto() # Move word From CP0
    DMFC0     = enum.auto() # Doubleword Move From CP0
    CFC0      = enum.auto() # Move control word From CP0

    MTC0      = enum.auto() # Move word to CP0
    DMTC0     = enum.auto() # Doubleword Move To CP0
    CTC0      = enum.auto() # Move control word To CP0

    TLBR      = enum.auto() # Read Indexed TLB Entry
    TLBWI     = enum.auto() # Write Indexed TLB Entry
    TLBWR     = enum.auto() # Write Random TLB Entry
    TLBP      = enum.auto() # Probe TLB for Matching Entry
    ERET      = enum.auto() # Return from Exception

    BC0T      = enum.auto() # Branch on FP True
    BC0F      = enum.auto() # Branch on FP False
    BC0TL     = enum.auto() # Branch on FP True Likely
    BC0FL     = enum.auto() # Branch on FP False Likely

    MFC1      = enum.auto() # Move Word From Floating-Point
    DMFC1     = enum.auto() # Doubleword Move From Floating-Point
    CFC1      = enum.auto() # Move Control Word from Floating-Point

    MTC1      = enum.auto() # Move Word to Floating-Point
    DMTC1     = enum.auto() # Doubleword Move To Floating-Point
    CTC1      = enum.auto() # Move Control Word to Floating-Point

    BC1F      = enum.auto()
    BC1T      = enum.auto()
    BC1FL     = enum.auto()
    BC1TL     = enum.auto()
    ADD_S     = enum.auto() # Floating-Point Add
    SUB_S     = enum.auto() # Floating-Point Sub
    MUL_S     = enum.auto() # Floating-Point Multiply
    DIV_S     = enum.auto() # Floating-Point Divide
    SQRT_S    = enum.auto() # Floating-Point Square Root
    ABS_S     = enum.auto() # Floating-Point Absolute Value
    MOV_S     = enum.auto() # Floating-Point Move
    NEG_S     = enum.auto() # Floating-Point Negate
    ROUND_L_S = enum.auto() # Floating-Point Round to Long Fixed-Point
    TRUNC_L_S = enum.auto() # Floating-Point Truncate to Long Fixed-Point
    CEIL_L_S  = enum.auto() # Floating-Point Ceiling Convert to Long Fixed-Point
    FLOOR_L_S = enum.auto() # Floating-Point Floor Convert to Long Fixed-Point
    ROUND_W_S = enum.auto() # Floating-Point Round to Word Fixed-Point
    TRUNC_W_S = enum.auto() # Floating-Point Truncate to Word Fixed-Point
    CEIL_W_S  = enum.auto() # Floating-Point Ceiling Convert to Word Fixed-Point
    FLOOR_W_S = enum.auto() # Floating-Point Floor Convert to Word Fixed-Point
    CVT_D_S   = enum.auto()
    CVT_W_S   = enum.auto()
    CVT_L_S   = enum.auto()
    C_F_S     = enum.auto()
    C_UN_S    = enum.auto()
    C_EQ_S    = enum.auto()
    C_UEQ_S   = enum.auto()
    C_OLT_S   = enum.auto()
    C_ULT_S   = enum.auto()
    C_OLE_S   = enum.auto()
    C_ULE_S   = enum.auto()
    C_SF_S    = enum.auto()
    C_NGLE_S  = enum.auto()
    C_SEQ_S   = enum.auto()
    C_NGL_S   = enum.auto()
    C_LT_S    = enum.auto()
    C_NGE_S   = enum.auto()
    C_LE_S    = enum.auto()
    C_NGT_S   = enum.auto()
    ADD_D     = enum.auto() # Floating-Point Add
    SUB_D     = enum.auto() # Floating-Point Sub
    MUL_D     = enum.auto() # Floating-Point Multiply
    DIV_D     = enum.auto() # Floating-Point Divide
    SQRT_D    = enum.auto() # Floating-Point Square Root
    ABS_D     = enum.auto() # Floating-Point Absolute Value
    MOV_D     = enum.auto() # Floating-Point Move
    NEG_D     = enum.auto() # Floating-Point Negate
    ROUND_L_D = enum.auto() # Floating-Point Round to Long Fixed-Point
    TRUNC_L_D = enum.auto() # Floating-Point Truncate to Long Fixed-Point
    CEIL_L_D  = enum.auto() # Floating-Point Ceiling Convert to Long Fixed-Point
    FLOOR_L_D = enum.auto() # Floating-Point Floor Convert to Long Fixed-Point
    ROUND_W_D = enum.auto() # Floating-Point Round to Word Fixed-Point
    TRUNC_W_D = enum.auto() # Floating-Point Truncate to Word Fixed-Point
    CEIL_W_D  = enum.auto() # Floating-Point Ceiling Convert to Word Fixed-Point
    FLOOR_W_D = enum.auto() # Floating-Point Floor Convert to Word Fixed-Point
    CVT_S_D   = enum.auto()
    CVT_W_D   = enum.auto()
    CVT_L_D   = enum.auto()
    C_F_D     = enum.auto()
    C_UN_D    = enum.auto()
    C_EQ_D    = enum.auto()
    C_UEQ_D   = enum.auto()
    C_OLT_D   = enum.auto()
    C_ULT_D   = enum.auto()
    C_OLE_D   = enum.auto()
    C_ULE_D   = enum.auto()
    C_SF_D    = enum.auto()
    C_NGLE_D  = enum.auto()
    C_SEQ_D   = enum.auto()
    C_NGL_D   = enum.auto()
    C_LT_D    = enum.auto()
    C_NGE_D   = enum.auto()
    C_LE_D    = enum.auto()
    C_NGT_D   = enum.auto()
    CVT_S_W   = enum.auto()
    CVT_D_W   = enum.auto()
    CVT_S_L   = enum.auto()
    CVT_D_L   = enum.auto()

    BEQL      = enum.auto() # Branch on EQual Likely
    BNEL      = enum.auto() # Branch on Not Equal Likely
    BLEZL     = enum.auto() # Branch on Less than or Equal to Zero Likely
    BGTZL     = enum.auto() # Branch on Greater Than Zero Likely

    DADDI     = enum.auto() # Doubleword add Immediate
    DADDIU    = enum.auto() # Doubleword add Immediate Unsigned
    LDL       = enum.auto() # Load Doubleword Left
    LDR       = enum.auto() # Load Doubleword Right

    LB        = enum.auto() # Load Byte
    LH        = enum.auto() # Load Halfword
    LWL       = enum.auto() # Load Word Left
    LW        = enum.auto() # Load Word
    LBU       = enum.auto() # Load Byte Insigned
    LHU       = enum.auto() # Load Halfword Unsigned
    LWR       = enum.auto() # Load Word Right
    LWU       = enum.auto() # Load Word Unsigned

    SB        = enum.auto() # Store Byte
    SH        = enum.auto() # Store Halfword
    SWL       = enum.auto() # Store Word Left
    SW        = enum.auto() # Store Word
    SDL       = enum.auto() # Store Doubleword Left
    SDR       = enum.auto() # Store Doubleword Right
    SWR       = enum.auto() # Store Word Right
    CACHE     = enum.auto() # Cache

    LL        = enum.auto() # Load Linked word
    LWC1      = enum.auto() # Load Word to Coprocessor z
    LWC2      = enum.auto() # Load Word to Coprocessor z
    PREF      = enum.auto() # Prefetch
    LLD       = enum.auto() # Load Linked Doubleword
    LDC1      = enum.auto() # Load Doubleword to Coprocessor z
    LDC2      = enum.auto() # Load Doubleword to Coprocessor z
    LD        = enum.auto() # Load Doubleword

    SC        = enum.auto() # Store Conditional word
    SWC1      = enum.auto() # Store Word from Coprocessor z
    SWC2      = enum.auto() # Store Word from Coprocessor z
    #
    SCD       = enum.auto() # Store Conditional Doubleword
    SDC1      = enum.auto() # Store Doubleword from Coprocessor z
    SDC2      = enum.auto() # Store Doubleword from Coprocessor z
    SD        = enum.auto() # Store Doubleword

    # Pseudo-Instruction Unique IDs
    BEQZ      = enum.auto() # Branch on EQual Zero
    BNEZ      = enum.auto() # Branch on Not Equal Zero
    B         = enum.auto() # Branch (unconditional)
    NOP       = enum.auto() # No OPeration
    MOVE      = enum.auto() # Move
    NEGU      = enum.auto() 
    NOT       = enum.auto() # Not

#endif
    RABBITIZER_INSTR_ID_MAX,
} RabbitizerInstrCpuId;

typedef enum RabbitizerInstrRspId {
    RABBITIZER_INSTR_VECTOR_ID_INVALID = RABBITIZER_INSTR_ID_MAX+1,

#if 0
    VMULF     = enum.auto()
    VMULU     = enum.auto()
    VRNDP     = enum.auto()
    VMULQ     = enum.auto()
    VMUDL     = enum.auto()
    VMUDM     = enum.auto()
    VMUDN     = enum.auto()
    VMUDH     = enum.auto()
    VMACF     = enum.auto()
    VMACU     = enum.auto()
    VRNDN     = enum.auto()
    VMACQ     = enum.auto()
    VMADL     = enum.auto()
    VMADM     = enum.auto()
    VMADN     = enum.auto()
    VMADH     = enum.auto()
    VADD      = enum.auto()
    VSUB      = enum.auto()
    VABS      = enum.auto()
    VADDC     = enum.auto()
    VSUBC     = enum.auto()
    VSAR      = enum.auto()
    VAND      = enum.auto()
    VNAND     = enum.auto()
    VOR       = enum.auto()
    VNOR      = enum.auto()
    VXOR      = enum.auto()
    VNXOR     = enum.auto()

    VLT       = enum.auto()
    VEQ       = enum.auto()
    VNE       = enum.auto()
    VGE       = enum.auto()
    VCL       = enum.auto()
    VCH       = enum.auto()
    VCR       = enum.auto()
    VMRG      = enum.auto()

    VRCP      = enum.auto()
    VRCPL     = enum.auto()
    VRCPH     = enum.auto()
    VMOV      = enum.auto()
    VRSQ      = enum.auto()
    VRSQL     = enum.auto()
    VRSQH     = enum.auto()
    VNOP      = enum.auto()

    MFC2      = enum.auto()
    MTC2      = enum.auto()
    CFC2      = enum.auto()
    CTC2      = enum.auto()

    SBV       = enum.auto()
    SSV       = enum.auto()
    SLV       = enum.auto()
    SDV       = enum.auto()

    SQV       = enum.auto()
    SRV       = enum.auto()

    SPV       = enum.auto()

    SUV       = enum.auto()
    SWV       = enum.auto()

    SHV       = enum.auto()

    SFV       = enum.auto()
    STV       = enum.auto()

    LBV       = enum.auto()
    LSV       = enum.auto()
    LLV       = enum.auto()
    LDV       = enum.auto()

    LQV       = enum.auto()
    LRV       = enum.auto()

    LPV       = enum.auto()

    LUV       = enum.auto()

    LHV       = enum.auto()

    LFV       = enum.auto()
    LTV       = enum.auto()
#endif
    RABBITIZER_INSTR_VECTOR_ID_MAX,
} RabbitizerInstrRspId;


typedef union RabbitizerInstrId {
    RabbitizerInstrCpuId cpuId;
    RabbitizerInstrRspId rspId;
} RabbitizerInstrId;

#endif
