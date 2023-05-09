#![no_std]
#![no_main]

mod keymap;
mod keymap_common;
mod ws2812;

use bsp::{
    board,
    hal::{adc::ResolutionBits, iomuxc},
};
use imxrt_usbd::BusAdapter;
use teensy4_bsp as bsp;
use teensy4_panic as _;
use usb_device::{
    bus::UsbBusAllocator,
    class_prelude::UsbBus,
    prelude::{UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_hid::hid_class::HIDClass;
use usbd_hid::{
    descriptor::generator_prelude::*,
    hid_class::{HidClassSettings, ProtocolModeConfig},
};
//use usbd_serial::SerialPort;

use crate::keymap::{Keymap, KeymapIOPoints, KeymapState};

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] mouse_buttons=input;
            };
            (usage_page = GENERIC_DESKTOP, usage = WHEEL,) = {
                #[item_settings data,variable,relative] wheel=input;
            };
        }
    },
)]
struct MouseReport {
    pub mouse_buttons: u8,
    pub wheel: i8,
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = JOYSTICK) = {
        (collection = PHYSICAL, usage = JOYSTICK) = {
            (usage_page = GENERIC_DESKTOP,) = {
                (usage = X, usage_min = 0x000, usage_max = 0x3FF) = {
                    #[item_settings data,variable,absolute] x=input;
                };
                (usage = Y, usage_min = 0x000, usage_max = 0x3FF) = {
                    #[item_settings data,variable,absolute] y=input;
                };
            };
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] joy_buttons=input;
            };
        }
    },
)]
struct JoystickReport {
    pub joy_buttons: u8,
    pub x: u16,
    pub y: u16,
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
    },
)]
struct KeyboardReport {
    pub modifier: u8,
    pub keycodes: [u8; 26],
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x29C) = {
            #[item_settings data,array,absolute,not_null] consumer_keycode=input;
        };
    }
)]
struct ConsumerReport {
    pub consumer_keycode: u16,
}

pub struct KeypadReport {
    pub mouse_buttons: u8,
    pub wheel: i8,
    pub joy_buttons: u8,
    pub x: u16,
    pub y: u16,
    pub modifier: u8,
    pub keycodes: [u8; 26],
    pub consumer_keycode: u16,
}

static PIN_CONFIG: [Option<iomuxc::PullKeeper>; 31] = [
    Some(iomuxc::PullKeeper::Pulldown100k), // 0
    Some(iomuxc::PullKeeper::Pulldown100k), // 1
    Some(iomuxc::PullKeeper::Pulldown100k), // 2
    Some(iomuxc::PullKeeper::Pulldown100k), // 3
    Some(iomuxc::PullKeeper::Pulldown100k), // 4
    Some(iomuxc::PullKeeper::Pulldown100k), // 5
    Some(iomuxc::PullKeeper::Pulldown100k), // 6
    Some(iomuxc::PullKeeper::Pulldown100k), // 7
    Some(iomuxc::PullKeeper::Pulldown100k), // 8
    Some(iomuxc::PullKeeper::Pulldown100k), // 9
    Some(iomuxc::PullKeeper::Pulldown100k), // 10
    Some(iomuxc::PullKeeper::Pulldown100k), // 11
    Some(iomuxc::PullKeeper::Pulldown100k), // 12
    Some(iomuxc::PullKeeper::Pulldown100k), // 13
    None,                                   // 14
    None,                                   // 15
    Some(iomuxc::PullKeeper::Pulldown100k), // 16
    Some(iomuxc::PullKeeper::Pulldown100k), // 17
    Some(iomuxc::PullKeeper::Pullup100k),   // 18
    Some(iomuxc::PullKeeper::Pulldown100k), // 19
    Some(iomuxc::PullKeeper::Pulldown100k), // 20
    Some(iomuxc::PullKeeper::Pulldown100k), // 21
    Some(iomuxc::PullKeeper::Pulldown100k), // 22
    Some(iomuxc::PullKeeper::Pulldown100k), // 23
    Some(iomuxc::PullKeeper::Pulldown100k), // 24
    Some(iomuxc::PullKeeper::Pulldown100k), // 25
    Some(iomuxc::PullKeeper::Pulldown100k), // 26
    Some(iomuxc::PullKeeper::Pulldown100k), // 27
    Some(iomuxc::PullKeeper::Pulldown100k), // 28
    Some(iomuxc::PullKeeper::Pulldown100k), // 29
    Some(iomuxc::PullKeeper::Pulldown100k), // 30
];
static EP_MEMORY: imxrt_usbd::EndpointMemory<4096> = imxrt_usbd::EndpointMemory::new();
static EP_STATE: imxrt_usbd::EndpointState = imxrt_usbd::EndpointState::max_endpoints();

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{:?}", info);
    teensy4_panic::sos();
}

macro_rules! configure_pin {
    ($pin_index: tt, $pins: ident) => {
        ::paste::paste! {
            bsp::hal::iomuxc::configure(
                &mut $pins.[<p $pin_index>],
                bsp::hal::iomuxc::Config::zero()
                    .set_hysteresis(bsp::hal::iomuxc::Hysteresis::Enabled)
                    .set_pull_keeper(PIN_CONFIG[$pin_index])
            )
        }
    };
}

#[bsp::rt::entry]
fn main() -> ! {
    // These are peripheral instances. Let the board configure these for us.
    // This function can only be called once!
    let instances = board::instances();

    // Driver resources that are configured by the board. For more information,
    // see the `board` documentation.
    let board::Resources {
        pit,
        mut pins,
        mut adc1,
        usb,
        mut gpio1,
        mut gpio2,
        mut gpio3,
        mut gpio4,
        ..
    } = board::t41(instances);

    // Set up pullup/pulldown
    configure_pin!(0, pins);
    configure_pin!(1, pins);
    configure_pin!(2, pins);
    configure_pin!(3, pins);
    configure_pin!(4, pins);
    configure_pin!(5, pins);
    configure_pin!(6, pins);
    configure_pin!(7, pins);
    configure_pin!(8, pins);
    configure_pin!(9, pins);
    configure_pin!(10, pins);
    configure_pin!(11, pins);
    configure_pin!(12, pins);
    configure_pin!(13, pins);
    configure_pin!(14, pins);
    configure_pin!(15, pins);
    configure_pin!(16, pins);
    configure_pin!(17, pins);
    configure_pin!(18, pins);
    configure_pin!(19, pins);
    configure_pin!(20, pins);
    configure_pin!(21, pins);
    configure_pin!(22, pins);
    configure_pin!(23, pins);
    configure_pin!(24, pins);
    configure_pin!(25, pins);
    configure_pin!(26, pins);
    configure_pin!(27, pins);
    configure_pin!(28, pins);
    configure_pin!(29, pins);
    configure_pin!(30, pins);

    // Set up ADC
    adc1.set_resolution(ResolutionBits::Res10);
    adc1.calibrate();

    // Configure IO for keymap use
    let mut keymap_io =
        KeymapIOPoints::new(&mut gpio1, &mut gpio2, &mut gpio3, &mut gpio4, pins, pit.1);

    // set up keymap TODO: load from flash (teensy4-fcb?)
    let mut keymap = Keymap::default();
    let mut keymap_state = KeymapState::default();

    // Polling
    let mut delay = bsp::hal::timer::Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);

    // set up USB HID device
    let bus_adapter = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, imxrt_usbd::Speed::High);
    let usb_alloc = UsbBusAllocator::new(bus_adapter);
    let hid_settings = HidClassSettings {
        config: ProtocolModeConfig::ForceReport,
        ..HidClassSettings::default()
    };
    let mut mouse_class = HIDClass::new_ep_in(&usb_alloc, MouseReport::desc(), 10);
    let mut joystick_class = HIDClass::new_ep_in(&usb_alloc, JoystickReport::desc(), 10);
    let mut keyboard_class =
        HIDClass::new_ep_in_with_settings(&usb_alloc, KeyboardReport::desc(), 10, hid_settings);
    let mut consumer_class = HIDClass::new_ep_in(&usb_alloc, ConsumerReport::desc(), 10);
    //let mut serial_port = SerialPort::new(&usb_alloc);
    let mut keypad_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0x1209, 0x0001)) // TODO: fork pid.codes accordingly; finish stuff first
        .manufacturer("kitknacks")
        .product("padtarust keypad")
        .device_class(0x03)
        .serial_number("00000")
        .max_power(500)
        .composite_with_iads()
        .max_packet_size_0(64)
        .build();

    loop {
        if !keypad_dev.poll(&mut [
            &mut mouse_class,
            &mut joystick_class,
            &mut keyboard_class,
            &mut consumer_class,
        ]) {
            continue;
        }
        let state = keypad_dev.state();
        if state == usb_device::device::UsbDeviceState::Configured {
            break;
        }
    }
    keypad_dev.bus().configure();

    let mut report_written = false;
    let mut report = keymap_state.update(&mut adc1, &mut keymap_io, &mut keymap);
    let mut mouse_written = false;
    let mut joystick_written = false;
    let mut keyboard_written = false;
    let mut consumer_written = false;

    loop {
        if report_written {
            report = keymap_state.update(&mut adc1, &mut keymap_io, &mut keymap);
            report_written = false;
            mouse_written = false;
            joystick_written = false;
            keyboard_written = false;
            consumer_written = false;
        }

        if !mouse_written {
            let mouse_report = MouseReport {
                mouse_buttons: report.mouse_buttons,
                wheel: report.wheel,
            };
            if let Err(e) = mouse_class.push_input(&mouse_report) {
                match e {
                    UsbError::WouldBlock => {
                        mouse_written = false;
                    }
                    UsbError::BufferOverflow => {
                        mouse_written = false;
                    }
                    _ => {
                        panic!("Failed to write mouse report: {:?}", e);
                    }
                }
            } else {
                mouse_written = true;
            }
        }

        if !joystick_written {
            let joystick_report = JoystickReport {
                joy_buttons: report.joy_buttons,
                x: report.x,
                y: report.y,
            };
            if let Err(e) = joystick_class.push_input(&joystick_report) {
                match e {
                    UsbError::WouldBlock => {
                        joystick_written = false;
                    }
                    UsbError::BufferOverflow => {
                        joystick_written = false;
                    }
                    _ => {
                        panic!("Failed to write joystick report: {:?}", e);
                    }
                }
            } else {
                joystick_written = true;
            }
        }

        if !keyboard_written {
            let keyboard_report = KeyboardReport {
                modifier: report.modifier,
                keycodes: report.keycodes,
            };
            if let Err(e) = keyboard_class.push_input(&keyboard_report) {
                match e {
                    UsbError::WouldBlock => {
                        keyboard_written = false;
                    }
                    UsbError::BufferOverflow => {
                        keyboard_written = false;
                    }
                    _ => {
                        panic!("Failed to write keyboard report: {:?}", e);
                    }
                }
            } else {
                keyboard_written = true;
            }
        }

        if !consumer_written {
            let consumer_report = ConsumerReport {
                consumer_keycode: report.consumer_keycode,
            };
            if let Err(e) = consumer_class.push_input(&consumer_report) {
                match e {
                    UsbError::WouldBlock => {
                        consumer_written = false;
                    }
                    UsbError::BufferOverflow => {
                        consumer_written = false;
                    }
                    _ => {
                        panic!("Failed to write consumer report: {:?}", e);
                    }
                }
            } else {
                consumer_written = true;
            }
        }

        if mouse_written && joystick_written && keyboard_written && consumer_written {
            report_written = true;
        }

        if !keypad_dev.poll(&mut [
            &mut mouse_class,
            &mut joystick_class,
            &mut keyboard_class,
            &mut consumer_class,
        ]) {
            continue;
        }

        delay.block_ms(10);
    }
}
