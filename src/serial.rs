use core::fmt;
use spin::Lazy;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_PORT_ADDRESS: u16 = 0x3F8;

#[allow(dead_code)]
pub static SERIAL_PORT: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(SERIAL_PORT_ADDRESS) };
    serial_port.init();
    Mutex::new(serial_port)
});

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {{
        $crate::serial::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the Serial Port through the global `SERIAL_PORT` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_PORT.lock().write_fmt(args).unwrap();
}
