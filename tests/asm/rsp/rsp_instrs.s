/* Use https://github.com/Thar0/binutils-rsp/ to build this with `--march=rsp` */

.section .text
.set noreorder
.set noat

.global rsp_instrs
.type rsp_instrs, @function
.ent rsp_instrs
rsp_instrs:

lbv      $v0[0], 0x00($0)
lsv      $v1[1], 0x10($1)
llv      $v2[2], 0x20($2)
ldv      $v3[3], 0x30($3)
lqv      $v4[4], 0x40($4)
lrv      $v5[5], 0x50($5)
lpv      $v6[6], 0x60($6)
luv      $v7[7], 0x70($7)
lhv      $v8[8], 0x80($8)
lfv      $v9[9], 0x90($9)
lwv      $v10[10], 0x100($10)
ltv      $v11[11], 0x110($11)
sbv      $v12[12], 0x00($12)
ssv      $v13[13], 0x10($13)
slv      $v14[14], 0x20($14)
sdv      $v15[15], 0x30($15)
sqv      $v16[0], 0x40($16)
srv      $v17[1], 0x50($17)
spv      $v18[2], 0x60($18)
suv      $v19[3], 0x70($19)
shv      $v20[4], 0x80($20)
sfv      $v21[5], 0x90($21)
swv      $v22[6], 0x100($22)
stv      $v23[7], 0x110($23)
vmulf    $v0, $v1, $v2
vrndp    $v3, $v4, $v5
vmudl    $v6, $v7, $v8
vmudm    $v9, $v10, $v11
vmudn    $v12, $v13, $v14
vmudh    $v15, $v16, $v17
vmacf    $v18, $v19, $v20
vmacu    $v21, $v22, $v23
vrndn    $v24, $v25, $v26
vmacq    $v27, $v28, $v29
vmadl    $v30, $v31, $v0
vmadm    $v0, $v1, $v2
vmadn    $v3, $v4, $v5
vmadh    $v6, $v7, $v8
vsut     $v9, $v10, $v11
vaddc    $v12, $v13, $v14
vsubc    $v15, $v16, $v17
vaddb    $v18, $v19, $v20
vsubb    $v21, $v22, $v23
vaccb    $v24, $v25, $v26
vsucb    $v27, $v28, $v29
vsad     $v30, $v31, $v0
vsac     $v0, $v1, $v2
vsum     $v3, $v4, $v5
vsar     $v6, $v7, $v8
vacc     $v9, $v10, $v11
vsuc     $v12, $v13, $v14
vlt      $v15, $v16, $v17
veq      $v18, $v19, $v20
vne      $v21, $v22, $v23
vge      $v24, $v25, $v26
vcl      $v27, $v28, $v29
vch      $v30, $v31, $v0
vcr      $v0, $v1, $v2
vmrg     $v3, $v4, $v5
vand     $v6, $v7, $v8
vnand    $v9, $v10, $v11
vor      $v12, $v13, $v14
vnor     $v15, $v16, $v17
vxor     $v18, $v19, $v20
vnxor    $v21, $v22, $v23
v056     $v24, $v25, $v26
v057     $v27, $v28, $v29
vrcp     $v30, $v31
vrcpl    $v0, $v1
vrcph    $v2, $v3
vmov     $v4, $v5
vrsq     $v6, $v7
vrsql    $v8, $v9
vrsqh    $v10, $v11
vextt    $v12, $v13, $v14
vextq    $v15, $v16, $v17
vextn    $v18, $v19, $v20
v073     $v21, $v22, $v23
vinst    $v24, $v25, $v26
vinsq    $v27, $v28, $v29
vinsn    $v30, $v31, $v0
vnull
vmulu    $v0, $v1, $v2
vmulq    $v3, $v4, $v5
vadd     $v6, $v7, $v8
vsub     $v9, $v10, $v11
vabs     $v12, $v13, $v14
vnop

.end rsp_instrs
.size rsp_instrs,  . - rsp_instrs

.global bracket_weirdness
.type bracket_weirdness, @function
.ent bracket_weirdness
bracket_weirdness:

vcl         $v3, $v28, $v20[3h]
vmacf       $v3, $v10, $v5[2h]
vch         $v3, $v29, $v21[3h]
vaddc       $v27, $v6, $v11[2h]

vand        $v2, $v11, $v12[0]
mtc2        $14, $v27[0]
vmudn       $v7, $v4, $v27[0]
vmadn       $v2, $v9, $v31[3]
vmudm       $v8, $v7, $v10[1]
vmudh       $v27, $v3, $v29[1]
vmadh       $v8, $v31, $v29[3]
vsar        $v10, $v10, $v10[0]
vsub        $v13, $v8, $v10[5]
vmadm       $v27, $v17, $v31[0]
vmudl       $v14, $v14, $v31[1]
vmulf       $v5, $v5, $v31[4]
vadd        $v14, $v8, $v10[5]
mfc2        $19, $v27[2]
vge         $v27, $v13, $v29[3]
vlt         $v27, $v15, $v29[3]
vmadl       $v8, $v13, $v31[0]
vmov        $v11[1], $v2[1]
vrcp        $v15[0], $v3[0]
vrcph       $v8[0], $v29[3]
vcr         $v0, $v0, $v30[6]
vrcpl       $v7[0], $v13[0]
veq         $v8, $v8, $v31[0]
vne         $v15, $v15, $v31[0]
vnxor       $v8, $v15, $v31[0]
vrsqh       $v11[0], $v31[0]
vrsql       $v15[0], $v7[2]
vnor        $v7, $v0, $v30[1]

.end bracket_weirdness
.size bracket_weirdness,  . - bracket_weirdness

.set reorder
.set at

.global rsp_macro_instrs
.type rsp_macro_instrs, @function
.ent rsp_macro_instrs
rsp_macro_instrs:

lbv         $v0, sym
lbv         $v0, sym($a1)

.end rsp_macro_instrs
.size rsp_macro_instrs,  . - rsp_macro_instrs
