.macro RTPS
    .word 0x4A180001
.endm

.macro RTPT
    .word 0x4A280030
.endm

.macro DPCL
    .word 0x4A680029
.endm

.macro DPCS
    .word 0x4A780010
.endm

.macro DPCT
    .word 0x4AF8002A
.endm

.macro INTPL
    .word 0x4A980011
.endm

.macro NCS
    .word 0x4AC8041E
.endm

.macro NCT
    .word 0x4AD80420
.endm

.macro NCDS
    .word 0x4AE80413
.endm

.macro NCDT
    .word 0x4AF80416
.endm

.macro NCCS
    .word 0x4B08041B
.endm

.macro NCCT
    .word 0x4B18043F
.endm

.macro CDP
    .word 0x4B280414
.endm

.macro CC
    .word 0x4B38041C
.endm

.macro NCLIP
    .word 0x4B400006
.endm

.macro AVSZ3
    .word 0x4B58002D
.endm

.macro AVSZ4
    .word 0x4B68002E
.endm

.macro MVMVA sf, mx, v, cv, lm
    .word 0x4A400012
.endm

.macro SQR sf
    .word 0x4AA00428
.endm

.macro OP sf
    .word 0x4B70000C
.endm

.macro GPF sf
    .word 0x4B90003D
.endm

.macro GPL sf
    .word 0x4BA0003E
.endm



.macro rtv0
    .word 0x4A486012
.endm

.macro rtv1
    .word 0x4A48E012
.endm

.macro rtv2
    .word 0x4A496012
.endm

.macro rtir12
    .word 0x4A49E012
.endm

.macro rtir0
    .word 0x4A41E012
.endm

.macro rtv0tr
    .word 0x4A480012
.endm

.macro rtv1tr
    .word 0x4A488012
.endm

.macro rtv2tr
    .word 0x4A490012
.endm

.macro rtirtr
    .word 0x4A498012
.endm

.macro rtv0bk
    .word 0x4A482012
.endm

.macro rtv1bk
    .word 0x4A48A012
.endm

.macro rtv2bk
    .word 0x4A492012
.endm

.macro rtirbk
    .word 0x4A49A012
.endm

.macro ll
    .word 0x4A4A6412
.endm

.macro llv0
    .word 0x4A4A6012
.endm

.macro llv1
    .word 0x4A4AE012
.endm

.macro llv2
    .word 0x4A4B6012
.endm

.macro llvir
    .word 0x4A4BE012
.endm

.macro llv0tr
    .word 0x4A4A0012
.endm

.macro llv1tr
    .word 0x4A4A8012
.endm

.macro llv2tr
    .word 0x4A4B0012
.endm

.macro llirtr
    .word 0x4A4B8012
.endm

.macro llv0bk
    .word 0x4A4A2012
.endm

.macro llv1bk
    .word 0x4A4AA012
.endm

.macro llv2bk
    .word 0x4A4B2012
.endm

.macro llirbk
    .word 0x4A4BA012
.endm

.macro lc
    .word 0x4A4DA412
.endm

.macro lcv0
    .word 0x4A4C6012
.endm

.macro lcv1
    .word 0x4A4CE012
.endm

.macro lcv2
    .word 0x4A4D6012
.endm

.macro lcvir
    .word 0x4A4DE012
.endm

.macro lcv0tr
    .word 0x4A4C0012
.endm

.macro lcv1tr
    .word 0x4A4C8012
.endm

.macro lcv2tr
    .word 0x4A4D0012
.endm

.macro lcirtr
    .word 0x4A4D8012
.endm

.macro lev0bk
    .word 0x4A4C2012
.endm

.macro lev1bk
    .word 0x4A4CA012
.endm

.macro lev2bk
    .word 0x4A4D2012
.endm

.macro leirbk
    .word 0x4A4DA012
.endm

.macro sqr12
    .word 0x4AA80428
.endm

.macro sqr0
    .word 0x4AA80428
.endm

.macro op12
    .word 0x4B78000C
.endm

.macro op0
    .word 0x4B70000C
.endm

.macro gpf12
    .word 0x4B98003D
.endm

.macro gpf0
    .word 0x4B90003D
.endm

.macro gpl12
    .word 0x4BA8003E
.endm

.macro gpl0
    .word 0x4BA0003E
.endm

/*  DPCS        macro */
/*          dw    $00000e3f */
/*          endm */
/*   */
/*  DPCT        macro */
/*          dw    $00000e7f */
/*          endm */
/*   */
/*  INTPL        macro */
/*          dw    $00000ebf */
/*          endm */
/*   */
/*  NCS        macro */
/*          dw    $00000f7f */
/*          endm */
/*   */
/*  NCT        macro */
/*          dw    $00000fbf */
/*          endm */
/*   */
/*  NCDS        macro */
/*          dw    $00000fff */
/*          endm */
/*   */
/*  NCDT        macro */
/*          dw    $0000103f */
/*          endm */
/*   */
/*  NCCS        macro */
/*          dw    $0000107f */
/*          endm */
/*   */
/*  NCCT        macro */
/*          dw    $000010bf */
/*          endm */
/*   */
/*  CDP        macro */
/*          dw    $000010ff */
/*          endm */
/*   */
/*  CC        macro */
/*          dw    $0000113f */
/*          endm */
/*   */
/*  NCLIP        macro */
/*          dw    $0000117f */
/*          endm */
/*   */
/*  AVSZ3        macro */
/*          dw    $000011bf */
/*          endm */
/*   */
/*  AVSZ4        macro */
/*          dw    $000011ff */
/*          endm */
/*   */
/*  MVMVA        macro    sf,mx,v,cv,lm */
/*          dw    $000013bf|sf<<25|mx<<23|v<<21|cv<<19|lm<<18 */
/*          endm */
/*   */
/*  SQR        macro    sf */
/*          dw    $000013ff|sf<<25 */
/*          endm */
/*   */
/*  OP        macro    sf */
/*          dw    $0000143f|sf<<25 */
/*          endm */
/*   */
/*  GPF        macro    sf */
/*          dw    $0000147f|sf<<25 */
/*          endm */
/*   */
/*  GPL        macro    sf */
/*          dw    $000014bf|sf<<25 */
/*          endm */


/*  RTPS    15      0x4A180001  Perspective transform */
/*  RTPT    23      0x4A280030  Perspective transform on 3 points */
/*  MVMVA   8       0x4A400012  Multiply vector by matrix and vector addition. */
/*  DPCL    8       0x4A680029  Depth Cue Color light */
/*  DPCS    8       0x4A780010  Depth Cueing */
/*  DPCT    17      0x4AF8002A  Depth cue color RGB0,RGB1,RGB2 */
/*  INTPL   8       0x4A980011  Interpolation of vector and far color */
/*  SQR     5       0x4AA00428  Square of vector */
/*  NCS     14      0x4AC8041E  Normal color v0 */
/*  NCT     30      0x4AD80420  Normal color v0, v1, v2 */
/*  NCDS    19      0x4AE80413  Normal color depth cuev0 */
/*  NCDT    44      0x4AF80416  Normal color depth cue v0, v1, v2 */
/*  NCCS    17      0x4B08041B  Normal color col. v0 */
/*  NCCT    39      0x4B18043F  Normal color col.v0, v1, v2 */
/*  CDP     13      0x4B280414  Color Depth Queue */
/*  CC      11      0x4B38041C  Color Col. */
/*  NCLIP   8       0x4B400006  Normal clipping */
/*  AVSZ3   5       0x4B58002D  Average of three Z values */
/*  AVSZ4   6       0x4B68002E  Average of four Z values */
/*  OP      6       0x4B70000C  Outer Product */
/*  GPF     6       0x4B90003D  General purpose interpolation */
/*  GPL     5       0x4BA0003E  general purpose interpolation */
/*  rtv0    -       0x4A486012  v0 * rotmatrix */
/*  rtv1    -       0x4A48E012  v1 * rotmatrix */
/*  rtv2    -       0x4A496012  v2 * rotmatrix */
/*  rtir12  -       0x4A49E012  ir * rotmatrix */
/*  rtir0   -       0x4A41E012  ir * rotmatrix */
/*  rtv0tr  -       0x4A480012  v0 * rotmatrix + tr vector */
/*  rtv1tr  -       0x4A488012  v1 * rotmatrix + tr vector */
/*  rtv2tr  -       0x4A490012  v2 * rotmatrix + tr vector */
/*  rtirtr  -       0x4A498012  ir * rotmatrix + tr vector */
/*  rtv0bk  -       0x4A482012  v0 * rotmatrix + bk vector */
/*  rtv1bk  -       0x4A48A012  v1 * rotmatrix + bk vector */
/*  rtv2bk  -       0x4A492012  v2 * rotmatrix + bk vector */
/*  rtirbk  -       0x4A49A012  ir * rotmatrix + bk vector */
/*  ll      -       0x4A4A6412  v0 * light matrix. Lower limit result to 0 */
/*  llv0    -       0x4A4A6012  v0 * light matrix */
/*  llv1    -       0x4A4AE012  v1 * light matrix */
/*  llv2    -       0x4A4B6012  v2 * light matrix */
/*  llvir   -       0x4A4BE012  ir * light matrix */
/*  llv0tr  -       0x4A4A0012  v0 * light matrix + tr vector */
/*  llv1tr  -       0x4A4A8012  v1 * light matrix + tr vector */
/*  llv2tr  -       0x4A4B0012  v2 * light matrix + tr vector */
/*  llirtr  -       0x4A4B8012  ir * light matrix + tr vector */
/*  llv0bk  -       0x4A4A2012  v0 * light matrix + bk vector */
/*  llv1bk  -       0x4A4AA012  v1 * light matrix + bk vector */
/*  llv2bk  -       0x4A4B2012  v2 * light matrix + bk vector */
/*  llirbk  -       0x4A4BA012  ir * light matrix + bk vector */
/*  lc      -       0x4A4DA412  v0 * color matrix, Lower limit clamped to 0 */
/*  lcv0    -       0x4A4C6012  v0 * color matrix */
/*  lcv1    -       0x4A4CE012  v1 * color matrix */
/*  lcv2    -       0x4A4D6012  v2 * color matrix */
/*  lcvir   -       0x4A4DE012  ir * color matrix */
/*  lcv0tr  -       0x4A4C0012  v0 * color matrix + tr vector */
/*  lcv1tr  -       0x4A4C8012  v1 * color matrix + tr vector */
/*  lcv2tr  -       0x4A4D0012  v2 * color matrix + tr vector */
/*  lcirtr  -       0x4A4D8012  ir * color matrix + tr vector */
/*  lev0bk  -       0x4A4C2012  v0 * color matrix + bk vector */
/*  lev1bk  -       0x4A4CA012  v1 * color matrix + bk vector */
/*  lev2bk  -       0x4A4D2012  v2 * color matrix + bk vector */
/*  leirbk  -       0x4A4DA012  ir * color matrix + bk vector */
/*  sqr12   -       0x4AA80428  square of ir    1,19,12 */
/*  sqr0    -       0x4AA80428  square of ir    1,31, 0 */
/*  op12    -       0x4B78000C  outer product   1,19,12 */
/*  op0     -       0x4B70000C  outer product   1,31, 0 */
/*  gpf12   -       0x4B98003D  general purpose interpolation   1,19,12 */
/*  gpf0    -       0x4B90003D  general purpose interpolation   1,31, 0 */
/*  gpl12   -       0x4BA8003E  general purpose interpolation   1,19,12 */
/*  gpl0    -       0x4BA0003E  general purpose interpolation   1,31, 0 */
