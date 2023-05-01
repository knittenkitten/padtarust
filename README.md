padtarust
=========

A gaming keypad (and custom Rust firmware thereof) that might just happen to fit in the plastic case of the Razer Tartarus v2.  Supports an analog joystick, 4 software layers, and toggleable WASD encoding of the joystick.

Loosely based on [PyPad](https://github.com/Ayehavgunne/pypad).

(PCB, plate design, and customization in progress)

Build instructions
==================

        cargo objcopy --release -- -O ihex padtarust.hex

Will not compile successfully in debug mode due to the amount of libraries.  I might fix this someday, but also... I'm not going to be running this in debug mode any time soon, and I doubt anybody else is.