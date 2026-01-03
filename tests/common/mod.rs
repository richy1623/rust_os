#[macro_export]
macro_rules! should_panic_test {
    ($test_fn:expr) => {
        // Use unsafe() to wrap attributes that can break safety invariants
        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            rust_os::serial_print!("{}...\t", core::any::type_name_of_val(&$test_fn));
            rust_os::init();

            $test_fn();

            rust_os::serial_println!("[test did not panic]");
            rust_os::qemu_exit::exit_qemu(rust_os::qemu_exit::QemuExitCode::Failed);
            loop {}
        }

        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            rust_os::serial_println!("[ok]");
            rust_os::qemu_exit::exit_qemu(rust_os::qemu_exit::QemuExitCode::Success);
            loop {}
        }
    };
}
