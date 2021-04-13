#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod interrupt;

global_asm!(include_str!("entry.asm"));
// global_asm!(include_str!("link_app.S"));


// pub fn clear_bss(){
//     extern "C"{
//         fn sbss();
//         fn ebss();
//     }
//     (sbss as usize..ebss as usize).for_each(
//         |a|{
//             unsafe{(a as *mut u8).write_volatile(0)}
//         }
//     );
// }


#[no_mangle]
pub fn rust_main()->!{
//    clear_bss();
    interrupt::init();
    println!("Hello Quark!");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    loop{}
//    println!("[kernel] Hello World!");
//    panic!("Shutting down quark......");
}


