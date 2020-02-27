
led_flashing.ino.standard.hex:     file format ihex


Disassembly of section .sec1:

00000000 <.sec1>:

   // 読み方
   0:	0c 94 5c 00 	jmp	0xb8	;  0xb8
                ^ ここまでで 4バイト = 32 bit 命令.
                  プログラムメモリは 16bit なので、2セグメント分
      ^ 0c で 1バイト
   ^ これは 0 バイト目から読みますよ、を示す（0 からスタート）

   4:	0c 94 6e 00 	jmp	0xdc	;  0xdc
   ^ これは 4 バイト目から読みますよ、を示す
     プログラムメモリのアドレスとしては「2」になる！

   0x940c <- PC=0.  JMP
   0x005c
   = 1001_0100_0000_1100_0000_0000_0101_1100
   <- k = 1011100 = 0x5c = 0d92
   0:	0c 94 5c 00 	jmp	0xb8	;  0xb8 // RESET

   4:	0c 94 6e 00 	jmp	0xdc	;  0xdc // 外部割り込み 0
   8:	0c 94 6e 00 	jmp	0xdc	;  0xdc // 外部割り込み 1
   c:	0c 94 6e 00 	jmp	0xdc	;  0xdc // ピン入力？割り込み
  10:	0c 94 6e 00 	jmp	0xdc	;  0xdc // ピン入力？割り込み
  14:	0c 94 6e 00 	jmp	0xdc	;  0xdc // ピン入力？割り込み
  18:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Watchdog Timer 割り込み
  1c:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter2 コンペアマッチA
  20:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter2 コンペアマッチB
  24:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter2 Overflow
  28:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter1 Capture Event
  2c:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter1 コンペアマッチA
  30:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter1 コンペアマッチB
  34:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter1 Overflow
  38:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter0 コンペアマッチA
  3c:	0c 94 6e 00 	jmp	0xdc	;  0xdc // Timer/counter0 コンペアマッチB
  40:	0c 94 13 01 	jmp	0x226	;  0x226// Timer/counter0 Overflow
  44:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  48:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  4c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  50:	0c 94 6e 00 	jmp	0xdc	;  0xdc

  0x940c_006e
  = 1001_0100_0000_1100  0000_0000_0110_1110
  -> k = 1101110 = 6e
  54:	0c 94 6e 00 	jmp	0xdc	;  0xdc

  58:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  5c:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  60:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  64:	0c 94 6e 00 	jmp	0xdc	;  0xdc
  68:	00 00       	nop
  6a:	00 00       	nop
  6c:	24 00       	.word	0x0024	; ????

  0x0027
  = 0000_0000_0010_0111
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

  0x1124 = 0b0001_0001_0010_0100 <- 逆に並んでいる!
  0x2411 = 0b0010_0100_0001_0001 <- EOR
  b8:	11 24       	eor	r1, r1
  ^ 0xb8 = 0d184. これの半分は 0d92 = 0x5c でこれがメモリアドレス.

  0xbe1f = 0b1011_1110_0001_1111 <- OUT A=0b111111=0d63 r=1.
                                      IO(63) = SREG に R1 の内容を書き込み
  ba:	1f be       	out	0x3f, r1	; 63 --> SREG

  bc:	cf ef       	ldi	r28, 0xFF	; 255
  be:	d8 e0       	ldi	r29, 0x08	; 8
  c0:	de bf       	out	0x3e, r29	; 62 --> SPH
  c2:	cd bf       	out	0x3d, r28	; 61 --> SPL
  c4:	21 e0       	ldi	r18, 0x01	; 1
  c6:	a0 e0       	ldi	r26, 0x00	; 0
  c8:	b1 e0       	ldi	r27, 0x01	; 1

  c001
  1100_0000_0000_0001
  k = 1
  ca:	01 c0       	rjmp	.+2      	;  0xce

  cc:	1d 92       	st	X+, r1
  ce:	a9 30       	cpi	r26, 0x09	; 9
  d0:	b2 07       	cpc	r27, r18

  0xf7e1
  = 1111_0111_1110_0001
  -> k = 1111100
  d2:	e1 f7       	brne	.-8      	;  0xcc

  0x940e015d
  = 0b1001_0100_0000_1110_ 0000_0001_0101_1101 <- call
  k = 0000_0001_0101_1101
    = 0b0000000101011101
    = 0b101011101
    = 0x15d =  0d349 --> 0d698 = 0x2ba
  d4:	0e 94 5d 01 	call	0x2ba	;  0x2ba

  d8:	0c 94 cc 01 	jmp	0x398	;  0x398

  //  RESET 割り込み へ
  dc:	0c 94 00 00 	jmp	0	;  0x0

  e0:	e1 eb       	ldi	r30, 0xB1	; 177
  e2:	f0 e0       	ldi	r31, 0x00	; 0
  e4:	24 91       	lpm	r18, Z
  e6:	ed e9       	ldi	r30, 0x9D	; 157
  e8:	f0 e0       	ldi	r31, 0x00	; 0
  ea:	94 91       	lpm	r25, Z
  ec:	e9 e8       	ldi	r30, 0x89	; 137
  ee:	f0 e0       	ldi	r31, 0x00	; 0
  f0:	e4 91       	lpm	r30, Z
  f2:	ee 23       	and	r30, r30

  f0c9
  1111_0000_1100_1001
  k = 11001 = 0d25
  f4:	c9 f0       	breq	.+50     	;  0x128

  f6:	22 23       	and	r18, r18
  f8:	39 f0       	breq	.+14     	;  0x108
  fa:	23 30       	cpi	r18, 0x03	; 3
  fc:	01 f1       	breq	.+64     	;  0x13e

  fe:	a8 f4       	brcc	.+42     	;  0x12a
 100:	21 30       	cpi	r18, 0x01	; 1
 102:	19 f1       	breq	.+70     	;  0x14a
 104:	22 30       	cpi	r18, 0x02	; 2
 106:	29 f1       	breq	.+74     	;  0x152
 108:	f0 e0       	ldi	r31, 0x00	; 0
 10a:	ee 0f       	add	r30, r30
 10c:	ff 1f       	adc	r31, r31
 10e:	ee 58       	subi	r30, 0x8E	; 142
 110:	ff 4f       	sbci	r31, 0xFF	; 255
 112:	a5 91       	lpm	r26, Z+
 114:	b4 91       	lpm	r27, Z
 116:	2f b7       	in	r18, 0x3f	; 63
 118:	f8 94       	cli
 11a:	ec 91       	ld	r30, X
 11c:	81 11       	cpse	r24, r1
 11e:	26 c0       	rjmp	.+76     	;  0x16c
 120:	90 95       	com	r25
 122:	9e 23       	and	r25, r30
 124:	9c 93       	st	X, r25
 126:	2f bf       	out	0x3f, r18	; 63  --> SREG

 128:	08 95       	ret

 12a:	27 30       	cpi	r18, 0x07	; 7
 12c:	a9 f0       	breq	.+42     	;  0x158
 12e:	28 30       	cpi	r18, 0x08	; 8
 130:	c9 f0       	breq	.+50     	;  0x164
 132:	24 30       	cpi	r18, 0x04	; 4
 134:	49 f7       	brne	.-46     	;  0x108
 136:	20 91 80 00 	lds	r18, 0x0080	;  0x800080
 13a:	2f 7d       	andi	r18, 0xDF	; 223
 13c:	03 c0       	rjmp	.+6      	;  0x144
 13e:	20 91 80 00 	lds	r18, 0x0080	;  0x800080
 142:	2f 77       	andi	r18, 0x7F	; 127
 144:	20 93 80 00 	sts	0x0080, r18	;  0x800080
 148:	df cf       	rjmp	.-66     	;  0x108
 14a:	24 b5       	in	r18, 0x24	; 36
 14c:	2f 77       	andi	r18, 0x7F	; 127
 14e:	24 bd       	out	0x24, r18	; 36 --> 0x24+0x20=0x44=TCCR0a ??
 150:	db cf       	rjmp	.-74     	;  0x108
 152:	24 b5       	in	r18, 0x24	; 36
 154:	2f 7d       	andi	r18, 0xDF	; 223
 156:	fb cf       	rjmp	.-10     	;  0x14e
 158:	20 91 b0 00 	lds	r18, 0x00B0	;  0x8000b0
 15c:	2f 77       	andi	r18, 0x7F	; 127
 15e:	20 93 b0 00 	sts	0x00B0, r18	;  0x8000b0
 162:	d2 cf       	rjmp	.-92     	;  0x108
 164:	20 91 b0 00 	lds	r18, 0x00B0	;  0x8000b0
 168:	2f 7d       	andi	r18, 0xDF	; 223

 cff9
 = 1100111111111001
 -> 1111_1111_1001
 16a:	f9 cf       	rjmp	.-14     	;  0x15e

 16c:	9e 2b       	or	r25, r30
 16e:	da cf       	rjmp	.-76     	;  0x124
 170:	3f b7       	in	r19, 0x3f	; 63
 172:	f8 94       	cli
 174:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
 178:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
 17c:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
 180:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
 184:	26 b5       	in	r18, 0x26	; 38
 186:	a8 9b       	sbis	0x15, 0	; 21
 188:	05 c0       	rjmp	.+10     	;  0x194
 18a:	2f 3f       	cpi	r18, 0xFF	; 255
 18c:	19 f0       	breq	.+6      	;  0x194
 18e:	01 96       	adiw	r24, 0x01	; 1
 190:	a1 1d       	adc	r26, r1
 192:	b1 1d       	adc	r27, r1
 194:	3f bf       	out	0x3f, r19	; 63 --> SREG
 196:	ba 2f       	mov	r27, r26
 198:	a9 2f       	mov	r26, r25
 19a:	98 2f       	mov	r25, r24
 19c:	88 27       	eor	r24, r24
 19e:	bc 01       	movw	r22, r24
 1a0:	cd 01       	movw	r24, r26
 1a2:	62 0f       	add	r22, r18
 1a4:	71 1d       	adc	r23, r1
 1a6:	81 1d       	adc	r24, r1
 1a8:	91 1d       	adc	r25, r1
 1aa:	42 e0       	ldi	r20, 0x02	; 2
 1ac:	66 0f       	add	r22, r22
 1ae:	77 1f       	adc	r23, r23
 1b0:	88 1f       	adc	r24, r24
 1b2:	99 1f       	adc	r25, r25
 1b4:	4a 95       	dec	r20
 1b6:	d1 f7       	brne	.-12     	;  0x1ac

 1b8:	08 95       	ret

 1ba:	8f 92       	push	r8
 1bc:	9f 92       	push	r9
 1be:	af 92       	push	r10
 1c0:	bf 92       	push	r11
 1c2:	cf 92       	push	r12
 1c4:	df 92       	push	r13
 1c6:	ef 92       	push	r14
 1c8:	ff 92       	push	r15
 1ca:	0e 94 b8 00 	call	0x170	;  0x170
 1ce:	4b 01       	movw	r8, r22
 1d0:	5c 01       	movw	r10, r24
 1d2:	88 ee       	ldi	r24, 0xE8	; 232
 1d4:	c8 2e       	mov	r12, r24
 1d6:	83 e0       	ldi	r24, 0x03	; 3
 1d8:	d8 2e       	mov	r13, r24
 1da:	e1 2c       	mov	r14, r1
 1dc:	f1 2c       	mov	r15, r1
 1de:	0e 94 b8 00 	call	0x170	;  0x170
 1e2:	68 19       	sub	r22, r8
 1e4:	79 09       	sbc	r23, r9
 1e6:	8a 09       	sbc	r24, r10
 1e8:	9b 09       	sbc	r25, r11
 1ea:	68 3e       	cpi	r22, 0xE8	; 232
 1ec:	73 40       	sbci	r23, 0x03	; 3
 1ee:	81 05       	cpc	r24, r1
 1f0:	91 05       	cpc	r25, r1
 1f2:	a8 f3       	brcs	.-22     	;  0x1de
 1f4:	21 e0       	ldi	r18, 0x01	; 1
 1f6:	c2 1a       	sub	r12, r18
 1f8:	d1 08       	sbc	r13, r1
 1fa:	e1 08       	sbc	r14, r1
 1fc:	f1 08       	sbc	r15, r1
 1fe:	88 ee       	ldi	r24, 0xE8	; 232
 200:	88 0e       	add	r8, r24
 202:	83 e0       	ldi	r24, 0x03	; 3
 204:	98 1e       	adc	r9, r24
 206:	a1 1c       	adc	r10, r1
 208:	b1 1c       	adc	r11, r1
 20a:	c1 14       	cp	r12, r1
 20c:	d1 04       	cpc	r13, r1
 20e:	e1 04       	cpc	r14, r1
 210:	f1 04       	cpc	r15, r1
 212:	29 f7       	brne	.-54     	;  0x1de
 214:	ff 90       	pop	r15
 216:	ef 90       	pop	r14
 218:	df 90       	pop	r13
 21a:	cf 90       	pop	r12
 21c:	bf 90       	pop	r11
 21e:	af 90       	pop	r10
 220:	9f 90       	pop	r9
 222:	8f 90       	pop	r8

 224:	08 95       	ret

 226:	1f 92       	push	r1
 228:	0f 92       	push	r0
 22a:	0f b6       	in	r0, 0x3f	; 63
 22c:	0f 92       	push	r0
 22e:	11 24       	eor	r1, r1
 230:	2f 93       	push	r18
 232:	3f 93       	push	r19
 234:	8f 93       	push	r24
 236:	9f 93       	push	r25
 238:	af 93       	push	r26
 23a:	bf 93       	push	r27
 23c:	80 91 01 01 	lds	r24, 0x0101	;  0x800101
 240:	90 91 02 01 	lds	r25, 0x0102	;  0x800102
 244:	a0 91 03 01 	lds	r26, 0x0103	;  0x800103
 248:	b0 91 04 01 	lds	r27, 0x0104	;  0x800104
 24c:	30 91 00 01 	lds	r19, 0x0100	;  0x800100
 250:	23 e0       	ldi	r18, 0x03	; 3
 252:	23 0f       	add	r18, r19
 254:	2d 37       	cpi	r18, 0x7D	; 125
 256:	58 f5       	brcc	.+86     	;  0x2ae
 258:	01 96       	adiw	r24, 0x01	; 1
 25a:	a1 1d       	adc	r26, r1
 25c:	b1 1d       	adc	r27, r1
 25e:	20 93 00 01 	sts	0x0100, r18	;  0x800100
 262:	80 93 01 01 	sts	0x0101, r24	;  0x800101
 266:	90 93 02 01 	sts	0x0102, r25	;  0x800102
 26a:	a0 93 03 01 	sts	0x0103, r26	;  0x800103
 26e:	b0 93 04 01 	sts	0x0104, r27	;  0x800104
 272:	80 91 05 01 	lds	r24, 0x0105	;  0x800105
 276:	90 91 06 01 	lds	r25, 0x0106	;  0x800106
 27a:	a0 91 07 01 	lds	r26, 0x0107	;  0x800107
 27e:	b0 91 08 01 	lds	r27, 0x0108	;  0x800108
 282:	01 96       	adiw	r24, 0x01	; 1
 284:	a1 1d       	adc	r26, r1
 286:	b1 1d       	adc	r27, r1
 288:	80 93 05 01 	sts	0x0105, r24	;  0x800105
 28c:	90 93 06 01 	sts	0x0106, r25	;  0x800106
 290:	a0 93 07 01 	sts	0x0107, r26	;  0x800107
 294:	b0 93 08 01 	sts	0x0108, r27	;  0x800108
 298:	bf 91       	pop	r27
 29a:	af 91       	pop	r26
 29c:	9f 91       	pop	r25
 29e:	8f 91       	pop	r24
 2a0:	3f 91       	pop	r19
 2a2:	2f 91       	pop	r18
 2a4:	0f 90       	pop	r0
 2a6:	0f be       	out	0x3f, r0	; 63 --> SREG
 2a8:	0f 90       	pop	r0
 2aa:	1f 90       	pop	r1

 2ac:	18 95       	reti

 2ae:	26 e8       	ldi	r18, 0x86	; 134
 2b0:	23 0f       	add	r18, r19
 2b2:	02 96       	adiw	r24, 0x02	; 2
 2b4:	a1 1d       	adc	r26, r1
 2b6:	b1 1d       	adc	r27, r1
 2b8:	d2 cf       	rjmp	.-92     	;  0x25e
 2ba:	78 94       	sei
 2bc:	84 b5       	in	r24, 0x24	; 36
 2be:	82 60       	ori	r24, 0x02	; 2
 2c0:	84 bd       	out	0x24, r24	; 36 --> TCCR0A == タイマー
 2c2:	84 b5       	in	r24, 0x24	; 36
 2c4:	81 60       	ori	r24, 0x01	; 1
 2c6:	84 bd       	out	0x24, r24	; 36 --> TCCR0A
 2c8:	85 b5       	in	r24, 0x25	; 37
 2ca:	82 60       	ori	r24, 0x02	; 2
 2cc:	85 bd       	out	0x25, r24	; 37 --> TCCR0B
 2ce:	85 b5       	in	r24, 0x25	; 37
 2d0:	81 60       	ori	r24, 0x01	; 1
 2d2:	85 bd       	out	0x25, r24	; 37 --> TCCR0B
 2d4:	80 91 6e 00 	lds	r24, 0x006E	;  0x80006e
 2d8:	81 60       	ori	r24, 0x01	; 1
 2da:	80 93 6e 00 	sts	0x006E, r24	;  0x80006e
 2de:	10 92 81 00 	sts	0x0081, r1	;  0x800081
 2e2:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 2e6:	82 60       	ori	r24, 0x02	; 2
 2e8:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 2ec:	80 91 81 00 	lds	r24, 0x0081	;  0x800081
 2f0:	81 60       	ori	r24, 0x01	; 1
 2f2:	80 93 81 00 	sts	0x0081, r24	;  0x800081
 2f6:	80 91 80 00 	lds	r24, 0x0080	;  0x800080
 2fa:	81 60       	ori	r24, 0x01	; 1
 2fc:	80 93 80 00 	sts	0x0080, r24	;  0x800080
 300:	80 91 b1 00 	lds	r24, 0x00B1	;  0x8000b1
 304:	84 60       	ori	r24, 0x04	; 4
 306:	80 93 b1 00 	sts	0x00B1, r24	;  0x8000b1
 30a:	80 91 b0 00 	lds	r24, 0x00B0	;  0x8000b0
 30e:	81 60       	ori	r24, 0x01	; 1
 310:	80 93 b0 00 	sts	0x00B0, r24	;  0x8000b0
 314:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 318:	84 60       	ori	r24, 0x04	; 4
 31a:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 31e:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 322:	82 60       	ori	r24, 0x02	; 2
 324:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 328:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 32c:	81 60       	ori	r24, 0x01	; 1
 32e:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 332:	80 91 7a 00 	lds	r24, 0x007A	;  0x80007a
 336:	80 68       	ori	r24, 0x80	; 128
 338:	80 93 7a 00 	sts	0x007A, r24	;  0x80007a
 33c:	10 92 c1 00 	sts	0x00C1, r1	;  0x8000c1
 340:	ed e9       	ldi	r30, 0x9D	; 157
 342:	f0 e0       	ldi	r31, 0x00	; 0
 344:	24 91       	lpm	r18, Z
 346:	e9 e8       	ldi	r30, 0x89	; 137
 348:	f0 e0       	ldi	r31, 0x00	; 0
 34a:	84 91       	lpm	r24, Z
 34c:	88 23       	and	r24, r24

 f099
 1111_0000_1001_1001
 k = 10011 = 0d19
 34e:	99 f0       	breq	.+38     	;  0x376

 350:	90 e0       	ldi	r25, 0x00	; 0
 352:	88 0f       	add	r24, r24
 354:	99 1f       	adc	r25, r25
 356:	fc 01       	movw	r30, r24
 358:	e8 59       	subi	r30, 0x98	; 152
 35a:	ff 4f       	sbci	r31, 0xFF	; 255
 35c:	a5 91       	lpm	r26, Z+
 35e:	b4 91       	lpm	r27, Z
 360:	fc 01       	movw	r30, r24
 362:	ee 58       	subi	r30, 0x8E	; 142
 364:	ff 4f       	sbci	r31, 0xFF	; 255
 366:	85 91       	lpm	r24, Z+
 368:	94 91       	lpm	r25, Z
 36a:	8f b7       	in	r24, 0x3f	; 63
 36c:	f8 94       	cli
 36e:	ec 91       	ld	r30, X
 370:	e2 2b       	or	r30, r18
 372:	ec 93       	st	X, r30
 374:	8f bf       	out	0x3f, r24	; 63 --> EECR
 376:	c0 e0       	ldi	r28, 0x00	; 0
 378:	d0 e0       	ldi	r29, 0x00	; 0
 37a:	81 e0       	ldi	r24, 0x01	; 1

 940e 0070
 = 1001_0100_0000_1110 0000_0000_0111_0000
 k = 1110000 = 0d112
 37c:	0e 94 70 00 	call	0xe0	;  0xe0

 380:	0e 94 dd 00 	call	0x1ba	;  0x1ba
 384:	80 e0       	ldi	r24, 0x00	; 0
 386:	0e 94 70 00 	call	0xe0	;  0xe0
 38a:	0e 94 dd 00 	call	0x1ba	;  0x1ba
 38e:	20 97       	sbiw	r28, 0x00	; 0
 390:	a1 f3       	breq	.-24     	;  0x37a
 392:	0e 94 00 00 	call	0	;  0x0
 396:	f1 cf       	rjmp	.-30     	;  0x37a
 398:	f8 94       	cli
 39a:	ff cf       	rjmp	.-2      	;  0x39a
