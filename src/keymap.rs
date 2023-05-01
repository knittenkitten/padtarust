use crate::ws2812::WS2812;
use teensy4_bsp::hal;
use teensy4_bsp::pins::t41::Pins;
use usbd_human_interface_device::device::joystick::JoystickReport;
use usbd_human_interface_device::device::mouse::WheelMouseReport;
use usbd_human_interface_device::page::Keyboard;

const DEFAULT_JOY_X_CENTER: u16 = 500;
const DEFAULT_JOY_Y_CENTER: u16 = 500;
const DEFAULT_JOY_X_Y_ROTATION: u16 = 15;
const DEFAULT_JOY_X_DEAD_ZONE: u16 = 200;
const DEFAULT_JOY_Y_DEAD_ZONE: u16 = 200;
const DEFAULT_WASD_MODE: bool = false;

macro_rules! declare_gpio_pin {
    ($type:tt,$number:tt,$pin:tt,1) => {
        ::paste::paste! {
            type [<$type $number Pin>] = teensy4_pins::common::[<P $pin>];
            macro_rules! [<get_ $type $number _gpio_input>]{
                ($gpio1:ident, $gpio2:ident, $gpio3:ident, $gpio4:ident, $pins:ident) => {
                    $gpio1.input($pins.[<p $pin>])
                }
            }
        }
    };
    ($type:tt,$number:tt,$pin:tt,2) => {
        ::paste::paste! {
            type [<$type $number Pin>] = teensy4_pins::common::[<P $pin>];
            macro_rules! [<get_ $type $number _gpio_input>]{
                ($gpio1:ident, $gpio2:ident, $gpio3:ident, $gpio4:ident, $pins:ident) => {
                    $gpio2.input($pins.[<p $pin>])
                }
            }
        }
    };
    ($type:tt,$number:tt,$pin:tt,3) => {
        ::paste::paste! {
            type [<$type $number Pin>] = teensy4_pins::common::[<P $pin>];
            macro_rules! [<get_ $type $number _gpio_input>]{
                ($gpio1:ident, $gpio2:ident, $gpio3:ident, $gpio4:ident, $pins:ident) => {
                    $gpio3.input($pins.[<p $pin>])
                }
            }
        }
    };
    ($type:tt,$number:tt,$pin:tt,4) => {
        ::paste::paste! {
            type [<$type $number Pin>] = teensy4_pins::common::[<P $pin>];
            macro_rules! [<get_ $type $number _gpio_input>]{
                ($gpio1:ident, $gpio2:ident, $gpio3:ident, $gpio4:ident, $pins:ident) => {
                    $gpio4.input($pins.[<p $pin>])
                }
            }
        }
    };
}

macro_rules! declare_adc_pin {
    ($type:tt,$pin:tt,$adc_num:tt) => {
        ::paste::paste! {
            type [<$type Pin>] = teensy4_pins::common::[<P $pin>];
            macro_rules! [<get_ $type _adc_input>]{
                ($pins:ident) => {
                    hal::adc::AnalogInput::new($pins.[<p $pin>])
                }
            }
        }
    };
}

declare_gpio_pin!(Key, 0, 30, 3);
declare_gpio_pin!(Key, 1, 26, 1);
declare_gpio_pin!(Key, 2, 11, 2);
declare_gpio_pin!(Key, 3, 7, 2);
declare_gpio_pin!(Key, 4, 3, 4);
declare_gpio_pin!(Key, 5, 29, 4);
declare_gpio_pin!(Key, 6, 25, 1);
declare_gpio_pin!(Key, 7, 10, 2);
declare_gpio_pin!(Key, 8, 6, 2);
declare_gpio_pin!(Key, 9, 2, 4);
declare_gpio_pin!(Key, 10, 28, 3);
declare_gpio_pin!(Key, 11, 24, 1);
declare_gpio_pin!(Key, 12, 9, 2);
declare_gpio_pin!(Key, 13, 5, 4);
declare_gpio_pin!(Key, 14, 1, 1);
declare_gpio_pin!(Key, 15, 27, 1);
declare_gpio_pin!(Key, 16, 12, 2);
declare_gpio_pin!(Key, 17, 8, 2);
declare_gpio_pin!(Key, 18, 4, 4);
declare_gpio_pin!(Key, 19, 16, 1);
declare_gpio_pin!(Key, 20, 17, 1);
declare_gpio_pin!(JoyButton, 0, 18, 1);
declare_gpio_pin!(ScrollButton, 0, 0, 1);
declare_gpio_pin!(Rotary, 1, 21, 1);
declare_gpio_pin!(Rotary, 2, 22, 1);
declare_adc_pin!(JoyX, 14, 1);
declare_adc_pin!(JoyY, 15, 1);

pub struct KeymapIOPoints {
    key0: hal::gpio::Input<Key0Pin>,
    key1: hal::gpio::Input<Key1Pin>,
    key2: hal::gpio::Input<Key2Pin>,
    key3: hal::gpio::Input<Key3Pin>,
    key4: hal::gpio::Input<Key4Pin>,
    key5: hal::gpio::Input<Key5Pin>,
    key6: hal::gpio::Input<Key6Pin>,
    key7: hal::gpio::Input<Key7Pin>,
    key8: hal::gpio::Input<Key8Pin>,
    key9: hal::gpio::Input<Key9Pin>,
    key10: hal::gpio::Input<Key10Pin>,
    key11: hal::gpio::Input<Key11Pin>,
    key12: hal::gpio::Input<Key12Pin>,
    key13: hal::gpio::Input<Key13Pin>,
    key14: hal::gpio::Input<Key14Pin>,
    key15: hal::gpio::Input<Key15Pin>,
    key16: hal::gpio::Input<Key16Pin>,
    key17: hal::gpio::Input<Key17Pin>,
    key18: hal::gpio::Input<Key18Pin>,
    key19: hal::gpio::Input<Key19Pin>,
    key20: hal::gpio::Input<Key20Pin>,
    joy_button0: hal::gpio::Input<JoyButton0Pin>,
    scroll_button0: hal::gpio::Input<ScrollButton0Pin>,
    rotary1: hal::gpio::Input<Rotary1Pin>,
    rotary2: hal::gpio::Input<Rotary2Pin>,
    joyx: hal::adc::AnalogInput<JoyXPin, 1>,
    joyy: hal::adc::AnalogInput<JoyYPin, 1>,
    leds: WS2812,
}

impl KeymapIOPoints {
    pub fn new(
        gpio1: &mut hal::gpio::Port<1>,
        gpio2: &mut hal::gpio::Port<2>,
        gpio3: &mut hal::gpio::Port<3>,
        gpio4: &mut hal::gpio::Port<4>,
        pins: Pins,
        pit1: hal::pit::Pit<1>,
    ) -> KeymapIOPoints {
        KeymapIOPoints {
            key0: get_Key0_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key1: get_Key1_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key2: get_Key2_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key3: get_Key3_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key4: get_Key4_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key5: get_Key5_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key6: get_Key6_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key7: get_Key7_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key8: get_Key8_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key9: get_Key9_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key10: get_Key10_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key11: get_Key11_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key12: get_Key12_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key13: get_Key13_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key14: get_Key14_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key15: get_Key15_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key16: get_Key16_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key17: get_Key17_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key18: get_Key18_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key19: get_Key19_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            key20: get_Key20_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            joy_button0: get_JoyButton0_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            scroll_button0: get_ScrollButton0_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            rotary1: get_Rotary1_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            rotary2: get_Rotary2_gpio_input!(gpio1, gpio2, gpio3, gpio4, pins),
            joyx: get_JoyX_adc_input!(pins),
            joyy: get_JoyY_adc_input!(pins),
            leds: WS2812::new(gpio1.output(pins.p41), pit1),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum KeyboardAction {
    None,
    Layer0Momentary,
    Layer0Set,
    Layer1Momentary,
    Layer1Set,
    Layer2Momentary,
    Layer2Set,
    Layer3Momentary,
    Layer3Set,
    WasdModeOn,
    WasdModeOff,
    WasdModeToggle,
    Transparent,
    MouseLeftButton,
    MouseRightButton,
    MouseScrollButton,
    JoystickButton,
}

pub struct Report {
    buttons: [Option<Keyboard>; 26],
    button_count: usize,
    mouse_buttons: [Option<bool>; 3],
    joystick_button: Option<bool>,
}

impl Report {
    fn new() -> Report {
        Report {
            buttons: [None; 26],
            button_count: 0,
            mouse_buttons: [None; 3],
            joystick_button: None,
        }
    }
    fn add_mapping(&mut self, mapping: Mapping) {
        if mapping.button > Keyboard::ErrorUndefine {
            if self.button_count < 26 {
                self.buttons[self.button_count] = Some(mapping.button);
                self.button_count += 1;
            } else {
                // TODO: log
            }
        }
        match mapping.action {
            KeyboardAction::MouseLeftButton => {
                self.mouse_buttons[0] = Some(true);
            }
            KeyboardAction::MouseRightButton => {
                self.mouse_buttons[1] = Some(true);
            }
            KeyboardAction::MouseScrollButton => {
                self.mouse_buttons[2] = Some(true);
            }
            KeyboardAction::JoystickButton => self.joystick_button = Some(true),
            _ => {}
        }
    }
    fn finalize(
        &self,
        mouse: &mut WheelMouseReport,
        joystick: &mut JoystickReport,
    ) -> [Keyboard; 26] {
        let mut k: usize = 0;
        let mut nkro_keys = [Keyboard::NoEventIndicated; 26];
        for (i, button) in self.buttons.into_iter().enumerate() {
            if let Some(b) = button {
                nkro_keys[k] = b;
                k += i;
            } else {
                break;
            }
        }
        if self.joystick_button.is_some() {
            joystick.buttons = 1;
        }
        if self.mouse_buttons[0].is_some() {
            if self.mouse_buttons[1].is_some() {
                if self.mouse_buttons[2].is_some() {
                    mouse.buttons = 7;
                } else {
                    mouse.buttons = 3;
                }
            } else {
                mouse.buttons = 1;
            }
        } else if self.mouse_buttons[1].is_some() {
            if self.mouse_buttons[2].is_some() {
                mouse.buttons = 6;
            } else {
                mouse.buttons = 2;
            }
        } else if self.mouse_buttons[2].is_some() {
            mouse.buttons = 4;
        }
        nkro_keys
    }
}

#[derive(Copy, Clone)]
pub struct Mapping {
    action: KeyboardAction,
    button: Keyboard,
}

impl Mapping {
    fn from_button(b: Keyboard) -> Mapping {
        Mapping {
            action: KeyboardAction::None,
            button: b,
        }
    }
    fn from_action(a: KeyboardAction) -> Mapping {
        Mapping {
            action: a,
            button: Keyboard::NoEventIndicated,
        }
    }
    fn affects_reports(&self) -> Option<KeyboardAction> {
        match self.action {
            KeyboardAction::Layer0Momentary => Some(KeyboardAction::Layer0Momentary),
            KeyboardAction::Layer1Momentary => Some(KeyboardAction::Layer1Momentary),
            KeyboardAction::Layer2Momentary => Some(KeyboardAction::Layer2Momentary),
            KeyboardAction::Layer3Momentary => Some(KeyboardAction::Layer3Momentary),
            KeyboardAction::Layer0Set => Some(KeyboardAction::Layer0Set),
            KeyboardAction::Layer1Set => Some(KeyboardAction::Layer1Set),
            KeyboardAction::Layer2Set => Some(KeyboardAction::Layer2Set),
            KeyboardAction::Layer3Set => Some(KeyboardAction::Layer3Set),
            KeyboardAction::WasdModeOff => Some(KeyboardAction::WasdModeOff),
            KeyboardAction::WasdModeOn => Some(KeyboardAction::WasdModeOn),
            KeyboardAction::WasdModeToggle => Some(KeyboardAction::WasdModeToggle),
            _ => None,
        }
    }
}

pub struct Keymap {
    key_mappings: [[Mapping; 4]; 21],
    joy_button_mappings: [Mapping; 4],
    scroll_button_mappings: [Mapping; 4],
    /// KeyboardAction mappings are currently not supported in WASD mode to save processing power
    wasd_mappings: [[Mapping; 4]; 4],
    joy_x_center: u16,
    joy_y_center: u16,
    joy_x_y_rotation: u16,
    joy_x_deadzone: u16,
    joy_y_deadzone: u16,
}

pub struct KeymapState {
    wasd_mode: bool,
    stored_layer: u8,
    current_layer: u8,
    rotary_1_prev: bool,
}

impl KeymapState {
    pub fn default() -> KeymapState {
        KeymapState {
            wasd_mode: DEFAULT_WASD_MODE,
            stored_layer: 0,
            current_layer: 0,
            rotary_1_prev: false,
        }
    }
}

macro_rules! collapse_mapping {
    ($mapping_array: expr, $layer: expr) => {{
        let mut layer = $layer as usize;
        while $mapping_array[layer].action == KeyboardAction::Transparent && layer > 0 {
            layer -= 1;
        }
        $mapping_array[layer]
    }};
}

impl Keymap {
    pub fn default() -> Keymap {
        Keymap {
            key_mappings: [
                [
                    Mapping::from_button(Keyboard::Clear),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad0),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadDot),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadAdd),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadEqual),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad7),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad8),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad9),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadSubtract),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadMultiply),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad4),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad5),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad6),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadDivide),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadNumLockAndClear),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad1),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad2),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Keypad3),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::KeypadEnter),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::Space),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_action(KeyboardAction::Layer1Set),
                    Mapping::from_action(KeyboardAction::Layer2Set),
                    Mapping::from_action(KeyboardAction::Layer3Set),
                    Mapping::from_action(KeyboardAction::Layer0Set),
                ],
            ],
            joy_button_mappings: [
                Mapping::from_action(KeyboardAction::JoystickButton),
                Mapping::from_action(KeyboardAction::Transparent),
                Mapping::from_action(KeyboardAction::Transparent),
                Mapping::from_action(KeyboardAction::Transparent),
            ],
            scroll_button_mappings: [
                Mapping::from_action(KeyboardAction::MouseScrollButton),
                Mapping::from_action(KeyboardAction::Transparent),
                Mapping::from_action(KeyboardAction::Transparent),
                Mapping::from_action(KeyboardAction::Transparent),
            ],
            wasd_mappings: [
                [
                    Mapping::from_button(Keyboard::W),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::A),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::S),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
                [
                    Mapping::from_button(Keyboard::D),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                    Mapping::from_action(KeyboardAction::Transparent),
                ],
            ],
            joy_x_center: DEFAULT_JOY_X_CENTER,
            joy_y_center: DEFAULT_JOY_Y_CENTER,
            joy_x_y_rotation: DEFAULT_JOY_X_Y_ROTATION,
            joy_x_deadzone: DEFAULT_JOY_X_DEAD_ZONE,
            joy_y_deadzone: DEFAULT_JOY_Y_DEAD_ZONE,
        }
    }
    pub fn update(
        &mut self,
        adc1: &mut hal::adc::Adc<1>,
        io: &mut KeymapIOPoints,
        state: &mut KeymapState,
    ) -> ([Keyboard; 26], WheelMouseReport, JoystickReport) {
        let mut report = Report::new();

        // undo momentary changes
        state.current_layer = state.stored_layer;

        // check GPIO
        let keys = [
            io.key0.is_set(),
            io.key1.is_set(),
            io.key2.is_set(),
            io.key3.is_set(),
            io.key4.is_set(),
            io.key5.is_set(),
            io.key6.is_set(),
            io.key7.is_set(),
            io.key8.is_set(),
            io.key9.is_set(),
            io.key10.is_set(),
            io.key11.is_set(),
            io.key12.is_set(),
            io.key13.is_set(),
            io.key14.is_set(),
            io.key15.is_set(),
            io.key16.is_set(),
            io.key17.is_set(),
            io.key18.is_set(),
            io.key19.is_set(),
            io.key20.is_set(),
        ];
        let joy_button = io.joy_button0.is_set();
        let scroll_button = io.scroll_button0.is_set();
        let rotary_1_state = io.rotary1.is_set();

        // check ADC
        let joy_x = adc1.read_blocking(&mut io.joyx);
        let joy_y = adc1.read_blocking(&mut io.joyy);

        // generate mouse wheel report
        // TODO: allow the scroll wheel to do something else
        let mut mouse_report = WheelMouseReport::default();
        if rotary_1_state != state.rotary_1_prev {
            if io.rotary2.is_set() != rotary_1_state {
                // negative
                mouse_report.vertical_wheel = -1;
            } else {
                // positive
                mouse_report.vertical_wheel = 1;
            }
        }

        // generate joystick report
        let mut joystick_report = JoystickReport::default();
        let mut joy_x_f = joy_x as f32 - self.joy_x_center as f32;
        let mut joy_y_f = joy_y as f32 - self.joy_y_center as f32;
        // perform rotation
        if self.joy_x_y_rotation != 0 {
            let rads = (self.joy_x_y_rotation as f32).to_radians();
            let cosine = libm::cosf(rads);
            let sine = libm::sinf(rads);
            joy_x_f = (joy_x as f32) * cosine + (joy_y as f32) * sine;
            joy_y_f = -(joy_x as f32) * sine + (joy_y as f32) * cosine;
        }
        joy_x_f += self.joy_x_center as f32;
        joy_y_f += self.joy_y_center as f32;
        if !state.wasd_mode {
            // convert 10-bit to 8-bit then write x and y to report
            let joy_x_u8 = joy_x_f / 1023.0 * 255.0 - 128.0;
            let joy_y_u8 = joy_y_f / 1023.0 * 255.0 - 128.0;
            joystick_report.x = unsafe { joy_x_u8.to_int_unchecked::<i8>() };
            joystick_report.y = unsafe { joy_y_u8.to_int_unchecked::<i8>() };
        }

        // generate keyboard report and set joystick button
        // determine current layer and wasd mode before doing anything else
        let mut keyboard_operations = [KeyboardAction::None; 23];
        let mut keyboard_op_count: usize = 0;
        // keys in order
        for (i, key) in keys.into_iter().enumerate() {
            if key {
                let mapping = collapse_mapping!(self.key_mappings[i], state.current_layer);
                if let Some(op) = mapping.affects_reports() {
                    keyboard_operations[keyboard_op_count] = op;
                    keyboard_op_count += 1;
                }
            }
        }
        // then joystick button
        if joy_button {
            let mapping = collapse_mapping!(self.joy_button_mappings, state.current_layer);
            if let Some(op) = mapping.affects_reports() {
                keyboard_operations[keyboard_op_count] = op;
                keyboard_op_count += 1;
            }
        }
        // finally scroll button
        if scroll_button {
            let mapping = collapse_mapping!(self.scroll_button_mappings, state.current_layer);
            if let Some(op) = mapping.affects_reports() {
                keyboard_operations[keyboard_op_count] = op;
                keyboard_op_count += 1;
            }
        }
        if keyboard_op_count > 0 {
            //let mut layer_change = false;
            //let mut wasd_change = false;
            for op in keyboard_operations.iter().take(keyboard_op_count) {
                match op {
                    KeyboardAction::Layer0Momentary => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 0;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer1Momentary => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 1;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer2Momentary => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 2;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer3Momentary => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 3;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer0Set => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 0;
                        state.stored_layer = 0;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer1Set => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 1;
                        state.stored_layer = 1;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer2Set => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 2;
                        state.stored_layer = 2;
                        //layer_change = true;
                    }
                    KeyboardAction::Layer3Set => {
                        /* TODO: log if layer_change{
                        } */
                        state.current_layer = 3;
                        state.stored_layer = 3;
                        //layer_change = true;
                    }
                    KeyboardAction::WasdModeOff => {
                        /* TODO: log if wasd_change{
                        } */
                        state.wasd_mode = false;
                        //wasd_change = true;
                    }
                    KeyboardAction::WasdModeOn => {
                        /* TODO: log if wasd_change{
                        } */
                        state.wasd_mode = true;
                        //wasd_change = true;
                    }
                    KeyboardAction::WasdModeToggle => {
                        /* TODO: log if wasd_change{
                        } */
                        state.wasd_mode = !state.wasd_mode;
                        //wasd_change = true;
                    }
                    _ => {
                        unreachable!("Mapping::affects_reports does not function correctly")
                    }
                }
            }
        }
        // add WASD keys first
        if state.wasd_mode {
            if joy_y_f > (self.joy_y_center as f32 + self.joy_y_deadzone as f32) {
                let mapping = collapse_mapping!(self.wasd_mappings[0], state.current_layer);
                report.add_mapping(mapping);
            } else if joy_y_f < (self.joy_y_center as f32 + self.joy_y_deadzone as f32) {
                let mapping = collapse_mapping!(self.wasd_mappings[1], state.current_layer);
                report.add_mapping(mapping);
            }
            if joy_x_f < (self.joy_x_center as f32 - self.joy_x_deadzone as f32) {
                let mapping = collapse_mapping!(self.wasd_mappings[2], state.current_layer);
                report.add_mapping(mapping);
            } else if joy_y_f > (self.joy_x_center as f32 + self.joy_x_deadzone as f32) {
                let mapping = collapse_mapping!(self.wasd_mappings[3], state.current_layer);
                report.add_mapping(mapping);
            }
        }
        // then keys in order
        for (i, key) in keys.into_iter().enumerate() {
            if key {
                let mapping = collapse_mapping!(self.key_mappings[i], state.current_layer);
                report.add_mapping(mapping);
            }
        }
        // then joystick button
        if joy_button {
            let mapping = collapse_mapping!(self.joy_button_mappings, state.current_layer);
            report.add_mapping(mapping);
        }
        // finally scroll button
        if scroll_button {
            let mapping = collapse_mapping!(self.scroll_button_mappings, state.current_layer);
            report.add_mapping(mapping);
        }

        io.leds.show();
        let keys = report.finalize(&mut mouse_report, &mut joystick_report);
        (keys, mouse_report, joystick_report)
    }
}
