// #![no_std]
// #![no_main]

// mod writer;

// use bootloader_api::config::Mapping;
// use writer::FrameBufferWriter;
// use x86_64::instructions::hlt;

// //Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
// //optionally pass a custom config
// pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
//     let mut config = bootloader_api::BootloaderConfig::new_default();
//     config.mappings.physical_memory = Some(Mapping::Dynamic);
//     config.kernel_stack_size = 100 * 1024; // 100 KiB
//     config
// };
// bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

// fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

//     let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

//     let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

//     let mut frame_buffer_writer = 
//         FrameBufferWriter::new(buffer, frame_buffer_info);

//     use core::fmt::Write;//below requires this
//     writeln!(frame_buffer_writer, "Testing testing {} and {}", 1, 4.0/2.0).unwrap();

//     loop {
//         hlt(); //stop x86_64 from being unnecessarily busy while looping
//     }
// }

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {
//         hlt();
//     }
// }



#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named start by default regardless of your host OS.
    println!("Hello World!");
    panic!("Some panic message");
}

