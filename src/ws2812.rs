use teensy4_bsp::board;
use teensy4_bsp::hal;

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
                50u8.reverse_bits(),
                50u8.reverse_bits(),
                50u8.reverse_bits(),
            ]; LED_COUNT],
            output,
            timer,
        }
    }

    pub fn set_color(&mut self, index: usize, color: [u8; 3]) {
        assert!(index < LED_COUNT);
        self.colors[index] = [
            color[1].reverse_bits(),
            color[0].reverse_bits(),
            color[2].reverse_bits(),
        ]; // set to format understood by the WS2812
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
