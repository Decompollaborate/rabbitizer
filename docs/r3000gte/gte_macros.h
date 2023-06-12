/**
 * GTE macros
 * This file is meant to be used as an alternative to DMPSX
 */


/*  RTPS    15      0x4A180001  Perspective transform */
#define gte_nRTPS() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4A180001" \
    )

#define gte_RTPS() __asm__ ( \
        ".word 0x4A180001" \
    )

/*  RTPT    23      0x4A280030  Perspective transform on 3 points */
#define gte_nRTPT() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4A280030" \
    )

#define gte_RTPT() __asm__ ( \
        ".word 0x4A280030" \
    )

/*  DPCL    8       0x4A680029  Depth Cue Color light */
#define gte_nDPCL() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4A680029" \
    )

#define gte_DPCL() __asm__ ( \
        ".word 0x4A680029" \
    )

/*  DPCS    8       0x4A780010  Depth Cueing */
#define gte_nDPCS() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4A780010" \
    )

#define gte_DPCS() __asm__ ( \
        ".word 0x4A780010" \
    )

/*  DPCT    17      0x4AF8002A  Depth cue color RGB0,RGB1,RGB2 */
#define gte_nDPCT() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4AF8002A" \
    )

#define gte_DPCT() __asm__ ( \
        ".word 0x4AF8002A" \
    )

/*  INTPL   8       0x4A980011  Interpolation of vector and far color */
#define gte_nINTPL() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4A980011" \
    )

#define gte_INTPL() __asm__ ( \
        ".word 0x4A980011" \
    )

/*  NCS     14      0x4AC8041E  Normal color v0 */
#define gte_nNCS() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4AC8041E" \
    )

#define gte_NCS() __asm__ ( \
        ".word 0x4AC8041E" \
    )

/*  NCT     30      0x4AD80420  Normal color v0, v1, v2 */
#define gte_nNCT() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4AD80420" \
    )

#define gte_NCT() __asm__ ( \
        ".word 0x4AD80420" \
    )

/*  NCDS    19      0x4AE80413  Normal color depth cuev0 */
#define gte_nNCDS() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4AE80413" \
    )

#define gte_NCDS() __asm__ ( \
        ".word 0x4AE80413" \
    )

/*  NCDT    44      0x4AF80416  Normal color depth cue v0, v1, v2 */
#define gte_nNCDT() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4AF80416" \
    )

#define gte_NCDT() __asm__ ( \
        ".word 0x4AF80416" \
    )

/*  NCCS    17      0x4B08041B  Normal color col. v0 */
#define gte_nNCCS() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B08041B" \
    )

#define gte_NCCS() __asm__ ( \
        ".word 0x4B08041B" \
    )

/*  NCCT    39      0x4B18043F  Normal color col.v0, v1, v2 */
#define gte_nNCCT() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B18043F" \
    )

#define gte_NCCT() __asm__ ( \
        ".word 0x4B18043F" \
    )

/*  CDP     13      0x4B280414  Color Depth Queue */
#define gte_nCDP() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B280414" \
    )

#define gte_CDP() __asm__ ( \
        ".word 0x4B280414" \
    )

/*  CC      11      0x4B38041C  Color Col. */
#define gte_CC() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B38041C" \
    )

#define gte_CC() __asm__ ( \
        ".word 0x4B38041C" \
    )

/*  NCLIP   8       0x4B400006  Normal clipping */
#define gte_nNCLIP() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B400006" \
    )

#define gte_NCLIP() __asm__ ( \
        ".word 0x4B400006" \
    )

/*  AVSZ3   5       0x4B58002D  Average of three Z values */
#define gte_nAVSZ3() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B58002D" \
    )

#define gte_AVSZ3() __asm__ ( \
        ".word 0x4B58002D" \
    )

/*  AVSZ4   6       0x4B68002E  Average of four Z values */
#define gte_nAVSZ4() __asm__ ( \
        "nop;" \
        "nop;" \
        ".word 0x4B68002E" \
    )

#define gte_AVSZ4() __asm__ ( \
        ".word 0x4B68002E" \
    )

/**
 * Instructions which take an argument
 *
 * sf : arg is 1 bit wide
 * mx : arg is 2 bit wide
 * v  : arg is 2 bit wide
 * cv : arg is 2 bit wide
 * lm : arg is 1 bit wide
 */

/*  MVMVA   8       0x4A400012  Multiply vector by matrix and vector addition. */
#define gte_nMVMVA(sf, mx, v, cv, lm) __asm__ ( \
        "nop;" \
        "nop;" \
        ".word %0" \
        : : "g"(0x4A400012 | ((sf) & 0x1) << 19 | ((mx) & 0x3) << 17 | ((v) & 0x3) << 15 | ((cv) & 0x3) << 13 | ((lm) & 0x1) << 10) \
    )

#define gte_MVMVA(sf, mx, v, cv, lm) __asm__ ( \
        ".word %0" \
        : : "g"(0x4A400012 | ((sf) & 0x1) << 19 | ((mx) & 0x3) << 17 | ((v) & 0x3) << 15 | ((cv) & 0x3) << 13 | ((lm) & 0x1) << 10) \
    )

/*  SQR     5       0x4AA00428  Square of vector */
#define gte_nSQR(sf) __asm__ ( \
        "nop;" \
        "nop;" \
        ".word %0" \
        : : "g"(0x4AA00428 | ((sf) & 0x1) << 19) \
    )

#define gte_SQR(sf) __asm__ ( \
        ".word %0" \
        : : "g"(0x4AA00428 | ((sf) & 0x1) << 19) \
    )

/*  OP      6       0x4B70000C  Outer Product */
#define gte_nOP(sf) __asm__ ( \
        "nop;" \
        "nop;" \
        ".word %0" \
        : : "g"(0x4B70000C | ((sf) & 0x1) << 19) \
    )

#define gte_OP(sf) __asm__ ( \
        ".word %0" \
        : : "g"(0x4B70000C | ((sf) & 0x1) << 19) \
    )

/*  GPF     6       0x4B90003D  General purpose interpolation */
#define gte_nGPF(sf) __asm__ ( \
        "nop;" \
        "nop;" \
        ".word %0" \
        : : "g"(0x4B90003D | ((sf) & 0x1) << 19) \
    )

#define gte_GPF(sf) __asm__ ( \
        ".word %0" \
        : : "g"(0x4B90003D | ((sf) & 0x1) << 19) \
    )

/*  GPL     5       0x4BA0003E  general purpose interpolation */
#define gte_nGPL(sf) __asm__ ( \
        "nop;" \
        "nop;" \
        ".word %0" \
        : : "g"(0x4BA0003E | ((sf) & 0x1) << 19) \
    )

#define gte_GPL(sf) __asm__ ( \
        ".word %0" \
        : : "g"(0x4BA0003E | ((sf) & 0x1) << 19) \
    )


/**
 * Convenience macros
 */

/*  rtv0    -       0x4A486012  v0 * rotmatrix */
#define gte_nrtv0() gte_nMVMVA(1, 0, 0, 3, 0)

#define gte_rtv0() gte_MVMVA(1, 0, 0, 3, 0)

/*  rtv1    -       0x4A48E012  v1 * rotmatrix */
#define gte_nrtv1() gte_nMVMVA(1, 0, 1, 3, 0)

#define gte_rtv1() gte_MVMVA(1, 0, 1, 3, 0)

/*  rtv2    -       0x4A496012  v2 * rotmatrix */
#define gte_nrtv2() gte_nMVMVA(1, 0, 2, 3, 0)

#define gte_rtv2() gte_MVMVA(1, 0, 2, 3, 0)

/*  rtir12  -       0x4A49E012  ir * rotmatrix */
#define gte_nrtir12() gte_nMVMVA(1, 0, 3, 3, 0)

#define gte_rtir12() gte_MVMVA(1, 0, 3, 3, 0)

/*  rtir0   -       0x4A41E012  ir * rotmatrix */
#define gte_nrtir0() gte_nMVMVA(0, 0, 3, 3, 0)

#define gte_rtir0() gte_MVMVA(0, 0, 3, 3, 0)

/*  rtv0tr  -       0x4A480012  v0 * rotmatrix + tr vector */
#define gte_nrtv0tr() gte_nMVMVA(1, 0, 0, 0, 0)

#define gte_rtv0tr() gte_MVMVA(1, 0, 0, 0, 0)

/*  rtv1tr  -       0x4A488012  v1 * rotmatrix + tr vector */
#define gte_nrtv1tr() gte_nMVMVA(1, 0, 1, 0, 0)

#define gte_rtv1tr() gte_MVMVA(1, 0, 1, 0, 0)

/*  rtv2tr  -       0x4A490012  v2 * rotmatrix + tr vector */
#define gte_nrtv2tr() gte_nMVMVA(1, 0, 2, 0, 0)

#define gte_rtv2tr() gte_MVMVA(1, 0, 2, 0, 0)

/*  rtirtr  -       0x4A498012  ir * rotmatrix + tr vector */
#define gte_nrtirtr() gte_nMVMVA(1, 0, 3, 0, 0)

#define gte_rtirtr() gte_MVMVA(1, 0, 3, 0, 0)

/*  rtv0bk  -       0x4A482012  v0 * rotmatrix + bk vector */
#define gte_nrtv0bk() gte_nMVMVA(1, 0, 0, 1, 0)

#define gte_rtv0bk() gte_MVMVA(1, 0, 0, 1, 0)

/*  rtv1bk  -       0x4A48A012  v1 * rotmatrix + bk vector */
#define gte_nrtv1bk() gte_nMVMVA(1, 0, 1, 1, 0)

#define gte_rtv1bk() gte_MVMVA(1, 0, 1, 1, 0)

/*  rtv2bk  -       0x4A492012  v2 * rotmatrix + bk vector */
#define gte_nrtv2bk() gte_nMVMVA(1, 0, 2, 1, 0)

#define gte_rtv2bk() gte_MVMVA(1, 0, 2, 1, 0)

/*  rtirbk  -       0x4A49A012  ir * rotmatrix + bk vector */
#define gte_nrtirbk() gte_nMVMVA(1, 0, 3, 1, 0)

#define gte_rtirbk() gte_MVMVA(1, 0, 3, 1, 0)

/*  ll      -       0x4A4A6412  v0 * light matrix. Lower limit result to 0 */
#define gte_nll() gte_nMVMVA(1, 1, 0, 3, 1)

#define gte_ll() gte_MVMVA(1, 1, 0, 3, 1)

/*  llv0    -       0x4A4A6012  v0 * light matrix */
#define gte_nllv0() gte_nMVMVA(1, 1, 0, 3, 0)

#define gte_llv0() gte_MVMVA(1, 1, 0, 3, 0)

/*  llv1    -       0x4A4AE012  v1 * light matrix */
#define gte_nllv1() gte_nMVMVA(1, 1, 1, 3, 0)

#define gte_llv1() gte_MVMVA(1, 1, 1, 3, 0)

/*  llv2    -       0x4A4B6012  v2 * light matrix */
#define gte_nllv2() gte_nMVMVA(1, 1, 2, 3, 0)

#define gte_llv2() gte_MVMVA(1, 1, 2, 3, 0)

/*  llvir   -       0x4A4BE012  ir * light matrix */
#define gte_nllvir() gte_nMVMVA(1, 1, 3, 3, 0)

#define gte_llvir() gte_MVMVA(1, 1, 3, 3, 0)

/*  llv0tr  -       0x4A4A0012  v0 * light matrix + tr vector */
#define gte_nllv0tr() gte_nMVMVA(1, 1, 0, 0, 0)

#define gte_llv0tr() gte_MVMVA(1, 1, 0, 0, 0)

/*  llv1tr  -       0x4A4A8012  v1 * light matrix + tr vector */
#define gte_nllv1tr() gte_nMVMVA(1, 1, 1, 0, 0)

#define gte_llv1tr() gte_MVMVA(1, 1, 1, 0, 0)

/*  llv2tr  -       0x4A4B0012  v2 * light matrix + tr vector */
#define gte_nllv2tr() gte_nMVMVA(1, 1, 2, 0, 0)

#define gte_llv2tr() gte_MVMVA(1, 1, 2, 0, 0)

/*  llirtr  -       0x4A4B8012  ir * light matrix + tr vector */
#define gte_nllirtr() gte_nMVMVA(1, 1, 3, 0, 0)

#define gte_llirtr() gte_MVMVA(1, 1, 3, 0, 0)

/*  llv0bk  -       0x4A4A2012  v0 * light matrix + bk vector */
#define gte_nllv0bk() gte_nMVMVA(1, 1, 0, 1, 0)

#define gte_llv0bk() gte_MVMVA(1, 1, 0, 1, 0)

/*  llv1bk  -       0x4A4AA012  v1 * light matrix + bk vector */
#define gte_nllv1bk() gte_nMVMVA(1, 1, 1, 1, 0)

#define gte_llv1bk() gte_MVMVA(1, 1, 1, 1, 0)

/*  llv2bk  -       0x4A4B2012  v2 * light matrix + bk vector */
#define gte_nllv2bk() gte_nMVMVA(1, 1, 2, 1, 0)

#define gte_llv2bk() gte_MVMVA(1, 1, 2, 1, 0)

/*  llirbk  -       0x4A4BA012  ir * light matrix + bk vector */
#define gte_nllirbk() gte_nMVMVA(1, 1, 3, 1, 0)

#define gte_llirbk() gte_MVMVA(1, 1, 3, 1, 0)

/*  lc      -       0x4A4DA412  v0 * color matrix, Lower limit clamped to 0 */
#define gte_nlc() gte_nMVMVA(1, 2, 3, 1, 1)

#define gte_lc() gte_MVMVA(1, 2, 3, 1, 1)

/*  lcv0    -       0x4A4C6012  v0 * color matrix */
#define gte_nlcv0() gte_nMVMVA(1, 2, 0, 3, 0)

#define gte_lcv0() gte_MVMVA(1, 2, 0, 3, 0)

/*  lcv1    -       0x4A4CE012  v1 * color matrix */
#define gte_nlcv1() gte_nMVMVA(1, 2, 1, 3, 0)

#define gte_lcv1() gte_MVMVA(1, 2, 1, 3, 0)

/*  lcv2    -       0x4A4D6012  v2 * color matrix */
#define gte_nlcv2() gte_nMVMVA(1, 2, 2, 3, 0)

#define gte_lcv2() gte_MVMVA(1, 2, 2, 3, 0)

/*  lcvir   -       0x4A4DE012  ir * color matrix */
#define gte_nlcvir() gte_nMVMVA(1, 2, 3, 3, 0)

#define gte_lcvir() gte_MVMVA(1, 2, 3, 3, 0)

/*  lcv0tr  -       0x4A4C0012  v0 * color matrix + tr vector */
#define gte_nlcv0tr() gte_nMVMVA(1, 2, 0, 0, 0)

#define gte_lcv0tr() gte_MVMVA(1, 2, 0, 0, 0)

/*  lcv1tr  -       0x4A4C8012  v1 * color matrix + tr vector */
#define gte_nlcv1tr() gte_nMVMVA(1, 2, 1, 0, 0)

#define gte_lcv1tr() gte_MVMVA(1, 2, 1, 0, 0)

/*  lcv2tr  -       0x4A4D0012  v2 * color matrix + tr vector */
#define gte_nlcv2tr() gte_nMVMVA(1, 2, 2, 0, 0)

#define gte_lcv2tr() gte_MVMVA(1, 2, 2, 0, 0)

/*  lcirtr  -       0x4A4D8012  ir * color matrix + tr vector */
#define gte_nlcirtr() gte_nMVMVA(1, 2, 3, 0, 0)

#define gte_lcirtr() gte_MVMVA(1, 2, 3, 0, 0)

/*  lev0bk  -       0x4A4C2012  v0 * color matrix + bk vector */
#define gte_nlev0bk() gte_nMVMVA(1, 2, 0, 1, 0)

#define gte_lev0bk() gte_MVMVA(1, 2, 0, 1, 0)

/*  lev1bk  -       0x4A4CA012  v1 * color matrix + bk vector */
#define gte_nlev1bk() gte_nMVMVA(1, 2, 1, 1, 0)

#define gte_lev1bk() gte_MVMVA(1, 2, 1, 1, 0)

/*  lev2bk  -       0x4A4D2012  v2 * color matrix + bk vector */
#define gte_nlev2bk() gte_nMVMVA(1, 2, 2, 1, 0)

#define gte_lev2bk() gte_MVMVA(1, 2, 2, 1, 0)

/*  leirbk  -       0x4A4DA012  ir * color matrix + bk vector */
#define gte_nleirbk() gte_nMVMVA(1, 2, 3, 1, 0)

#define gte_leirbk() gte_MVMVA(1, 2, 3, 1, 0)

/*  sqr12   -       0x4AA80428  square of ir    1,19,12 */
// #define gte_nsqr12() gte_nSQR(1)
// 
// #define gte_sqr12() gte_SQR(1)

/*  sqr0    -       0x4AA80428  square of ir    1,31, 0 */
// #define gte_nsqr0() gte_nSQR(1)
// 
// #define gte_sqr0() gte_SQR(1)

/*  op12    -       0x4B78000C  outer product   1,19,12 */
#define gte_nop12() gte_nOP(1)

#define gte_op12() gte_OP(1)

/*  op0     -       0x4B70000C  outer product   1,31, 0 */
#define gte_nop0() gte_nOP(0)

#define gte_op0() gte_OP(0)

/*  gpf12   -       0x4B98003D  general purpose interpolation   1,19,12 */
#define gte_ngpf12() gte_nGPF(1)

#define gte_gpf12() gte_GPF(1)

/*  gpf0    -       0x4B90003D  general purpose interpolation   1,31, 0 */
#define gte_ngpf0() gte_nGPF(0)

#define gte_gpf0() gte_GPF(0)

/*  gpl12   -       0x4BA8003E  general purpose interpolation   1,19,12 */
#define gte_ngpl12() gte_nGPL(1)

#define gte_gpl12() gte_GPL(1)

/*  gpl0    -       0x4BA0003E  general purpose interpolation   1,31, 0 */
#define gte_ngpl0() gte_nGPL(0)

#define gte_gpl0() gte_GPL(0)
