#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // すべてのRustレベルのエントリポイントを無効にする
#![feature(custom_test_frameworks)]
#![test_runner(mos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");  

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mos::test_panic_handler(info);
}
