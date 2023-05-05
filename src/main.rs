#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // すべてのRustレベルのエントリポイントを無効にする

use core::panic::PanicInfo;

// panic_handler attribute はパニックが発生したときにコンパイラが呼び出す関数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // PanicInfo: パニックが発生したファイルと行、およびオプションでパニックメッセージ
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで、`_start`という名前の関数を探すので
    // この関数がエントリポイントとなる
    
    let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}


