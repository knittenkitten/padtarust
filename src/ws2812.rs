use teensy4_bsp::board;
use teensy4_bsp::hal;
// borrowed from the NeoPixel library
const GAMMA_TABLE: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 7,
    7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14, 14, 15, 15, 16, 16, 17,
    17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27, 27, 28, 29, 29, 30, 31,
    31, 32, 33, 34, 34, 35, 36, 37, 38, 38, 39, 40, 41, 42, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 68, 69, 70, 71, 72, 73, 75, 76, 77,
    78, 80, 81, 82, 84, 85, 86, 88, 89, 90, 92, 93, 94, 96, 97, 99, 100, 102, 103, 105, 106, 108,
    109, 111, 112, 114, 115, 117, 119, 120, 122, 124, 125, 127, 129, 130, 132, 134, 136, 137, 139,
    141, 143, 145, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 176,
    178, 180, 182, 184, 186, 188, 191, 193, 195, 197, 199, 202, 204, 206, 209, 211, 213, 215, 218,
    220, 223, 225, 227, 230, 232, 235, 237, 240, 242, 245, 247, 250, 252, 255,
];
// 16mA per channel = 48mA maximum
// power usage per research is an approximately linear curve with a minimum of about 3.3mA per channel
// power budget is ~400mA with 100mA used by teensy; would like to go slightly lower than this
// power budget = (3.3mA + (((max RGB value) / 255) * (48mA - 3.3mA))) * LED_COUNT
// solving for a budget of 350mA we obtain 350mA / LED_COUNT = 14.58mA = 3.3mA + (((max RGB value) / 255) * (48mA - 3.3mA))
// 11.28mA = ((max RGB value) / 255) * (44.7mA)
// 0.25 = ((max RGB value) / 255)
// max RGB value = 63.75
// using gamma correction this amounts to 150
const MAX_COLOR_VALUE: u8 = 150;
const LED_COUNT: usize = 24;
const T0H_NS: fugit::Duration<u32, 1, 1000000> = fugit::Duration::<u32, 1, 1000000>::nanos(275u32);
const T1H_NS: fugit::Duration<u32, 1, 1000000> = fugit::Duration::<u32, 1, 1000000>::nanos(750u32);
const T0L_NS: fugit::Duration<u32, 1, 1000000> = fugit::Duration::<u32, 1, 1000000>::nanos(750u32);
const T1L_NS: fugit::Duration<u32, 1, 1000000> = fugit::Duration::<u32, 1, 1000000>::nanos(275u32);
const RESET_US: u32 = 300;

pub type LedPin = teensy4_pins::t41::P41;
pub struct WS2812 {
    colors: [[u8; 3]; LED_COUNT], // stored in GRB with each component in reversed bit order using reverse_bits()
    output: hal::gpio::Output<LedPin>,
    timer: hal::timer::BlockingPit<1, { board::PERCLK_FREQUENCY }>,
}

impl WS2812 {
    pub fn new(output: hal::gpio::Output<LedPin>, pit1: hal::pit::Pit<1>) -> WS2812 {
        let timer = hal::timer::Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit1);
        WS2812 {
            colors: [[
                GAMMA_TABLE[MAX_COLOR_VALUE as usize].reverse_bits(),
                GAMMA_TABLE[MAX_COLOR_VALUE as usize].reverse_bits(),
                GAMMA_TABLE[MAX_COLOR_VALUE as usize].reverse_bits(),
            ]; LED_COUNT],
            output,
            timer,
        }
    }

    pub fn set_color(&mut self, index: usize, color: [u8; 3]) {
        assert!(index < LED_COUNT);
        // apply maximum
        let max_corrected = [
            unsafe {
                (color[0] as f32 * (MAX_COLOR_VALUE as f32 / 255.0)).to_int_unchecked::<usize>()
            },
            unsafe {
                (color[1] as f32 * (MAX_COLOR_VALUE as f32 / 255.0)).to_int_unchecked::<usize>()
            },
            unsafe {
                (color[2] as f32 * (MAX_COLOR_VALUE as f32 / 255.0)).to_int_unchecked::<usize>()
            },
        ];
        let gamma_corrected = [
            GAMMA_TABLE[max_corrected[0]],
            GAMMA_TABLE[max_corrected[1]],
            GAMMA_TABLE[max_corrected[2]],
        ];
        // set to format understood by the WS2812 (GBR with reversed bits)
        self.colors[index] = [
            gamma_corrected[1].reverse_bits(),
            gamma_corrected[0].reverse_bits(),
            gamma_corrected[2].reverse_bits(),
        ];
    }

    pub fn show(&mut self) {
        for led in self.colors {
            for color in led {
                for i in 0..8 {
                    let high = color >> (7 - i) & 1;
                    self.output.set();
                    if high > 0 {
                        self.timer.block(T1H_NS);
                    } else {
                        self.timer.block(T0H_NS);
                    }
                    self.output.clear();
                    if high > 0 {
                        self.timer.block(T1L_NS);
                    } else {
                        self.timer.block(T0L_NS);
                    }
                }
            }
        }
        self.timer.block_us(RESET_US);
    }
}
