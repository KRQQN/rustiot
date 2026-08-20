The following pins are used to bootstrap the chip. They are available
for use, but check the datasheet of the module for more information on them.
GPIO0
GPIO3
GPIO45
GPIO46

These GPIO pins are in use by some feature of the module and should not be used.
let _ = peripherals.GPIO26;
let _ = peripherals.GPIO27;
let _ = peripherals.GPIO28;
let _ = peripherals.GPIO29;
let _ = peripherals.GPIO30;
let _ = peripherals.GPIO31;
let _ = peripherals.GPIO32;
