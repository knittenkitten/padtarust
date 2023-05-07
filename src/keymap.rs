use crate::ws2812::WS2812;
use crate::KeypadReport;
use teensy4_bsp::hal;
use teensy4_bsp::pins::t41::Pins;

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

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Keyboard {
    NoEventIndicated = 0x00,
    RolloverError = 0x01,
    POSTFail = 0x02,
    ErrorUndefined = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    Number1 = 0x1E,
    Number2 = 0x1F,
    Number3 = 0x20,
    Number4 = 0x21,
    Number5 = 0x22,
    Number6 = 0x23,
    Number7 = 0x24,
    Number8 = 0x25,
    Number9 = 0x26,
    Number0 = 0x27,
    Return = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Plus = 0x2E,
    LeftBracket = 0x2F,
    RightBracket = 0x30,
    Backslash = 0x31,
    Intl = 0x32,
    Semicolon = 0x33,
    Quote = 0x34,
    GraveTilde = 0x35,
    Comma = 0x36,
    Period = 0x37,
    ForwardSlash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,
    KeypadNumLockAndClear = 0x53,
    KeypadDivide = 0x54,
    KeypadMultiply = 0x55,
    KeypadSubtract = 0x56,
    KeypadAdd = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadDot = 0x63,
    Intl2 = 0x64,
    Application = 0x65,
    Power = 0x66,
    KeypadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Execute = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LockingCapsLock = 0x82,
    LockingNumLock = 0x83,
    LockingScrollLock = 0x84,
    KeypadComma = 0x85,
    KeypadEqualsSign = 0x86,
    Ro = 0x87,
    Kana = 0x88,
    Yen = 0x89,
    Henkan = 0x8A,
    Muhenkan = 0x8B,
    Int6 = 0x8C,
    Int7 = 0x8D,
    Int8 = 0x8E,
    Int9 = 0x8F,
    Lang1 = 0x90,
    Lang2 = 0x91,
    Lang3 = 0x92,
    Lang4 = 0x93,
    Lang5 = 0x94,
    Lang6 = 0x95,
    Lang7 = 0x96,
    Lang8 = 0x97,
    Lang9 = 0x98,
    AltErase = 0x99,
    SysReq = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return2 = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    ClSelProps = 0xA3,
    ExSel = 0xA4,
    Keypad00 = 0xB0,
    Keypad000 = 0xB1,
    ThousandsSep = 0xB2,
    DecimalSep = 0xB3,
    CurrencyUnit = 0xB4,
    CurrencySubUnit = 0xB5,
    KeypadLeftParen = 0xB6,
    KeypadRightParen = 0xB7,
    KeypadLeftBracket = 0xB8,
    KeypadRightBracket = 0xB9,
    KeypadTab = 0xBA,
    KeypadBackspace = 0xBB,
    KeypadA = 0xBC,
    KeypadB = 0xBD,
    KeypadC = 0xBE,
    KeypadD = 0xBF,
    KeypadE = 0xC0,
    KeypadF = 0xC1,
    KeypadXOR = 0xC2,
    KeypadCaret = 0xC3,
    KeypadPercent = 0xC4,
    KeypadLeftAngleBracket = 0xC5,
    KeypadRightAngleBracket = 0xC6,
    KeypadBitwiseAnd = 0xC7,
    KeypadAnd = 0xC8,
    KeypadBitwiseOr = 0xC9,
    KeypadOr = 0xCA,
    KeypadColon = 0xCB,
    KeypadPound = 0xCC,
    KeypadSpace = 0xCD,
    KeypadAt = 0xCE,
    KeypadExclamation = 0xCF,
    KeypadMemStore = 0xD0,
    KeypadMemRecall = 0xD1,
    KeypadMemClear = 0xD2,
    KeypadMemAdd = 0xD3,
    KeypadMemSubstract = 0xD4,
    KeypadMemMultiply = 0xD5,
    KeypadMemDivide = 0xD6,
    KeypadSign = 0xD7,
    KeypadClear = 0xD8,
    KeypadClearEntry = 0xD9,
    KeypadBinary = 0xDA,
    KeypadOctal = 0xDB,
    KeypadDecimal = 0xDC,
    KeypadHexadecimal = 0xDD,
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

// we have lots of RAM... why not...
#[repr(u16)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Consumer {
    Unassigned = 0x00,
    Plus10 = 0x20,
    Plus100 = 0x21,
    AmPm = 0x22,
    Power = 0x30,
    Reset = 0x31,
    Sleep = 0x32,
    SleepAfter = 0x33,
    SleepMode = 0x34,
    Illumination = 0x35,
    Menu = 0x40,
    MenuPick = 0x41,
    MenuUp = 0x42,
    MenuDown = 0x43,
    MenuLeft = 0x44,
    MenuRight = 0x45,
    MenuEscape = 0x46,
    MenuValueIncrease = 0x47,
    MenuValueDecrease = 0x48,
    DataOnScreen = 0x60,
    ClosedCaption = 0x61,
    ClosedCaptionSelect = 0x62,
    VcrTv = 0x63,
    BroadcastMode = 0x64,
    Snapshot = 0x65,
    Still = 0x66,
    Selection = 0x80,
    AssignSelection = 0x81,
    ModeStep = 0x82,
    RecallLast = 0x83,
    EnterChannel = 0x84,
    OrderMovie = 0x85,
    Channel = 0x86,
    MediaSelection = 0x87,
    MediaSelectComputer = 0x88,
    MediaSelectTV = 0x89,
    MediaSelectWWW = 0x8A,
    MediaSelectDVD = 0x8B,
    MediaSelectTelephone = 0x8C,
    MediaSelectProgramGuide = 0x8D,
    MediaSelectVideoPhone = 0x8E,
    MediaSelectGames = 0x8F,
    MediaSelectMessages = 0x90,
    MediaSelectCD = 0x91,
    MediaSelectVCR = 0x92,
    MediaSelectTuner = 0x93,
    Quit = 0x94,
    Help = 0x95,
    MediaSelectTape = 0x96,
    MediaSelectCable = 0x97,
    MediaSelectSatellite = 0x98,
    MediaSelectSecurity = 0x99,
    MediaSelectHome = 0x9A,
    MediaSelectCall = 0x9B,
    ChannelIncrement = 0x9C,
    ChannelDecrement = 0x9D,
    MediaSelectSAP = 0x9E,
    VCRPlus = 0xA0,
    Once = 0xA1,
    Daily = 0xA2,
    Weekly = 0xA3,
    Monthly = 0xA4,
    Play = 0xB0,
    Pause = 0xB1,
    Record = 0xB2,
    FastForward = 0xB3,
    Rewind = 0xB4,
    ScanNextTrack = 0xB5,
    ScanPreviousTrack = 0xB6,
    Stop = 0xB7,
    Eject = 0xB8,
    RandomPlay = 0xB9,
    SelectDisc = 0xBA,
    EnterDisc = 0xBB,
    Repeat = 0xBC,
    Tracking = 0xBD,
    TrackNormal = 0xBE,
    SlowTracking = 0xBF,
    FrameForward = 0xC0,
    FrameBack = 0xC1,
    Mark = 0xC2,
    ClearMark = 0xC3,
    RepeatFromMark = 0xC4,
    ReturnToMark = 0xC5,
    SearchMarkForward = 0xC6,
    SearchMarkBackwards = 0xC7,
    CounterReset = 0xC8,
    ShowCounter = 0xC9,
    TrackingIncrement = 0xCA,
    TrackingDecrement = 0xCB,
    StopEject = 0xCC,
    PlayPause = 0xCD,
    PlaySkip = 0xCE,
    Volume = 0xE0,
    Balance = 0xE1,
    Mute = 0xE2,
    Bass = 0xE3,
    Treble = 0xE4,
    BassBoost = 0xE5,
    SurroundMode = 0xE6,
    Loudness = 0xE7,
    MPX = 0xE8,
    VolumeIncrement = 0xE9,
    VolumeDecrement = 0xEA,
    SpeedSelect = 0xF0,
    PlaybackSpeed = 0xF1,
    StandardPlay = 0xF2,
    LongPlay = 0xF3,
    ExtendedPlay = 0xF4,
    Slow = 0xF5,
    FanEnable = 0x100,
    FanSpeed = 0x101,
    LightEnable = 0x102,
    LightIlluminationLevel = 0x103,
    ClimateControlEnable = 0x104,
    RoomTemperature = 0x105,
    SecurityEnable = 0x106,
    FireAlarm = 0x107,
    PoliceAlarm = 0x108,
    Proximity = 0x109,
    Motion = 0x10A,
    DuressAlarm = 0x10B,
    HoldupAlarm = 0x10C,
    MedicalAlarm = 0x10D,
    BalanceRight = 0x150,
    BalanceLeft = 0x151,
    BassIncrement = 0x152,
    BassDecrement = 0x153,
    TrebleIncrement = 0x154,
    TrebleDecrement = 0x155,
    SubChannel = 0x170,
    SubChannelIncrement = 0x171,
    SubChannelDecrement = 0x172,
    AlternateAudioIncrement = 0x173,
    AlternateAudioDecrement = 0x174,
    ALLaunchButtonConfigurationTool = 0x181,
    ALProgrammableButtonConfiguration = 0x182,
    ALConsumerControlConfiguration = 0x183,
    ALWordProcessor = 0x184,
    ALTextEditor = 0x185,
    ALSpreadsheet = 0x186,
    ALGraphicsEditor = 0x187,
    ALPresentationApp = 0x188,
    ALDatabaseApp = 0x189,
    ALEmailReader = 0x18A,
    ALNewsreader = 0x18B,
    ALVoicemail = 0x18C,
    ALContactsAddressBook = 0x18D,
    ALCalendarSchedule = 0x18E,
    ALTaskProjectManager = 0x18F,
    ALLogJournalTimecard = 0x190,
    ALCheckbookFinance = 0x191,
    ALCalculator = 0x192,
    ALAvCapturePlayback = 0x193,
    ALLocalMachineBrowser = 0x194,
    ALLanWanBrowser = 0x195,
    ALInternetBrowser = 0x196,
    ALRemoteNetworkingISPConnect = 0x197,
    ALNetworkConference = 0x198,
    ALNetworkChat = 0x199,
    ALTelephonyDialer = 0x19A,
    ALLogon = 0x19B,
    ALLogoff = 0x19C,
    ALLogonLogoff = 0x19D,
    ALTerminalLockScreensaver = 0x19E,
    ALControlPanel = 0x19F,
    ALCommandLineProcessorRun = 0x1A0,
    ALProcessTaskManager = 0x1A1,
    ALSelectTaskApplication = 0x1A2,
    ALNextTaskApplication = 0x1A3,
    ALPreviousTaskApplication = 0x1A4,
    ALPreemptiveHaltTaskApplication = 0x1A5,
    ALIntegratedHelpCenter = 0x1A6,
    ALDocuments = 0x1A7,
    ALThesaurus = 0x1A8,
    ALDictionary = 0x1A9,
    ALDesktop = 0x1AA,
    ALSpellCheck = 0x1AB,
    ALGrammarCheck = 0x1AC,
    ALWirelessStatus = 0x1AD,
    ALKeyboardLayout = 0x1AE,
    ALVirusProtection = 0x1AF,
    ALEncryption = 0x1B0,
    ALScreenSaver = 0x1B1,
    ALAlarms = 0x1B2,
    ALClock = 0x1B3,
    ALFileBrowser = 0x1B4,
    ALPowerStatus = 0x1B5,
    ALImageBrowser = 0x1B6,
    ALAudioBrowser = 0x1B7,
    ALMovieBrowser = 0x1B8,
    ALDigitalRightsManager = 0x1B9,
    ALDigitalWallet = 0x1BA,
    ALInstantMessaging = 0x1BC,
    ALOemFeaturesTipsTutorialBrowser = 0x1BD,
    ALOemHelp = 0x1BE,
    ALOnlineCommunity = 0x1BF,
    ALEntertainmentContentBrowser = 0x1C0,
    ALOnlineShoppingBrowser = 0x1C1,
    ALSmartCardInformationHelp = 0x1C2,
    ALMarketMonitorFinanceBrowser = 0x1C3,
    ALCustomizedCorporateNewsBrowser = 0x1C4,
    ALOnlineActivityBrowser = 0x1C5,
    ALResearchSearchBrowser = 0x1C6,
    ALAudioPlayer = 0x1C7,
    ACNew = 0x201,
    ACOpen = 0x202,
    ACClose = 0x203,
    ACExit = 0x204,
    ACMaximize = 0x205,
    ACMinimize = 0x206,
    ACSave = 0x207,
    ACPrint = 0x208,
    ACProperties = 0x209,
    ACUndo = 0x21A,
    ACCopy = 0x21B,
    ACCut = 0x21C,
    ACPaste = 0x21D,
    ACSelectAll = 0x21E,
    ACFind = 0x21F,
    ACFindAndReplace = 0x220,
    ACSearch = 0x221,
    ACGoTo = 0x222,
    ACHome = 0x223,
    ACBack = 0x224,
    ACForward = 0x225,
    ACStop = 0x226,
    ACRefresh = 0x227,
    ACPreviousLink = 0x228,
    ACNextLink = 0x229,
    ACBookmarks = 0x22A,
    ACHistory = 0x22B,
    ACSubscriptions = 0x22C,
    ACZoomIn = 0x22D,
    ACZoomOut = 0x22E,
    ACZoom = 0x22F,
    ACFullScreenView = 0x230,
    ACNormalView = 0x231,
    ACViewToggle = 0x232,
    ACScrollUp = 0x233,
    ACScrollDown = 0x234,
    ACScroll = 0x235,
    ACPanLeft = 0x236,
    ACPanRight = 0x237,
    ACPan = 0x238,
    ACNewWindow = 0x239,
    ACTileHorizontally = 0x23A,
    ACTileVertically = 0x23B,
    ACFormat = 0x23C,
    ACEdit = 0x23D,
    ACBold = 0x23E,
    ACItalics = 0x23F,
    ACUnderline = 0x240,
    ACStrikethrough = 0x241,
    ACSubscript = 0x242,
    ACSuperscript = 0x243,
    ACAllCaps = 0x244,
    ACRotate = 0x245,
    ACResize = 0x246,
    ACFlipHorizontal = 0x247,
    ACFlipVertical = 0x248,
    ACMirrorHorizontal = 0x249,
    ACMirrorVertical = 0x24A,
    ACFontSelect = 0x24B,
    ACFontColor = 0x24C,
    ACFontSize = 0x24D,
    ACJustifyLeft = 0x24E,
    ACJustifyCenterH = 0x24F,
    ACJustifyRight = 0x250,
    ACJustifyBlockH = 0x251,
    ACJustifyTop = 0x252,
    ACJustifyCenterV = 0x253,
    ACJustifyBottom = 0x254,
    ACJustifyBlockV = 0x255,
    ACIndentDecrease = 0x256,
    ACIndentIncrease = 0x257,
    ACNumberedList = 0x258,
    ACRestartNumbering = 0x259,
    ACBulletedList = 0x25A,
    ACPromote = 0x25B,
    ACDemote = 0x25C,
    ACYes = 0x25D,
    ACNo = 0x25E,
    ACCancel = 0x25F,
    ACCatalog = 0x260,
    ACBuyCheckout = 0x261,
    ACAddToCart = 0x262,
    ACExpand = 0x263,
    ACExpandAll = 0x264,
    ACCollapse = 0x265,
    ACCollapseAll = 0x266,
    ACPrintPreview = 0x267,
    ACPasteSpecial = 0x268,
    ACInsertMode = 0x269,
    ACDelete = 0x26A,
    ACLock = 0x26B,
    ACUnlock = 0x26C,
    ACProtect = 0x26D,
    ACUnprotect = 0x26E,
    ACAttachComment = 0x26F,
    ACDeleteComment = 0x270,
    ACViewComment = 0x271,
    ACSelectWord = 0x272,
    ACSelectSentence = 0x273,
    ACSelectParagraph = 0x274,
    ACSelectColumn = 0x275,
    ACSelectRow = 0x276,
    ACSelectTable = 0x277,
    ACSelectObject = 0x278,
    ACRedoRepeat = 0x279,
    ACSort = 0x27A,
    ACSortAscending = 0x27B,
    ACSortDescending = 0x27C,
    ACFilter = 0x27D,
    ACSetClock = 0x27E,
    ACViewClock = 0x27F,
    ACSelectTimeZone = 0x280,
    ACEditTimeZones = 0x281,
    ACSetAlarm = 0x282,
    ACClearAlarm = 0x283,
    ACSnoozeAlarm = 0x284,
    ACResetAlarm = 0x285,
    ACSynchronize = 0x286,
    ACSendReceive = 0x287,
    ACSendTo = 0x288,
    ACReply = 0x289,
    ACReplyAll = 0x28A,
    ACForwardMsg = 0x28B,
    ACSend = 0x28C,
    ACAttachFile = 0x28D,
    ACUpload = 0x28E,
    ACDownloadSaveTargetAs = 0x28F,
    ACSetBorders = 0x290,
    ACInsertRow = 0x291,
    ACInsertColumn = 0x292,
    ACInsertFile = 0x293,
    ACInsertPicture = 0x294,
    ACInsertObject = 0x295,
    ACInsertSymbol = 0x296,
    ACSaveAndClose = 0x297,
    ACRename = 0x298,
    ACMerge = 0x299,
    ACSplit = 0x29A,
    ACDistributeHorizontally = 0x29B,
    ACDistributeVertically = 0x29C,
}

pub struct Report {
    buttons: [Option<Keyboard>; 26],
    button_count: usize,
    mouse_buttons: [Option<bool>; 3],
    joystick_button: Option<bool>,
    consumer_code: Consumer,
}

impl Report {
    fn new() -> Report {
        Report {
            buttons: [None; 26],
            button_count: 0,
            mouse_buttons: [None; 3],
            joystick_button: None,
            consumer_code: Consumer::Unassigned,
        }
    }
    fn add_mapping(&mut self, mapping: Mapping) {
        if mapping.button > Keyboard::ErrorUndefined {
            if self.button_count < 26 {
                self.buttons[self.button_count] = Some(mapping.button);
                self.button_count += 1;
            } else {
                log::warn!("more than 26 keypresses registered in one frame");
            }
        }
        let consumer = mapping.consumer_button;
        if consumer != Consumer::Unassigned {
            if self.consumer_code != Consumer::Unassigned {
                log::warn!("more than 1 consumer keypress registered in one frame");
            }
            self.consumer_code = consumer;
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
    fn finalize(&self) -> KeypadReport {
        let mut report = KeypadReport {
            mouse_buttons: 0,
            wheel: 0,
            joy_buttons: 0,
            x: 0,
            y: 0,
            modifier: 0,
            keycodes: [0; 26],
            consumer_keycode: 0,
        };
        let mut k: usize = 0;
        let mut nkro_keys = [0u8; 26];
        for (i, button) in self.buttons.into_iter().enumerate() {
            if let Some(b) = button {
                nkro_keys[k] = b as u8;
                k += i;
            } else {
                break;
            }
        }
        report.keycodes = nkro_keys;
        if self.joystick_button.is_some() {
            report.joy_buttons = 1;
        }
        if self.mouse_buttons[0].is_some() {
            if self.mouse_buttons[1].is_some() {
                if self.mouse_buttons[2].is_some() {
                    report.mouse_buttons = 7;
                } else {
                    report.mouse_buttons = 3;
                }
            } else {
                report.mouse_buttons = 1;
            }
        } else if self.mouse_buttons[1].is_some() {
            if self.mouse_buttons[2].is_some() {
                report.mouse_buttons = 6;
            } else {
                report.mouse_buttons = 2;
            }
        } else if self.mouse_buttons[2].is_some() {
            report.mouse_buttons = 4;
        }
        report.consumer_keycode = self.consumer_code as u16;
        report
    }
}

#[derive(Copy, Clone)]
pub struct Mapping {
    action: KeyboardAction,
    button: Keyboard,
    consumer_button: Consumer,
}

impl Mapping {
    fn from_button(b: Keyboard) -> Mapping {
        Mapping {
            action: KeyboardAction::None,
            button: b,
            consumer_button: Consumer::Unassigned,
        }
    }
    fn from_action(a: KeyboardAction) -> Mapping {
        Mapping {
            action: a,
            button: Keyboard::NoEventIndicated,
            consumer_button: Consumer::Unassigned,
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
    ) -> KeypadReport {
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

        // generate joystick report
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
            let mut layer_change = false;
            let mut wasd_change = false;
            for op in keyboard_operations.iter().take(keyboard_op_count) {
                match op {
                    KeyboardAction::Layer0Momentary => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 0;
                        layer_change = true;
                    }
                    KeyboardAction::Layer1Momentary => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 1;
                        layer_change = true;
                    }
                    KeyboardAction::Layer2Momentary => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 2;
                        layer_change = true;
                    }
                    KeyboardAction::Layer3Momentary => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 3;
                        layer_change = true;
                    }
                    KeyboardAction::Layer0Set => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 0;
                        state.stored_layer = 0;
                        layer_change = true;
                    }
                    KeyboardAction::Layer1Set => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 1;
                        state.stored_layer = 1;
                        layer_change = true;
                    }
                    KeyboardAction::Layer2Set => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 2;
                        state.stored_layer = 2;
                        layer_change = true;
                    }
                    KeyboardAction::Layer3Set => {
                        if layer_change {
                            log::warn!("more than one layer operation detected in one frame");
                        }
                        state.current_layer = 3;
                        state.stored_layer = 3;
                        layer_change = true;
                    }
                    KeyboardAction::WasdModeOff => {
                        if wasd_change {
                            log::warn!("more than one WASD mode operation detected in one frame");
                        }
                        state.wasd_mode = false;
                        wasd_change = true;
                    }
                    KeyboardAction::WasdModeOn => {
                        if wasd_change {
                            log::warn!("more than one WASD mode operation detected in one frame");
                        }
                        state.wasd_mode = true;
                        wasd_change = true;
                    }
                    KeyboardAction::WasdModeToggle => {
                        if wasd_change {
                            log::warn!("more than one WASD mode operation detected in one frame");
                        }
                        state.wasd_mode = !state.wasd_mode;
                        wasd_change = true;
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
        let mut usb_report = report.finalize();
        if !state.wasd_mode {
            // write joystick x and y to report
            usb_report.x = unsafe { joy_x_f.to_int_unchecked::<u16>() };
            usb_report.y = unsafe { joy_y_f.to_int_unchecked::<u16>() };
        }
        // generate mouse wheel report
        // TODO: allow the scroll wheel to do something else
        if rotary_1_state != state.rotary_1_prev {
            if io.rotary2.is_set() != rotary_1_state {
                // negative
                usb_report.wheel = -1;
            } else {
                // positive
                usb_report.wheel = 1;
            }
        }
        usb_report
    }
}
