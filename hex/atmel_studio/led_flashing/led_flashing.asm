
led_flashing.hex:     file format ihex


Disassembly of section .sec1:

00000000 <.sec1>:
   0:	0c 94 5c 00 	jmp	0xb8	;  0xb8
   4:	0c 94 6e 00 	jmp	0xdc	;  0xdc
   8:	0c 94 6e 00 	jmp	0xdc	;  0xdc
   c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  10:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  14:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  18:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  1c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  20:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  24:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  28:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  2c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  30:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  34:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  38:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  3c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  40:	0c 94 98 00 	jmp	0x130	;  0x130
  44:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  48:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  4c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  50:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  54:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  58:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  5c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  60:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  64:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  68:	00 00       	nop
  6a:	00 08       	sbc	r0, r0
  6c:	00 02       	muls	r16, r16
  6e:	01 00       	.word	0x0001	; ????
  70:	00 03       	mulsu	r16, r16
  72:	04 07       	cpc	r16, r20
	...
  7c:	01 02       	muls	r16, r17
  7e:	04 08       	sbc	r0, r4
  80:	10 20       	and	r1, r0
  82:	40 80       	ld	r4, Z
  84:	01 02       	muls	r16, r17
  86:	04 08       	sbc	r0, r4
  88:	10 20       	and	r1, r0
  8a:	01 02       	muls	r16, r17
  8c:	04 08       	sbc	r0, r4
  8e:	10 20       	and	r1, r0
  90:	04 04       	cpc	r0, r4
  92:	04 04       	cpc	r0, r4
  94:	04 04       	cpc	r0, r4
  96:	04 04       	cpc	r0, r4
  98:	02 02       	muls	r16, r18
  9a:	02 02       	muls	r16, r18
  9c:	02 02       	muls	r16, r18
  9e:	03 03       	mulsu	r16, r19
  a0:	03 03       	mulsu	r16, r19
  a2:	03 03       	mulsu	r16, r19
  a4:	00 00       	nop
  a6:	00 00       	nop
  a8:	25 00       	.word	0x0025	; ????
  aa:	28 00       	.word	0x0028	; ????
  ac:	2b 00       	.word	0x002b	; ????
  ae:	00 00       	nop
  b0:	00 00       	nop
  b2:	24 00       	.word	0x0024	; ????
  b4:	27 00       	.word	0x0027	; ????
  b6:	2a 00       	.word	0x002a	; ????
  b8:	11 24       	eor	r1, r1
  ba:	1f be       	out	0x3f, r1	; 63
  bc:	cf ef       	ldi	r28, 0xFF	; 255
  be:	d8 e0       	ldi	r29, 0x08	; 8
  c0:	de bf       	out	0x3e, r29	; 62
  c2:	cd bf       	out	0x3d, r28	; 61
  c4:	21 e0       	ldi	r18, 0x01	; 1
  c6:	a0 e0       	ldi	r26, 0x00	; 0
  c8:	b1 e0       	ldi	r27, 0x01	; 1
  ca:	01 c0       	rjmp	.+2      	;  0xce
  cc:	1d 92       	st	X+, r1
  ce:	a9 30       	cpi	r26, 0x09	; 9
  d0:	b2 07       	cpc	r27, r18
  d2:	e1 f7       	brne	.-8      	;  0xcc
  d4:	0e 94 89 00 	call	0x112	;  0x112
  d8:	0c 94 13 02 	jmp	0x426	;  0x426
  dc:	0c 94 00 00 	jmp	0	;  0x0
  e0:	61 e0       	ldi	r22, 0x01	; 1
  e2:	8d e0       	ldi	r24, 0x0D	; 13
  e4:	0c 94 a7 01 	jmp	0x34e	;  0x34e
  e8:	61 e0       	ldi	r22, 0x01	; 1
  ea:	8d e0       	ldi	r24, 0x0D	; 13
  ec:	0e 94 e3 01 	call	0x3c6	;  0x3c6
  f0:	68 ee       	ldi	r22, 0xE8	; 232
  f2:	73 e0       	ldi	r23, 0x03	; 3
  f4:	80 e0       	ldi	r24, 0x00	; 0
  f6:	90 e0       	ldi	r25, 0x00	; 0
  f8:	0e 94 07 01 	call	0x20e	;  0x20e
  fc:	60 e0       	ldi	r22, 0x00	; 0
  fe:	8d e0       	ldi	r24, 0x0D	; 13
 100:	0e 94 e3 01 	call	0x3c6	;  0x3c6
 104:	68 eb       	ldi	r22, 0xB8	; 184
 106:	7b e0       	ldi	r23, 0x0B	; 11
 108:	80 e0       	ldi	r24, 0x00	; 0
 10a:	90 e0       	ldi	r25, 0x00	; 0
 10c:	0c 94 07 01 	jmp	0x20e	;  0x20e
 110:	08 95       	ret
 112:	0e 94 43 01 	call	0x286	;  0x286
 116:	0e 94 88 00 	call	0x110	;  0x110
 11a:	0e 94 70 00 	call	0xe0	;  0xe0
 11e:	c0 e0       	ldi	r28, 0x00	; 0
 120:	d0 e0       	ldi	r29, 0x00	; 0
 122:	0e 94 74 00 	call	0xe8	;  0xe8
 126:	20 97       	sbiw	r28, 0x00	; 0
 128:	e1 f3       	breq	.-8      	;  0x122
 12a:	0e 94 00 00 	call	0	;  0x0
 12e:	f9 cf       	rjmp	.-14     	;  0x122
 130:	1f 92       	push	r1
 132:	0f 92       	push	r0
 134:	0f b6       	in	r0, 0x3f	; 63
 136:	0f 92       	push	r0
 138:	11 24       	eor	r1, r1
 13a:	2f 93       	push	r18
 13c:	3f 93       	push	r19
 13e:	8f 93       	push	r24
 140:	9f 93       	push	r25
 142:	af 93       	push	r26
 144:	bf 93       	push	r27
 146:	80 91 01 01 	lds	r24, 0x0101	;  0x800101
 14a:	90 91 02 01 	lds	r25, 0x0102	;  0x800102
 14e:	a0 91 03 01 	lds	r26, 0x0103	;  0x800103
 152:	b0 91 04 01 	lds	r27, 0x0104	;  0x800104
 156:	30 91 00 01 	lds	r19, 0x0100	;  0x800100
 15a:	23 e0       	ldi	r18, 0x03	; 3
 15c:	23 0f       	add	r18, r19
 15e:	2d 37       	cpi	r18, 0x7D	; 125
 160:	20 f4       	brcc	.+8      	;  0x16a
 162:	01 96       	adiw	r24, 0x01	; 1
 164:	a1 1d       	adc	r26, r1
 166:	b1 1d       	adc	r27, r1
 168:	05 c0       	rjmp	.+10     	;  0x174
 16a:	26 e8       	ldi	r18, 0x86	; 134
 16c:	23 0f       	add	r18, r19
 16e:	02 96       	adiw	r24, 0x02	; 2
 170:	a1 1d       	adc	r26, r1
 172:	b1 1d       	adc	r27, r1
 174:	20 93 00 01 	sts	0x0100, r18	;  0x800100
 178:	80 93 01 01 	sts	0x0101, r24	;  0x800101
 17c:	90 93 02 01 	sts	0x0102, r25	;  0x800102
 180:	a0 93 03 01 	sts	0x0103, r26	;  0x800103
 184:	b0 93 04 01 	sts	0x0104, r27	;  0x800104
 188:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
 18c:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
 190:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
 194:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
 198:	01 96       	adiw	r24, 0x01	; 1
 19a:	a1 1d       	adc	r26, r1
 19c:	b1 1d       	adc	r27, r1
 19e:	80 93 05 01 	sts	0x0105, r24	;  0x800105
 1a2:	90 93 06 01 	sts	0x0106, r25	;  0x800106
 1a6:	a0 93 07 01 	sts	0x0107, r26	;  0x800107
 1aa:	b0 93 08 01 	sts	0x0108, r27	;  0x800108
 1ae:	bf 91       	pop	r27
 1b0:	af 91       	pop	r26
 1b2:	9f 91       	pop	r25
 1b4:	8f 91       	pop	r24
 1b6:	3f 91       	pop	r19
 1b8:	2f 91       	pop	r18
 1ba:	0f 90       	pop	r0
 1bc:	0f be       	out	0x3f, r0	; 63
 1be:	0f 90       	pop	r0
 1c0:	1f 90       	pop	r1
 1c2:	18 95       	reti
 1c4:	3f b7       	in	r19, 0x3f	; 63
 1c6:	f8 94       	cli
 1c8:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
 1cc:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
 1d0:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
 1d4:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
 1d8:	26 b5       	in	r18, 0x26	; 38
 1da:	a8 9b       	sbis	0x15, 0	; 21
 1dc:	05 c0       	rjmp	.+10     	;  0x1e8
 1de:	2f 3f       	cpi	r18, 0xFF	; 255
 1e0:	19 f0       	breq	.+6      	;  0x1e8
 1e2:	01 96       	adiw	r24, 0x01	; 1
 1e4:	a1 1d       	adc	r26, r1
 1e6:	b1 1d       	adc	r27, r1
 1e8:	3f bf       	out	0x3f, r19	; 63
 1ea:	ba 2f       	mov	r27, r26
 1ec:	a9 2f       	mov	r26, r25
 1ee:	98 2f       	mov	r25, r24
 1f0:	88 27       	eor	r24, r24
 1f2:	82 0f       	add	r24, r18
 1f4:	91 1d       	adc	r25, r1
 1f6:	a1 1d       	adc	r26, r1
 1f8:	b1 1d       	adc	r27, r1
 1fa:	bc 01       	movw	r22, r24
 1fc:	cd 01       	movw	r24, r26
 1fe:	42 e0       	ldi	r20, 0x02	; 2
 200:	66 0f       	add	r22, r22
 202:	77 1f       	adc	r23, r23
 204:	88 1f       	adc	r24, r24
 206:	99 1f       	adc	r25, r25
 208:	4a 95       	dec	r20
 20a:	d1 f7       	brne	.-12     	;  0x200
 20c:	08 95       	ret
 20e:	8f 92       	push	r8
 210:	9f 92       	push	r9
 212:	af 92       	push	r10
 214:	bf 92       	push	r11
 216:	cf 92       	push	r12
 218:	df 92       	push	r13
 21a:	ef 92       	push	r14
 21c:	ff 92       	push	r15
 21e:	6b 01       	movw	r12, r22
 220:	7c 01       	movw	r14, r24
 222:	0e 94 e2 00 	call	0x1c4	;  0x1c4
 226:	4b 01       	movw	r8, r22
 228:	5c 01       	movw	r10, r24
 22a:	c1 14       	cp	r12, r1
 22c:	d1 04       	cpc	r13, r1
 22e:	e1 04       	cpc	r14, r1
 230:	f1 04       	cpc	r15, r1
 232:	01 f1       	breq	.+64     	;  0x274
 234:	0e 94 12 02 	call	0x424	;  0x424
 238:	0e 94 e2 00 	call	0x1c4	;  0x1c4
 23c:	dc 01       	movw	r26, r24
 23e:	cb 01       	movw	r24, r22
 240:	88 19       	sub	r24, r8
 242:	99 09       	sbc	r25, r9
 244:	aa 09       	sbc	r26, r10
 246:	bb 09       	sbc	r27, r11
 248:	88 3e       	cpi	r24, 0xE8	; 232
 24a:	93 40       	sbci	r25, 0x03	; 3
 24c:	a1 05       	cpc	r26, r1
 24e:	b1 05       	cpc	r27, r1
 250:	60 f3       	brcs	.-40     	;  0x22a
 252:	21 e0       	ldi	r18, 0x01	; 1
 254:	c2 1a       	sub	r12, r18
 256:	d1 08       	sbc	r13, r1
 258:	e1 08       	sbc	r14, r1
 25a:	f1 08       	sbc	r15, r1
 25c:	88 ee       	ldi	r24, 0xE8	; 232
 25e:	88 0e       	add	r8, r24
 260:	83 e0       	ldi	r24, 0x03	; 3
 262:	98 1e       	adc	r9, r24
 264:	a1 1c       	adc	r10, r1
 266:	b1 1c       	adc	r11, r1
 268:	c1 14       	cp	r12, r1
 26a:	d1 04       	cpc	r13, r1
 26c:	e1 04       	cpc	r14, r1
 26e:	f1 04       	cpc	r15, r1
 270:	19 f7       	brne	.-58     	;  0x238
 272:	db cf       	rjmp	.-74     	;  0x22a
 274:	ff 90       	pop	r15
 276:	ef 90       	pop	r14
 278:	df 90       	pop	r13
 27a:	cf 90       	pop	r12
 27c:	bf 90       	pop	r11
 27e:	af 90       	pop	r10
 280:	9f 90       	pop	r9
 282:	8f 90       	pop	r8
 284:	08 95       	ret
 286:	78 94       	sei
 288:	84 b5       	in	r24, 0x24	; 36
 28a:	82 60       	ori	r24, 0x02	; 2
 28c:	84 bd       	out	0x24, r24	; 36
 28e:	84 b5       	in	r24, 0x24	; 36
 290:	81 60       	ori	r24, 0x01	; 1
 292:	84 bd       	out	0x24, r24	; 36
 294:	85 b5       	in	r24, 0x25	; 37
 296:	82 60       	ori	r24, 0x02	; 2
 298:	85 bd       	out	0x25, r24	; 37
 29a:	85 b5       	in	r24, 0x25	; 37
 29c:	81 60       	ori	r24, 0x01	; 1
 29e:	85 bd       	out	0x25, r24	; 37
 2a0:	ee e6       	ldi	r30, 0x6E	; 110
 2a2:	f0 e0       	ldi	r31, 0x00	; 0
 2a4:	80 81       	ld	r24, Z
 2a6:	81 60       	ori	r24, 0x01	; 1
 2a8:	80 83       	st	Z, r24
 2aa:	e1 e8       	ldi	r30, 0x81	; 129
 2ac:	f0 e0       	ldi	r31, 0x00	; 0
 2ae:	10 82       	st	Z, r1
 2b0:	80 81       	ld	r24, Z
 2b2:	82 60       	ori	r24, 0x02	; 2
 2b4:	80 83       	st	Z, r24
 2b6:	80 81       	ld	r24, Z
 2b8:	81 60       	ori	r24, 0x01	; 1
 2ba:	80 83       	st	Z, r24
 2bc:	e0 e8       	ldi	r30, 0x80	; 128
 2be:	f0 e0       	ldi	r31, 0x00	; 0
 2c0:	80 81       	ld	r24, Z
 2c2:	81 60       	ori	r24, 0x01	; 1
 2c4:	80 83       	st	Z, r24
 2c6:	e1 eb       	ldi	r30, 0xB1	; 177
 2c8:	f0 e0       	ldi	r31, 0x00	; 0
 2ca:	80 81       	ld	r24, Z
 2cc:	84 60       	ori	r24, 0x04	; 4
 2ce:	80 83       	st	Z, r24
 2d0:	e0 eb       	ldi	r30, 0xB0	; 176
 2d2:	f0 e0       	ldi	r31, 0x00	; 0
 2d4:	80 81       	ld	r24, Z
 2d6:	81 60       	ori	r24, 0x01	; 1
 2d8:	80 83       	st	Z, r24
 2da:	ea e7       	ldi	r30, 0x7A	; 122
 2dc:	f0 e0       	ldi	r31, 0x00	; 0
 2de:	80 81       	ld	r24, Z
 2e0:	84 60       	ori	r24, 0x04	; 4
 2e2:	80 83       	st	Z, r24
 2e4:	80 81       	ld	r24, Z
 2e6:	82 60       	ori	r24, 0x02	; 2
 2e8:	80 83       	st	Z, r24
 2ea:	80 81       	ld	r24, Z
 2ec:	81 60       	ori	r24, 0x01	; 1
 2ee:	80 83       	st	Z, r24
 2f0:	80 81       	ld	r24, Z
 2f2:	80 68       	ori	r24, 0x80	; 128
 2f4:	80 83       	st	Z, r24
 2f6:	10 92 c1 00 	sts	0x00C1, r1	;  0x8000c1
 2fa:	08 95       	ret
 2fc:	83 30       	cpi	r24, 0x03	; 3
 2fe:	81 f0       	breq	.+32     	;  0x320
 300:	28 f4       	brcc	.+10     	;  0x30c
 302:	81 30       	cpi	r24, 0x01	; 1
 304:	99 f0       	breq	.+38     	;  0x32c
 306:	82 30       	cpi	r24, 0x02	; 2
 308:	a1 f0       	breq	.+40     	;  0x332
 30a:	08 95       	ret
 30c:	87 30       	cpi	r24, 0x07	; 7
 30e:	a9 f0       	breq	.+42     	;  0x33a
 310:	88 30       	cpi	r24, 0x08	; 8
 312:	b9 f0       	breq	.+46     	;  0x342
 314:	84 30       	cpi	r24, 0x04	; 4
 316:	d1 f4       	brne	.+52     	;  0x34c
 318:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 31c:	8f 7d       	andi	r24, 0xDF	; 223
 31e:	03 c0       	rjmp	.+6      	;  0x326
 320:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 324:	8f 77       	andi	r24, 0x7F	; 127
 326:	80 93 80 00 	sts	0x0080, r24	;  0x800080
 32a:	08 95       	ret
 32c:	84 b5       	in	r24, 0x24	; 36
 32e:	8f 77       	andi	r24, 0x7F	; 127
 330:	02 c0       	rjmp	.+4      	;  0x336
 332:	84 b5       	in	r24, 0x24	; 36
 334:	8f 7d       	andi	r24, 0xDF	; 223
 336:	84 bd       	out	0x24, r24	; 36
 338:	08 95       	ret
 33a:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 33e:	8f 77       	andi	r24, 0x7F	; 127
 340:	03 c0       	rjmp	.+6      	;  0x348
 342:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 346:	8f 7d       	andi	r24, 0xDF	; 223
 348:	80 93 b0 00 	sts	0x00B0, r24	;  0x8000b0
 34c:	08 95       	ret
 34e:	cf 93       	push	r28
 350:	df 93       	push	r29
 352:	90 e0       	ldi	r25, 0x00	; 0
 354:	fc 01       	movw	r30, r24
 356:	e4 58       	subi	r30, 0x84	; 132
 358:	ff 4f       	sbci	r31, 0xFF	; 255
 35a:	24 91       	lpm	r18, Z
 35c:	fc 01       	movw	r30, r24
 35e:	e0 57       	subi	r30, 0x70	; 112
 360:	ff 4f       	sbci	r31, 0xFF	; 255
 362:	84 91       	lpm	r24, Z
 364:	88 23       	and	r24, r24
 366:	61 f1       	breq	.+88     	;  0x3c0
 368:	90 e0       	ldi	r25, 0x00	; 0
 36a:	88 0f       	add	r24, r24
 36c:	99 1f       	adc	r25, r25
 36e:	fc 01       	movw	r30, r24
 370:	e2 55       	subi	r30, 0x52	; 82
 372:	ff 4f       	sbci	r31, 0xFF	; 255
 374:	c5 91       	lpm	r28, Z+
 376:	d4 91       	lpm	r29, Z
 378:	fc 01       	movw	r30, r24
 37a:	ec 55       	subi	r30, 0x5C	; 92
 37c:	ff 4f       	sbci	r31, 0xFF	; 255
 37e:	a5 91       	lpm	r26, Z+
 380:	b4 91       	lpm	r27, Z
 382:	61 11       	cpse	r22, r1
 384:	09 c0       	rjmp	.+18     	;  0x398
 386:	9f b7       	in	r25, 0x3f	; 63
 388:	f8 94       	cli
 38a:	88 81       	ld	r24, Y
 38c:	20 95       	com	r18
 38e:	82 23       	and	r24, r18
 390:	88 83       	st	Y, r24
 392:	ec 91       	ld	r30, X
 394:	2e 23       	and	r18, r30
 396:	0b c0       	rjmp	.+22     	;  0x3ae
 398:	62 30       	cpi	r22, 0x02	; 2
 39a:	61 f4       	brne	.+24     	;  0x3b4
 39c:	9f b7       	in	r25, 0x3f	; 63
 39e:	f8 94       	cli
 3a0:	88 81       	ld	r24, Y
 3a2:	32 2f       	mov	r19, r18
 3a4:	30 95       	com	r19
 3a6:	83 23       	and	r24, r19
 3a8:	88 83       	st	Y, r24
 3aa:	ec 91       	ld	r30, X
 3ac:	2e 2b       	or	r18, r30
 3ae:	2c 93       	st	X, r18
 3b0:	9f bf       	out	0x3f, r25	; 63
 3b2:	06 c0       	rjmp	.+12     	;  0x3c0
 3b4:	8f b7       	in	r24, 0x3f	; 63
 3b6:	f8 94       	cli
 3b8:	e8 81       	ld	r30, Y
 3ba:	2e 2b       	or	r18, r30
 3bc:	28 83       	st	Y, r18
 3be:	8f bf       	out	0x3f, r24	; 63
 3c0:	df 91       	pop	r29
 3c2:	cf 91       	pop	r28
 3c4:	08 95       	ret
 3c6:	1f 93       	push	r17
 3c8:	cf 93       	push	r28
 3ca:	df 93       	push	r29
 3cc:	28 2f       	mov	r18, r24
 3ce:	30 e0       	ldi	r19, 0x00	; 0
 3d0:	f9 01       	movw	r30, r18
 3d2:	e8 59       	subi	r30, 0x98	; 152
 3d4:	ff 4f       	sbci	r31, 0xFF	; 255
 3d6:	84 91       	lpm	r24, Z
 3d8:	f9 01       	movw	r30, r18
 3da:	e4 58       	subi	r30, 0x84	; 132
 3dc:	ff 4f       	sbci	r31, 0xFF	; 255
 3de:	d4 91       	lpm	r29, Z
 3e0:	f9 01       	movw	r30, r18
 3e2:	e0 57       	subi	r30, 0x70	; 112
 3e4:	ff 4f       	sbci	r31, 0xFF	; 255
 3e6:	c4 91       	lpm	r28, Z
 3e8:	cc 23       	and	r28, r28
 3ea:	c1 f0       	breq	.+48     	;  0x41c
 3ec:	16 2f       	mov	r17, r22
 3ee:	81 11       	cpse	r24, r1
 3f0:	0e 94 7e 01 	call	0x2fc	;  0x2fc
 3f4:	ec 2f       	mov	r30, r28
 3f6:	f0 e0       	ldi	r31, 0x00	; 0
 3f8:	ee 0f       	add	r30, r30
 3fa:	ff 1f       	adc	r31, r31
 3fc:	ec 55       	subi	r30, 0x5C	; 92
 3fe:	ff 4f       	sbci	r31, 0xFF	; 255
 400:	a5 91       	lpm	r26, Z+
 402:	b4 91       	lpm	r27, Z
 404:	9f b7       	in	r25, 0x3f	; 63
 406:	f8 94       	cli
 408:	11 11       	cpse	r17, r1
 40a:	04 c0       	rjmp	.+8      	;  0x414
 40c:	8c 91       	ld	r24, X
 40e:	d0 95       	com	r29
 410:	d8 23       	and	r29, r24
 412:	02 c0       	rjmp	.+4      	;  0x418
 414:	ec 91       	ld	r30, X
 416:	de 2b       	or	r29, r30
 418:	dc 93       	st	X, r29
 41a:	9f bf       	out	0x3f, r25	; 63
 41c:	df 91       	pop	r29
 41e:	cf 91       	pop	r28
 420:	1f 91       	pop	r17
 422:	08 95       	ret
 424:	08 95       	ret
 426:	f8 94       	cli
 428:	ff cf       	rjmp	.-2      	;  0x428
