// main.rs
#![no_std]

use core::panic::PanicInfo;

// panic_handler attribute はパニックが発生したときにコンパイラが呼び出す関数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // PanicInfo: パニックが発生したファイルと行、およびオプションでパニックメッセージ
    // この関数は戻り値を取るべきではないので、“never” 型(!)を返すことで発散する関数となります。
    // 今のところこの関数でできることは多くないので、無限にループするだけです。
    loop {}
}

fn main() {

}
