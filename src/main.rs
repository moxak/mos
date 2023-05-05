#![no_std] // Rustの標準ライブラリにリンクしない
#![no_main] // すべてのRustレベルのエントリポイントを無効にする

use core::panic::PanicInfo;

// panic_handler attribute はパニックが発生したときにコンパイラが呼び出す関数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // PanicInfo: パニックが発生したファイルと行、およびオプションでパニックメッセージ
    loop {}
}

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで、`_start`という名前の関数を探すので
    // この関数がエントリポイントとなる
    loop {}
}


