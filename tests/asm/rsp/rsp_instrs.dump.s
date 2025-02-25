
tests/asm/rsp/rsp_instrs.o:     file format elf32-bigmips


Disassembly of section .text:

00000000 <rsp_instrs>:
   0:	c8000000 	lbv	$v0[0],0(zero)
   4:	c8210888 	lsv	$v1[1],16(at)
   8:	c8421108 	llv	$v2[2],32(v0)
   c:	c8631986 	ldv	$v3[3],48(v1)
  10:	c8842204 	lqv	$v4[4],64(a0)
  14:	c8a52a85 	lrv	$v5[5],80(a1)
  18:	c8c6330c 	lpv	$v6[6],96(a2)
  1c:	c8e73b8e 	luv	$v7[7],112(a3)
  20:	c9084408 	lhv	$v8[8],128(t0)
  24:	c9294c89 	lfv	$v9[9],144(t1)
  28:	c94a5510 	lwv	$v10[10],256(t2)
  2c:	c96b5d91 	ltv	$v11[11],272(t3)
  30:	e98c0600 	sbv	$v12[12],0(t4)
  34:	e9ad0e88 	ssv	$v13[13],16(t5)
  38:	e9ce1708 	slv	$v14[14],32(t6)
  3c:	e9ef1f86 	sdv	$v15[15],48(t7)
  40:	ea102004 	sqv	$v16[0],64(s0)
  44:	ea312885 	srv	$v17[1],80(s1)
  48:	ea52310c 	spv	$v18[2],96(s2)
  4c:	ea73398e 	suv	$v19[3],112(s3)
  50:	ea944208 	shv	$v20[4],128(s4)
  54:	eab54a89 	sfv	$v21[5],144(s5)
  58:	ead65310 	swv	$v22[6],256(s6)
  5c:	eaf75b91 	stv	$v23[7],272(s7)
  60:	4a020800 	vmulf	$v0,$v1,$v2
  64:	4a0520c2 	vrndp	$v3,$v4,$v5
  68:	4a083984 	vmudl	$v6,$v7,$v8
  6c:	4a0b5245 	vmudm	$v9,$v10,$v11
  70:	4a0e6b06 	vmudn	$v12,$v13,$v14
  74:	4a1183c7 	vmudh	$v15,$v16,$v17
  78:	4a149c88 	vmacf	$v18,$v19,$v20
  7c:	4a17b549 	vmacu	$v21,$v22,$v23
  80:	4a1ace0a 	vrndn	$v24,$v25,$v26
  84:	4a1de6cb 	vmacq	$v27,$v28,$v29
  88:	4a00ff8c 	vmadl	$v30,$v31,$v0
  8c:	4a02080d 	vmadm	$v0,$v1,$v2
  90:	4a0520ce 	vmadn	$v3,$v4,$v5
  94:	4a08398f 	vmadh	$v6,$v7,$v8
  98:	4a0b5252 	vsut	$v9,$v10,$v11
  9c:	4a0e6b14 	vaddc	$v12,$v13,$v14
  a0:	4a1183d5 	vsubc	$v15,$v16,$v17
  a4:	4a149c96 	vaddb	$v18,$v19,$v20
  a8:	4a17b557 	vsubb	$v21,$v22,$v23
  ac:	4a1ace18 	vaccb	$v24,$v25,$v26
  b0:	4a1de6d9 	vsucb	$v27,$v28,$v29
  b4:	4a00ff9a 	vsad	$v30,$v31,$v0
  b8:	4a02081b 	vsac	$v0,$v1,$v2
  bc:	4a0520dc 	vsum	$v3,$v4,$v5
  c0:	4a08399d 	vsar	$v6,$v7,$v8
  c4:	4a0b525e 	vacc	$v9,$v10,$v11
  c8:	4a0e6b1f 	vsuc	$v12,$v13,$v14
  cc:	4a1183e0 	vlt	$v15,$v16,$v17
  d0:	4a149ca1 	veq	$v18,$v19,$v20
  d4:	4a17b562 	vne	$v21,$v22,$v23
  d8:	4a1ace23 	vge	$v24,$v25,$v26
  dc:	4a1de6e4 	vcl	$v27,$v28,$v29
  e0:	4a00ffa5 	vch	$v30,$v31,$v0
  e4:	4a020826 	vcr	$v0,$v1,$v2
  e8:	4a0520e7 	vmrg	$v3,$v4,$v5
  ec:	4a0839a8 	vand	$v6,$v7,$v8
  f0:	4a0b5269 	vnand	$v9,$v10,$v11
  f4:	4a0e6b2a 	vor	$v12,$v13,$v14
  f8:	4a1183eb 	vnor	$v15,$v16,$v17
  fc:	4a149cac 	vxor	$v18,$v19,$v20
 100:	4a17b56d 	vnxor	$v21,$v22,$v23
 104:	4a1ace2e 	v056	$v24,$v25,$v26
 108:	4a1de6ef 	v057	$v27,$v28,$v29
 10c:	4a1f07b0 	vrcp	$v30,$v31
 110:	4a010031 	vrcpl	$v0,$v1
 114:	4a0300b2 	vrcph	$v2,$v3
 118:	4a050133 	vmov	$v4,$v5
 11c:	4a0701b4 	vrsq	$v6,$v7
 120:	4a090235 	vrsql	$v8,$v9
 124:	4a0b02b6 	vrsqh	$v10,$v11
 128:	4a0e6b38 	vextt	$v12,$v13,$v14
 12c:	4a1183f9 	vextq	$v15,$v16,$v17
 130:	4a149cba 	vextn	$v18,$v19,$v20
 134:	4a17b57b 	v073	$v21,$v22,$v23
 138:	4a1ace3c 	vinst	$v24,$v25,$v26
 13c:	4a1de6fd 	vinsq	$v27,$v28,$v29
 140:	4a00ffbe 	vinsn	$v30,$v31,$v0
 144:	4a00003f 	vnull
 148:	4a020801 	vmulu	$v0,$v1,$v2
 14c:	4a0520c3 	vmulq	$v3,$v4,$v5
 150:	4a083990 	vadd	$v6,$v7,$v8
 154:	4a0b5251 	vsub	$v9,$v10,$v11
 158:	4a0e6b13 	vabs	$v12,$v13,$v14
 15c:	4a000037 	vnop

00000160 <bracket_weirdness>:
 160:	4af4e0e4 	vcl	$v3,$v28,$v20[3h]
 164:	4ac550c8 	vmacf	$v3,$v10,$v5[2h]
 168:	4af5e8e5 	vch	$v3,$v29,$v21[3h]
 16c:	4acb36d4 	vaddc	$v27,$v6,$v11[2h]
 170:	4b0c58a8 	vand	$v2,$v11,$v12[0]
 174:	488ed800 	mtc2	t6,$v27[0]
 178:	4b1b21c6 	vmudn	$v7,$v4,$v27[0]
 17c:	4b7f488e 	vmadn	$v2,$v9,$v31[3]
 180:	4b2a3a05 	vmudm	$v8,$v7,$v10[1]
 184:	4b3d1ec7 	vmudh	$v27,$v3,$v29[1]
 188:	4b7dfa0f 	vmadh	$v8,$v31,$v29[3]
 18c:	4b0a529d 	vsar	$v10,$v10,$v10[0]
 190:	4baa4351 	vsub	$v13,$v8,$v10[5]
 194:	4b1f8ecd 	vmadm	$v27,$v17,$v31[0]
 198:	4b3f7384 	vmudl	$v14,$v14,$v31[1]
 19c:	4b9f2940 	vmulf	$v5,$v5,$v31[4]
 1a0:	4baa4390 	vadd	$v14,$v8,$v10[5]
 1a4:	4813d900 	mfc2	s3,$v27[2]
 1a8:	4b7d6ee3 	vge	$v27,$v13,$v29[3]
 1ac:	4b7d7ee0 	vlt	$v27,$v15,$v29[3]
 1b0:	4b1f6a0c 	vmadl	$v8,$v13,$v31[0]
 1b4:	4b224af3 	vmov	$v11[1],$v2[1]
 1b8:	4b0343f0 	vrcp	$v15[0],$v3[0]
 1bc:	4b7d4232 	vrcph	$v8[0],$v29[3]
 1c0:	4bde0026 	vcr	$v0,$v0,$v30[6]
 1c4:	4b0d41f1 	vrcpl	$v7[0],$v13[0]
 1c8:	4b1f4221 	veq	$v8,$v8,$v31[0]
 1cc:	4b1f7be2 	vne	$v15,$v15,$v31[0]
 1d0:	4b1f7a2d 	vnxor	$v8,$v15,$v31[0]
 1d4:	4b1f42f6 	vrsqh	$v11[0],$v31[0]
 1d8:	4b4743f5 	vrsql	$v15[0],$v7[2]
 1dc:	4b3e01eb 	vnor	$v7,$v0,$v30[1]

000001e0 <rsp_macro_instrs>:
 1e0:	c8000000 	lbv	$v0[0],0(zero)
			1e0: R_MIPS_RSP70	sym
 1e4:	c8a00000 	lbv	$v0[0],0(a1)
			1e4: R_MIPS_RSP70	sym
