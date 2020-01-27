![](https://user-images.githubusercontent.com/11558164/72998818-c957eb00-3e41-11ea-989c-d32f183207a8.png)

# AVR Emulator

Rust で書く AVR マイクロコントローラのエミュレータです.
AVR 命令セットと ATmega328P のメモリマップを実装しています。

## Install & Setup

サンプルコードの実行

```sh
$ cargo run --bin core
```

または

```sh
$ just
```

テストの実行
```sh
$ cargo test // 標準出力なし
$ cargo test -- --nocapture // 標準出力あり
```
