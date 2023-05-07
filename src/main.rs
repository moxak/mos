#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // すべてのRustレベルのエントリポイントを無効にする
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

// panic_handler attribute はパニックが発生したときにコンパイラが呼び出す関数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // PanicInfo: パニックが発生したファイルと行、およびオプションでパニックメッセージ
    println!("{}", info);
    loop {}
}

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");  

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("tirivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
