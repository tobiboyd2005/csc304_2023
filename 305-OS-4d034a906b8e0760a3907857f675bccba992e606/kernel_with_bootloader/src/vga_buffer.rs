use core::fmt;
mod writer;
extern crate spin;


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
use spin::Mutex; // Import the spin crate for synchronization

// Define a static WRITER variable as a Mutex
pub static WRITER: spin::Mutex<core::fmt::Write> = spin::Mutex::new(some_writer_instance);
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
   //  #[warn(unused_imports)]
   //  use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
