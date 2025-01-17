void setup() {
  // ピンモードを INPUT or OUTPUT で設定する. 
  // デジタル 0 ~ 13 番はデフォルトで入力に設定されている.
  //
  //  0 ~ 13 : デジタルIO
  //  14 ~ 19 : アナログIO
  pinMode(13, OUTPUT);
}

void loop() {
  // OUTPUT と設定しているピンに電圧を出力する.
  // (ピンが INPUT モードの時にこの digitalWrite を行うと、入力プルアップがオンになる)
  // （内臓プルアップ機能：各入力端子を、抵抗を通して Vcc や GND と接続するもの）
  digitalWrite(13, HIGH);
  //
  // デジタル入出力端子の電気特性
  // Arduino のデジタル入出力ピンは AVR マイコンの電気特性を引き継いでいる. 
  // 1ピンあたりに流入/吐き出しが可能な電流は 20mA まで.
  // さらに、Arduino 入出力ピンを数本束ねた単位毎にそれぞれ X mA まで流せるかが決まっている.
  // また、入力端子にかけられる電圧の範囲は (GND~0.5V) ~ (Vcc+0.5) の範囲に入っている
  // 必要がある. これを越えるとラッチアップを起こす可能性がある.
  // ( Arduino の Vcc に電圧がかかっていない時は Vcc = 0V. つまり、きちんと電源電圧が
  //   かかっていない時に入力信号が入ってくると故障する恐れがある)
  //
  delay(1000);

  digitalWrite(13, LOW);
  delay(1000);
}
