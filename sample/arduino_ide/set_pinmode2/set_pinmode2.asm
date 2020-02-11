
set_pinmode2.ino.standard.hex:     file format ihex


Disassembly of section .sec1:

00000000 <.sec1>:
   0:	0c 94 52 00 	jmp	0xa4	;  0xa4
   4:	0c 94 64 00 	jmp	0xc8	;  0xc8
   8:	0c 94 64 00 	jmp	0xc8	;  0xc8
   c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  10:	0c 94 64 00 	jmp	0xc8	;  0xc8
  14:	0c 94 64 00 	jmp	0xc8	;  0xc8
  18:	0c 94 64 00 	jmp	0xc8	;  0xc8
  1c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  20:	0c 94 64 00 	jmp	0xc8	;  0xc8
  24:	0c 94 64 00 	jmp	0xc8	;  0xc8
  28:	0c 94 64 00 	jmp	0xc8	;  0xc8
  2c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  30:	0c 94 64 00 	jmp	0xc8	;  0xc8
  34:	0c 94 64 00 	jmp	0xc8	;  0xc8
  38:	0c 94 64 00 	jmp	0xc8	;  0xc8
  3c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  40:	0c 94 66 00 	jmp	0xcc	;  0xcc
  44:	0c 94 64 00 	jmp	0xc8	;  0xc8
  48:	0c 94 64 00 	jmp	0xc8	;  0xc8
  4c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  50:	0c 94 64 00 	jmp	0xc8	;  0xc8
  54:	0c 94 64 00 	jmp	0xc8	;  0xc8
  58:	0c 94 64 00 	jmp	0xc8	;  0xc8
  5c:	0c 94 64 00 	jmp	0xc8	;  0xc8
  60:	0c 94 64 00 	jmp	0xc8	;  0xc8
  64:	0c 94 64 00 	jmp	0xc8	;  0xc8
  68:	00 00       	nop
  6a:	00 00       	nop
  6c:	25 00       	.word	0x0025	; ????
  6e:	28 00       	.word	0x0028	; ????
  70:	2b 00       	.word	0x002b	; ????
  72:	00 00       	nop
  74:	00 00       	nop
  76:	24 00       	.word	0x0024	; ????
  78:	27 00       	.word	0x0027	; ????
  7a:	2a 00       	.word	0x002a	; ????
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
  a4:	11 24       	eor	r1, r1
  a6:	1f be       	out	0x3f, r1	; 63
  a8:	cf ef       	ldi	r28, 0xFF	; 255
  aa:	d8 e0       	ldi	r29, 0x08	; 8
  ac:	de bf       	out	0x3e, r29	; 62
  ae:	cd bf       	out	0x3d, r28	; 61
  b0:	21 e0       	ldi	r18, 0x01	; 1
  b2:	a0 e0       	ldi	r26, 0x00	; 0
  b4:	b1 e0       	ldi	r27, 0x01	; 1
  b6:	01 c0       	rjmp	.+2      	;  0xba
  b8:	1d 92       	st	X+, r1
  ba:	a9 30       	cpi	r26, 0x09	; 9
  bc:	b2 07       	cpc	r27, r18
  be:	e1 f7       	brne	.-8      	;  0xb8
  c0:	0e 94 b0 00 	call	0x160	;  0x160
  c4:	0c 94 15 01 	jmp	0x22a	;  0x22a
  c8:	0c 94 00 00 	jmp	0	;  0x0
  cc:	1f 92       	push	r1
  ce:	0f 92       	push	r0
  d0:	0f b6       	in	r0, 0x3f	; 63
  d2:	0f 92       	push	r0
  d4:	11 24       	eor	r1, r1
  d6:	2f 93       	push	r18
  d8:	3f 93       	push	r19
  da:	8f 93       	push	r24
  dc:	9f 93       	push	r25
  de:	af 93       	push	r26
  e0:	bf 93       	push	r27
  e2:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
  e6:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
  ea:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
  ee:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
  f2:	30 91 04 01 	lds	r19, 0x0104	;  0x800104
  f6:	23 e0       	ldi	r18, 0x03	; 3
  f8:	23 0f       	add	r18, r19
  fa:	2d 37       	cpi	r18, 0x7D	; 125
  fc:	58 f5       	brcc	.+86     	;  0x154
  fe:	01 96       	adiw	r24, 0x01	; 1
 100:	a1 1d       	adc	r26, r1
 102:	b1 1d       	adc	r27, r1
 104:	20 93 04 01 	sts	0x0104, r18	;  0x800104
 108:	80 93 05 01 	sts	0x0105, r24	;  0x800105
 10c:	90 93 06 01 	sts	0x0106, r25	;  0x800106
 110:	a0 93 07 01 	sts	0x0107, r26	;  0x800107
 114:	b0 93 08 01 	sts	0x0108, r27	;  0x800108
 118:	80 91 00 01 	lds	r24, 0x0100	;  0x800100
 11c:	90 91 01 01 	lds	r25, 0x0101	;  0x800101
 120:	a0 91 02 01 	lds	r26, 0x0102	;  0x800102
 124:	b0 91 03 01 	lds	r27, 0x0103	;  0x800103
 128:	01 96       	adiw	r24, 0x01	; 1
 12a:	a1 1d       	adc	r26, r1
 12c:	b1 1d       	adc	r27, r1
 12e:	80 93 00 01 	sts	0x0100, r24	;  0x800100
 132:	90 93 01 01 	sts	0x0101, r25	;  0x800101
 136:	a0 93 02 01 	sts	0x0102, r26	;  0x800102
 13a:	b0 93 03 01 	sts	0x0103, r27	;  0x800103
 13e:	bf 91       	pop	r27
 140:	af 91       	pop	r26
 142:	9f 91       	pop	r25
 144:	8f 91       	pop	r24
 146:	3f 91       	pop	r19
 148:	2f 91       	pop	r18
 14a:	0f 90       	pop	r0
 14c:	0f be       	out	0x3f, r0	; 63
 14e:	0f 90       	pop	r0
 150:	1f 90       	pop	r1
 152:	18 95       	reti
 154:	26 e8       	ldi	r18, 0x86	; 134
 156:	23 0f       	add	r18, r19
 158:	02 96       	adiw	r24, 0x02	; 2
 15a:	a1 1d       	adc	r26, r1
 15c:	b1 1d       	adc	r27, r1
 15e:	d2 cf       	rjmp	.-92     	;  0x104
 160:	78 94       	sei
 162:	84 b5       	in	r24, 0x24	; 36
 164:	82 60       	ori	r24, 0x02	; 2
 166:	84 bd       	out	0x24, r24	; 36
 168:	84 b5       	in	r24, 0x24	; 36
 16a:	81 60       	ori	r24, 0x01	; 1
 16c:	84 bd       	out	0x24, r24	; 36
 16e:	85 b5       	in	r24, 0x25	; 37
 170:	82 60       	ori	r24, 0x02	; 2
 172:	85 bd       	out	0x25, r24	; 37
 174:	85 b5       	in	r24, 0x25	; 37
 176:	81 60       	ori	r24, 0x01	; 1
 178:	85 bd       	out	0x25, r24	; 37
 17a:	80 91 6e 00 	lds	r24, 0x006E	;  0x80006e
 17e:	81 60       	ori	r24, 0x01	; 1
 180:	80 93 6e 00 	sts	0x006E, r24	;  0x80006e
 184:	10 92 81 00 	sts	0x0081, r1	;  0x800081
 188:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 18c:	82 60       	ori	r24, 0x02	; 2
 18e:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 192:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 196:	81 60       	ori	r24, 0x01	; 1
 198:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 19c:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 1a0:	81 60       	ori	r24, 0x01	; 1
 1a2:	80 93 80 00 	sts	0x0080, r24	;  0x800080
 1a6:	80 91 b1 00 	lds	r24, 0x00B1	;  0x8000b1
 1aa:	84 60       	ori	r24, 0x04	; 4
 1ac:	80 93 b1 00 	sts	0x00B1, r24	;  0x8000b1
 1b0:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 1b4:	81 60       	ori	r24, 0x01	; 1
 1b6:	80 93 b0 00 	sts	0x00B0, r24	;  0x8000b0
 1ba:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1be:	84 60       	ori	r24, 0x04	; 4
 1c0:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1c4:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1c8:	82 60       	ori	r24, 0x02	; 2
 1ca:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1ce:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1d2:	81 60       	ori	r24, 0x01	; 1
 1d4:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1d8:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 1dc:	80 68       	ori	r24, 0x80	; 128
 1de:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 1e2:	10 92 c1 00 	sts	0x00C1, r1	;  0x8000c1
 1e6:	ee e9       	ldi	r30, 0x9E	; 158
 1e8:	f0 e0       	ldi	r31, 0x00	; 0
 1ea:	24 91       	lpm	r18, Z
 1ec:	ea e8       	ldi	r30, 0x8A	; 138
 1ee:	f0 e0       	ldi	r31, 0x00	; 0
 1f0:	84 91       	lpm	r24, Z
 1f2:	88 23       	and	r24, r24
 1f4:	99 f0       	breq	.+38     	;  0x21c
 1f6:	90 e0       	ldi	r25, 0x00	; 0
 1f8:	88 0f       	add	r24, r24
 1fa:	99 1f       	adc	r25, r25
 1fc:	fc 01       	movw	r30, r24
 1fe:	ee 58       	subi	r30, 0x8E	; 142
 200:	ff 4f       	sbci	r31, 0xFF	; 255
 202:	a5 91       	lpm	r26, Z+
 204:	b4 91       	lpm	r27, Z
 206:	fc 01       	movw	r30, r24
 208:	e8 59       	subi	r30, 0x98	; 152
 20a:	ff 4f       	sbci	r31, 0xFF	; 255
 20c:	85 91       	lpm	r24, Z+
 20e:	94 91       	lpm	r25, Z
 210:	8f b7       	in	r24, 0x3f	; 63
 212:	f8 94       	cli
 214:	ec 91       	ld	r30, X
 216:	e2 2b       	or	r30, r18
 218:	ec 93       	st	X, r30
 21a:	8f bf       	out	0x3f, r24	; 63
 21c:	c0 e0       	ldi	r28, 0x00	; 0
 21e:	d0 e0       	ldi	r29, 0x00	; 0
 220:	20 97       	sbiw	r28, 0x00	; 0
 222:	f1 f3       	breq	.-4      	;  0x220
 224:	0e 94 00 00 	call	0	;  0x0
 228:	fb cf       	rjmp	.-10     	;  0x220
 22a:	f8 94       	cli
 22c:	ff cf       	rjmp	.-2      	;  0x22c
