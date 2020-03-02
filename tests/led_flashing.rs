use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;

const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";

struct TestCase {
    cycle: u64,
    sram: [u8; 200],
}

const TEST_CASES: [TestCase; 18] = [
    TestCase {
        cycle: 0,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x87, 0x00, 0x09, 0x01, 0xff, 0x08, 0x7a, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x08, 0x00, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 74,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x87, 0x00, 0x09, 0x01, 0xff, 0x08, 0x7a, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x08, 0x02, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 110,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x03, 0x00, 0x09, 0x01, 0xff, 0x08, 0x81, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x00, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x80, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, // 0x0080  0x81 = 0x00, 0x84 = 0x00
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 112,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x03, 0x00, 0x09, 0x01, 0xff, 0x08, 0x81, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x00, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x80, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x00, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, // 0x0080  0x81 -> 0x03, 0x84 -> 0x01
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 150,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x07, 0x00, 0x09, 0x01, 0xff, 0x08, 0x7a, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x00, 0x00, // 0x0040  0x46 = 0x00
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x80, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0080  0x84 = 0x01
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 152,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x07, 0x00, 0x09, 0x01, 0xff, 0x08, 0x7a, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x01, 0x00, // 0x0040  0x46 -> 0x01
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x80, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // 0x0080  0x84 -> 0x02
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 173,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x87, 0x00, 0x09, 0x01, 0xff, 0x08, 0x7a, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x01, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x94, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 189,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x0d, 0x00, 0x09, 0x01, 0xff, 0x08, 0x89, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x01, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0x08, 0xa1, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0 0xb2 = 0x00
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 190,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x0d, 0x00, 0x09, 0x01, 0xff, 0x08, 0x0d, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030 tov2 (0x37) set
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038  ^
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x01, 0x00, // 0x0040  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048  |
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0x08, 0xa1, // 0x0058  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070  |
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078  |
            0x01, 0x03, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // 0x0080  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0  |
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8  |
            0x01, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0 0xb2 -> 0x01
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 246,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x35, 0x00, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x02, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08,
            0x35, // 0x0058  シミュレータ の I flg が 1 step 実行だと立たず、ジャンプ実行だと立つ
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 252,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x0d, 0x00, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x02, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0 0xb2 = 0x01
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 254,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x0d, 0x00, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x02, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0 0xb2 -> 0x02
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 280,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x00, 0x00, 0x25, 0x00, 0x02, 0x20, 0x9d, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x03, 0x00, // 0x0040 0x46 -> 0x03
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x08, 0x21, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, // 0x0080 0x84 -> 0x04
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 317,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0010
            0x00, 0x21, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x03, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x08, 0x21, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 536,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x10, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x18, 0x00, // 0x0010
            0x08, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x07, 0x00, // 0x0040 0x46 -> 0x07
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 1040,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x10, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x38, 0x00, // 0x0010
            0x28, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x0e, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 2624,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x10, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x26, 0x00, 0x00, 0x00, 0x98, 0x00, // 0x0010
            0x88, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x27, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
    TestCase {
        cycle: 3704,
        sram: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0000
            0x10, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // 0x0008
            0x00, 0x00, 0x37, 0x00, 0x00, 0x00, 0xdc, 0x00, // 0x0010
            0xcc, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0018
            0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x00, 0x00, // 0x0020
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0028
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // 0x0030
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0038
            0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x38, 0x00, // 0x0040
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0048
            0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // 0x0050
            0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0x08, 0x35, // 0x0058
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0060
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, // 0x0068
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0070
            0x00, 0x00, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0078
            0x01, 0x03, 0x00, 0x00, 0x39, 0x00, 0x00, 0x00, // 0x0080
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0088
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0090
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0098
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a0
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00a8
            0x01, 0x04, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00b0
            0x00, 0xf8, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, // 0x00b8
            0x20, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00c0
        ],
    },
];

#[test]
fn time_variation_of_sram() {
    let avr = ATmega328P::new();
    let mut timer0 = avr.new_timer0();
    let mut timer1 = avr.new_timer1();
    let mut timer2 = avr.new_timer2();
    let mut portb = avr.new_portb();
    let mut portc = avr.new_portc();
    let mut portd = avr.new_portd();
    avr.load_hex(SAMPLE_FILE_NAME);
    avr.initialize_sram();

    let mut test_case_iterator = TEST_CASES.iter();
    let mut test_case = test_case_iterator.next().unwrap();

    'clk: loop {
        if test_case.cycle == avr.cycle() {
            for index in 0..test_case.sram.len() {
                assert!(
                    test_case.sram[index] == avr.sram().borrow().get(index),
                    "cycle: {}, index = {:#x}",
                    test_case.cycle,
                    index
                );
            }
            match test_case_iterator.next() {
                Some(case) => test_case = case,
                None => break 'clk,
            }
        }
        avr.execute();
        timer0.clk_io();
        timer1.clk_io();
        timer2.clk_io();
        portb.clk_io();
        portc.clk_io();
        portd.clk_io();
    }
}
