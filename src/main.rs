#![no_std]
#![no_main]

mod keymap;
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
    prelude::{UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;
//use usbd_serial::SerialPort;

use crate::keymap::{Keymap, KeymapIOPoints, KeymapState};

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage = WHEEL,) = {
                #[item_settings data,variable,relative] wheel=input;
            };
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] mouse_buttons=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = JOYSTICK) = {
        (collection = PHYSICAL, usage = JOYSTICK) = {
            (usage = X,) = {
                #[item_settings data,variable,absolute] x=input;
            };
            (usage = Y,) = {
                #[item_settings data,variable,absolute] y=input;
            };
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] joy_buttons=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYPAD) = {
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        }
    },
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x514) = {
            #[item_settings data,array,absolute,not_null] consumer_keycode=input;
        }
    }
)]
struct KeypadReport{
    pub mouse_buttons: u8,
    pub wheel: i8,
    pub joy_buttons: u8,
    pub x: u16,
    pub y: u16,
    pub modifier: u8,
    pub keycodes: [u8; 26],
    pub consumer_keycode: u16
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

    // set up USB HID device
    let bus_adapter = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, imxrt_usbd::Speed::High);
    let usb_alloc = UsbBusAllocator::new(bus_adapter);
    let mut keypad_class = HIDClass::new(&usb_alloc, KeypadReport::desc(), 10);
    //let mut serial_port = SerialPort::new(&usb_alloc);
    let mut keypad_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0x1209, 0x0001)) // TODO: fork pid.codes accordingly; finish stuff first
        .manufacturer("kitknacks")
        .product("padtarust keypad")
        .device_class(0x03)
        .serial_number("00000")
        .max_power(500)
        .max_packet_size_0(64)
        .build();
    loop {
        if !keypad_dev.poll(&mut [&mut keypad_class]){//, &mut serial_port]) {
            continue;
        }
        let state = keypad_dev.state();
        if state == usb_device::device::UsbDeviceState::Configured {
            break;
        }
    }
    keypad_dev.bus().configure();

    // set up keymap TODO: load from flash (teensy4-fcb?)
    let mut keymap = Keymap::default();
    let mut keymap_state = KeymapState::default();

    // Polling
    let mut delay = bsp::hal::timer::Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);

    loop {
        let report = keymap.update(&mut adc1, &mut keymap_io, &mut keymap_state);
        if let Err(e) = keypad_class.push_input(&report){
            match e{
                UsbError::WouldBlock => {}
                _ => {
                    panic!("Failed to write keyboard report: {:?}", e);
                }
            }
        }

        let _ = keypad_dev.poll(&mut [&mut keypad_class]); //, &mut serial_port]);

        delay.block_ms(10);
    }
}
