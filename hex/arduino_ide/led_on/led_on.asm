
led_on.ino.standard.hex:     file format ihex


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
  40:	0c 94 70 00 	jmp	0xe0	;  0xe0
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
  6a:	00 00       	nop
  6c:	24 00       	.word	0x0024	; ????
  6e:	27 00       	.word	0x0027	; ????
  70:	2a 00       	.word	0x002a	; ????
  72:	00 00       	nop
  74:	00 00       	nop
  76:	25 00       	.word	0x0025	; ????
  78:	28 00       	.word	0x0028	; ????
  7a:	2b 00       	.word	0x002b	; ????
  7c:	04 04       	cpc	r0, r4
  7e:	04 04       	cpc	r0, r4
  80:	04 04       	cpc	r0, r4
  82:	04 04       	cpc	r0, r4
  84:	02 02       	muls	r16, r18
  86:	02 02       	muls	r16, r18
  88:	02 02       	muls	r16, r18
  8a:	03 03       	mulsu	r16, r19
  8c:	03 03       	mulsu	r16, r19
  8e:	03 03       	mulsu	r16, r19
  90:	01 02       	muls	r16, r17
  92:	04 08       	sbc	r0, r4
  94:	10 20       	and	r1, r0
  96:	40 80       	ld	r4, Z
  98:	01 02       	muls	r16, r17
  9a:	04 08       	sbc	r0, r4
  9c:	10 20       	and	r1, r0
  9e:	01 02       	muls	r16, r17
  a0:	04 08       	sbc	r0, r4
  a2:	10 20       	and	r1, r0
  a4:	00 00       	nop
  a6:	00 08       	sbc	r0, r0
  a8:	00 02       	muls	r16, r16
  aa:	01 00       	.word	0x0001	; ????
  ac:	00 03       	mulsu	r16, r16
  ae:	04 07       	cpc	r16, r20
	...
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
  d4:	0e 94 ba 00 	call	0x174	;  0x174
  d8:	0c 94 68 01 	jmp	0x2d0	;  0x2d0
  dc:	0c 94 00 00 	jmp	0	;  0x0
  e0:	1f 92       	push	r1
  e2:	0f 92       	push	r0
  e4:	0f b6       	in	r0, 0x3f	; 63
  e6:	0f 92       	push	r0
  e8:	11 24       	eor	r1, r1
  ea:	2f 93       	push	r18
  ec:	3f 93       	push	r19
  ee:	8f 93       	push	r24
  f0:	9f 93       	push	r25
  f2:	af 93       	push	r26
  f4:	bf 93       	push	r27
  f6:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
  fa:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
  fe:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
 102:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
 106:	30 91 04 01 	lds	r19, 0x0104	;  0x800104
 10a:	23 e0       	ldi	r18, 0x03	; 3
 10c:	23 0f       	add	r18, r19
 10e:	2d 37       	cpi	r18, 0x7D	; 125
 110:	58 f5       	brcc	.+86     	;  0x168
 112:	01 96       	adiw	r24, 0x01	; 1
 114:	a1 1d       	adc	r26, r1
 116:	b1 1d       	adc	r27, r1
 118:	20 93 04 01 	sts	0x0104, r18	;  0x800104
 11c:	80 93 05 01 	sts	0x0105, r24	;  0x800105
 120:	90 93 06 01 	sts	0x0106, r25	;  0x800106
 124:	a0 93 07 01 	sts	0x0107, r26	;  0x800107
 128:	b0 93 08 01 	sts	0x0108, r27	;  0x800108
 12c:	80 91 00 01 	lds	r24, 0x0100	;  0x800100
 130:	90 91 01 01 	lds	r25, 0x0101	;  0x800101
 134:	a0 91 02 01 	lds	r26, 0x0102	;  0x800102
 138:	b0 91 03 01 	lds	r27, 0x0103	;  0x800103
 13c:	01 96       	adiw	r24, 0x01	; 1
 13e:	a1 1d       	adc	r26, r1
 140:	b1 1d       	adc	r27, r1
 142:	80 93 00 01 	sts	0x0100, r24	;  0x800100
 146:	90 93 01 01 	sts	0x0101, r25	;  0x800101
 14a:	a0 93 02 01 	sts	0x0102, r26	;  0x800102
 14e:	b0 93 03 01 	sts	0x0103, r27	;  0x800103
 152:	bf 91       	pop	r27
 154:	af 91       	pop	r26
 156:	9f 91       	pop	r25
 158:	8f 91       	pop	r24
 15a:	3f 91       	pop	r19
 15c:	2f 91       	pop	r18
 15e:	0f 90       	pop	r0
 160:	0f be       	out	0x3f, r0	; 63
 162:	0f 90       	pop	r0
 164:	1f 90       	pop	r1
 166:	18 95       	reti
 168:	26 e8       	ldi	r18, 0x86	; 134
 16a:	23 0f       	add	r18, r19
 16c:	02 96       	adiw	r24, 0x02	; 2
 16e:	a1 1d       	adc	r26, r1
 170:	b1 1d       	adc	r27, r1
 172:	d2 cf       	rjmp	.-92     	;  0x118
 174:	78 94       	sei
 176:	84 b5       	in	r24, 0x24	; 36
 178:	82 60       	ori	r24, 0x02	; 2
 17a:	84 bd       	out	0x24, r24	; 36
 17c:	84 b5       	in	r24, 0x24	; 36
 17e:	81 60       	ori	r24, 0x01	; 1
 180:	84 bd       	out	0x24, r24	; 36
 182:	85 b5       	in	r24, 0x25	; 37
 184:	82 60       	ori	r24, 0x02	; 2
 186:	85 bd       	out	0x25, r24	; 37
 188:	85 b5       	in	r24, 0x25	; 37
 18a:	81 60       	ori	r24, 0x01	; 1
 18c:	85 bd       	out	0x25, r24	; 37
 18e:	80 91 6e 00 	lds	r24, 0x006E	;  0x80006e
 192:	81 60       	ori	r24, 0x01	; 1
 194:	80 93 6e 00 	sts	0x006E, r24	;  0x80006e
 198:	10 92 81 00 	sts	0x0081, r1	;  0x800081
 19c:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 1a0:	82 60       	ori	r24, 0x02	; 2
 1a2:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 1a6:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 1aa:	81 60       	ori	r24, 0x01	; 1
 1ac:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 1b0:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 1b4:	81 60       	ori	r24, 0x01	; 1
 1b6:	80 93 80 00 	sts	0x0080, r24	;  0x800080
 1ba:	80 91 b1 00 	lds	r24, 0x00B1	;  0x8000b1
 1be:	84 60       	ori	r24, 0x04	; 4
 1c0:	80 93 b1 00 	sts	0x00B1, r24	;  0x8000b1
 1c4:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 1c8:	81 60       	ori	r24, 0x01	; 1
 1ca:	80 93 b0 00 	sts	0x00B0, r24	;  0x8000b0
 1ce:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1d2:	84 60       	ori	r24, 0x04	; 4
 1d4:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1d8:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1dc:	82 60       	ori	r24, 0x02	; 2
 1de:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1e2:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1e6:	81 60       	ori	r24, 0x01	; 1
 1e8:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1ec:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1f0:	80 68       	ori	r24, 0x80	; 128
 1f2:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1f6:	10 92 c1 00 	sts	0x00C1, r1	;  0x8000c1
 1fa:	cd e9       	ldi	r28, 0x9D	; 157
 1fc:	d0 e0       	ldi	r29, 0x00	; 0
 1fe:	fe 01       	movw	r30, r28
 200:	24 91       	lpm	r18, Z
 202:	09 e8       	ldi	r16, 0x89	; 137
 204:	10 e0       	ldi	r17, 0x00	; 0
 206:	f8 01       	movw	r30, r16
 208:	84 91       	lpm	r24, Z
 20a:	88 23       	and	r24, r24
 20c:	99 f0       	breq	.+38     	;  0x234
 20e:	90 e0       	ldi	r25, 0x00	; 0
 210:	88 0f       	add	r24, r24
 212:	99 1f       	adc	r25, r25
 214:	fc 01       	movw	r30, r24
 216:	e8 59       	subi	r30, 0x98	; 152
 218:	ff 4f       	sbci	r31, 0xFF	; 255
 21a:	a5 91       	lpm	r26, Z+
 21c:	b4 91       	lpm	r27, Z
 21e:	fc 01       	movw	r30, r24
 220:	ee 58       	subi	r30, 0x8E	; 142
 222:	ff 4f       	sbci	r31, 0xFF	; 255
 224:	85 91       	lpm	r24, Z+
 226:	94 91       	lpm	r25, Z
 228:	8f b7       	in	r24, 0x3f	; 63
 22a:	f8 94       	cli
 22c:	9c 91       	ld	r25, X
 22e:	29 2b       	or	r18, r25
 230:	2c 93       	st	X, r18
 232:	8f bf       	out	0x3f, r24	; 63
 234:	91 eb       	ldi	r25, 0xB1	; 177
 236:	e9 2e       	mov	r14, r25
 238:	90 e0       	ldi	r25, 0x00	; 0
 23a:	f9 2e       	mov	r15, r25
 23c:	20 e0       	ldi	r18, 0x00	; 0
 23e:	c2 2e       	mov	r12, r18
 240:	20 e0       	ldi	r18, 0x00	; 0
 242:	d2 2e       	mov	r13, r18
 244:	f7 01       	movw	r30, r14
 246:	84 91       	lpm	r24, Z
 248:	fe 01       	movw	r30, r28
 24a:	94 91       	lpm	r25, Z
 24c:	f8 01       	movw	r30, r16
 24e:	24 91       	lpm	r18, Z
 250:	22 23       	and	r18, r18
 252:	b9 f0       	breq	.+46     	;  0x282
 254:	88 23       	and	r24, r24
 256:	39 f0       	breq	.+14     	;  0x266
 258:	83 30       	cpi	r24, 0x03	; 3
 25a:	19 f1       	breq	.+70     	;  0x2a2
 25c:	c0 f4       	brcc	.+48     	;  0x28e
 25e:	81 30       	cpi	r24, 0x01	; 1
 260:	31 f1       	breq	.+76     	;  0x2ae
 262:	82 30       	cpi	r24, 0x02	; 2
 264:	41 f1       	breq	.+80     	;  0x2b6
 266:	e2 2f       	mov	r30, r18
 268:	f0 e0       	ldi	r31, 0x00	; 0
 26a:	ee 0f       	add	r30, r30
 26c:	ff 1f       	adc	r31, r31
 26e:	ee 58       	subi	r30, 0x8E	; 142
 270:	ff 4f       	sbci	r31, 0xFF	; 255
 272:	a5 91       	lpm	r26, Z+
 274:	b4 91       	lpm	r27, Z
 276:	8f b7       	in	r24, 0x3f	; 63
 278:	f8 94       	cli
 27a:	2c 91       	ld	r18, X
 27c:	92 2b       	or	r25, r18
 27e:	9c 93       	st	X, r25
 280:	8f bf       	out	0x3f, r24	; 63
 282:	c1 14       	cp	r12, r1
 284:	d1 04       	cpc	r13, r1
 286:	f1 f2       	breq	.-68     	;  0x244
 288:	0e 94 00 00 	call	0	;  0x0
 28c:	db cf       	rjmp	.-74     	;  0x244
 28e:	87 30       	cpi	r24, 0x07	; 7
 290:	a9 f0       	breq	.+42     	;  0x2bc
 292:	88 30       	cpi	r24, 0x08	; 8
 294:	c9 f0       	breq	.+50     	;  0x2c8
 296:	84 30       	cpi	r24, 0x04	; 4
 298:	31 f7       	brne	.-52     	;  0x266
 29a:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 29e:	8f 7d       	andi	r24, 0xDF	; 223
 2a0:	03 c0       	rjmp	.+6      	;  0x2a8
 2a2:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 2a6:	8f 77       	andi	r24, 0x7F	; 127
 2a8:	80 93 80 00 	sts	0x0080, r24	;  0x800080
 2ac:	dc cf       	rjmp	.-72     	;  0x266
 2ae:	84 b5       	in	r24, 0x24	; 36
 2b0:	8f 77       	andi	r24, 0x7F	; 127
 2b2:	84 bd       	out	0x24, r24	; 36
 2b4:	d8 cf       	rjmp	.-80     	;  0x266
 2b6:	84 b5       	in	r24, 0x24	; 36
 2b8:	8f 7d       	andi	r24, 0xDF	; 223
 2ba:	fb cf       	rjmp	.-10     	;  0x2b2
 2bc:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 2c0:	8f 77       	andi	r24, 0x7F	; 127
 2c2:	80 93 b0 00 	sts	0x00B0, r24	;  0x8000b0
 2c6:	cf cf       	rjmp	.-98     	;  0x266
 2c8:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 2cc:	8f 7d       	andi	r24, 0xDF	; 223
 2ce:	f9 cf       	rjmp	.-14     	;  0x2c2
 2d0:	f8 94       	cli
 2d2:	ff cf       	rjmp	.-2      	;  0x2d2
