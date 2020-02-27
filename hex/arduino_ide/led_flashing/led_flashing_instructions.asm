jmp	0xb8	;  0xb8
nop
cpc	r0, r4
muls	r16, r18
mulsu	r16, r19
sbc	r0, r4
and	r1, r0
ld	r4, Z

eor	r1, r1          // r1, r1 のXORてどういうことだ..？
out	0x3f, r1	; 63  // out A Rr : I/O(A) に Rr を書き込む. 0x3f = 63
ldi	r28, 0xFF	; 255 // R28 を即値で更新
rjmp	.+2      	;  0xce
st	X+, r1          // X+ ポインタを使用している
cpi	r26, 0x09	; 9
cpc	r27, r18
brne	.-8      	;  0xcc
call	0x2ba	;  0x2ba
jmp	0x398	;  0x398
lpm	r18, Z
and	r30, r30
breq	.+50     	;  0x128
cpi	r18, 0x03	; 3
breq	.+64     	;  0x13e
brcc	.+42     	;  0x12a
cpi	r18, 0x01	; 1
breq	.+70     	;  0x14a
cpi	r18, 0x02	; 2
breq	.+74     	;  0x152
ldi	r31, 0x00	; 0
add	r30, r30
adc	r31, r31
subi	r30, 0x8E	; 142
sbci	r31, 0xFF	; 255

lpm	r26, Z+
lpm	r27, Z
in	r18, 0x3f	; 63
cli
ld	r30, X
cpse	r24, r1
rjmp	.+76     	;  0x16c
com	r25
and	r25, r30
st	X, r25
out	0x3f, r18	; 63
ret
cpi	r18, 0x07	; 7
breq	.+42     	;  0x158
cpi	r18, 0x08	; 8
breq	.+50     	;  0x164
cpi	r18, 0x04	; 4
brne	.-46     	;  0x108
lds	r18, 0x0080	;  0x800080
andi	r18, 0xDF	; 223
rjmp	.+6      	;  0x144
lds	r18, 0x0080	;  0x800080
andi	r18, 0x7F	; 127
sts	0x0080, r18	;  0x800080
rjmp	.-66     	;  0x108
in	r18, 0x24	; 36
andi	r18, 0x7F	; 127
out	0x24, r18	; 36
rjmp	.-74     	;  0x108
in	r18, 0x24	; 36
andi	r18, 0xDF	; 223
rjmp	.-10     	;  0x14e
lds	r18, 0x00B0	;  0x8000b0
andi	r18, 0x7F	; 127
sts	0x00B0, r18	;  0x8000b0
rjmp	.-92     	;  0x108
lds	r18, 0x00B0	;  0x8000b0
andi	r18, 0xDF	; 223
rjmp	.-14     	;  0x15e
or	r25, r30
rjmp	.-76     	;  0x124
in	r19, 0x3f	; 63
cli
lds	r24, 0x0105	;  0x800105
lds	r25, 0x0106	;  0x800106
lds	r26, 0x0107	;  0x800107
lds	r27, 0x0108	;  0x800108
in	r18, 0x26	; 38
sbis	0x15, 0	; 21
rjmp	.+10     	;  0x194
cpi	r18, 0xFF	; 255
breq	.+6      	;  0x194
adiw	r24, 0x01	; 1
adc	r26, r1
adc	r27, r1
out	0x3f, r19	; 63
mov	r27, r26
mov	r26, r25
mov	r25, r24
eor	r24, r24
movw	r22, r24
movw	r24, r26
add	r22, r18
adc	r23, r1
adc	r24, r1
adc	r25, r1
ldi	r20, 0x02	; 2
add	r22, r22
adc	r23, r23
adc	r24, r24
adc	r25, r25
dec	r20
brne	.-12     	;  0x1ac
ret
push	r8
push	r9
push	r10
push	r11
push	r12
push	r13
push	r14
push	r15
call	0x170	;  0x170
movw	r8, r22
movw	r10, r24
ldi	r24, 0xE8	; 232
mov	r12, r24
ldi	r24, 0x03	; 3
mov	r13, r24
mov	r14, r1
mov	r15, r1
call	0x170	;  0x170
sub	r22, r8
sbc	r23, r9
sbc	r24, r10
sbc	r25, r11
cpi	r22, 0xE8	; 232
sbci	r23, 0x03	; 3
cpc	r24, r1
cpc	r25, r1
brcs	.-22     	;  0x1de
ldi	r18, 0x01	; 1
sub	r12, r18
sbc	r13, r1
sbc	r14, r1
sbc	r15, r1
ldi	r24, 0xE8	; 232
add	r8, r24
ldi	r24, 0x03	; 3
adc	r9, r24
adc	r10, r1
adc	r11, r1
cp	r12, r1
cpc	r13, r1
cpc	r14, r1
cpc	r15, r1
brne	.-54     	;  0x1de
pop	r15
pop	r14
pop	r13
pop	r12
pop	r11
pop	r10
pop	r9
pop	r8
ret
push	r1
push	r0
in	r0, 0x3f	; 63
push	r0
eor	r1, r1
push	r18
push	r19
push	r24
push	r25
push	r26
push	r27
lds	r24, 0x0101	;  0x800101
lds	r25, 0x0102	;  0x800102
lds	r26, 0x0103	;  0x800103
lds	r27, 0x0104	;  0x800104
lds	r19, 0x0100	;  0x800100
ldi	r18, 0x03	; 3
add	r18, r19
cpi	r18, 0x7D	; 125
brcc	.+86     	;  0x2ae
adiw	r24, 0x01	; 1
adc	r26, r1
adc	r27, r1
sts	0x0100, r18	;  0x800100
sts	0x0101, r24	;  0x800101
sts	0x0102, r25	;  0x800102
sts	0x0103, r26	;  0x800103
sts	0x0104, r27	;  0x800104
lds	r24, 0x0105	;  0x800105
lds	r25, 0x0106	;  0x800106
lds	r26, 0x0107	;  0x800107
lds	r27, 0x0108	;  0x800108
adiw	r24, 0x01	; 1
adc	r26, r1
adc	r27, r1
sts	0x0105, r24	;  0x800105
sts	0x0106, r25	;  0x800106
sts	0x0107, r26	;  0x800107
sts	0x0108, r27	;  0x800108
pop	r27
pop	r26
pop	r25
pop	r24
pop	r19
pop	r18
pop	r0
out	0x3f, r0	; 63
pop	r0
pop	r1
reti
ldi	r18, 0x86	; 134
add	r18, r19
adiw	r24, 0x02	; 2
adc	r26, r1
adc	r27, r1
rjmp	.-92     	;  0x25e
sei
in	r24, 0x24	; 36
ori	r24, 0x02	; 2
out	0x24, r24	; 36
in	r24, 0x24	; 36
ori	r24, 0x01	; 1
out	0x24, r24	; 36
in	r24, 0x25	; 37
ori	r24, 0x02	; 2
out	0x25, r24	; 37
in	r24, 0x25	; 37
ori	r24, 0x01	; 1
out	0x25, r24	; 37
lds	r24, 0x006E	;  0x80006e
ori	r24, 0x01	; 1
sts	0x006E, r24	;  0x80006e
sts	0x0081, r1	;  0x800081
lds	r24, 0x0081	;  0x800081
ori	r24, 0x02	; 2
sts	0x0081, r24	;  0x800081
lds	r24, 0x0081	;  0x800081
ori	r24, 0x01	; 1
sts	0x0081, r24	;  0x800081
lds	r24, 0x0080	;  0x800080
ori	r24, 0x01	; 1
sts	0x0080, r24	;  0x800080
lds	r24, 0x00B1	;  0x8000b1
ori	r24, 0x04	; 4
sts	0x00B1, r24	;  0x8000b1
lds	r24, 0x00B0	;  0x8000b0
ori	r24, 0x01	; 1
sts	0x00B0, r24	;  0x8000b0
lds	r24, 0x007A	;  0x80007a
ori	r24, 0x04	; 4
sts	0x007A, r24	;  0x80007a
lds	r24, 0x007A	;  0x80007a
ori	r24, 0x02	; 2
sts	0x007A, r24	;  0x80007a
lds	r24, 0x007A	;  0x80007a
ori	r24, 0x01	; 1
sts	0x007A, r24	;  0x80007a
lds	r24, 0x007A	;  0x80007a
ori	r24, 0x80	; 128
sts	0x007A, r24	;  0x80007a
sts	0x00C1, r1	;  0x8000c1
ldi	r30, 0x9D	; 157
ldi	r31, 0x00	; 0
lpm	r18, Z
ldi	r30, 0x89	; 137
ldi	r31, 0x00	; 0
lpm	r24, Z
and	r24, r24
breq	.+38     	;  0x376
ldi	r25, 0x00	; 0
add	r24, r24
adc	r25, r25
movw	r30, r24
subi	r30, 0x98	; 152
sbci	r31, 0xFF	; 255
lpm	r26, Z+
lpm	r27, Z
movw	r30, r24
subi	r30, 0x8E	; 142
sbci	r31, 0xFF	; 255
lpm	r24, Z+
lpm	r25, Z
in	r24, 0x3f	; 63
cli
ld	r30, X
or	r30, r18
st	X, r30
out	0x3f, r24	; 63
ldi	r28, 0x00	; 0
ldi	r29, 0x00	; 0
ldi	r24, 0x01	; 1
call	0xe0	;  0xe0
call	0x1ba	;  0x1ba
ldi	r24, 0x00	; 0
call	0xe0	;  0xe0
call	0x1ba	;  0x1ba
sbiw	r28, 0x00	; 0
breq	.-24     	;  0x37a
call	0	;  0x0
rjmp	.-30     	;  0x37a
cli
rjmp	.-2      	;  0x39a
