#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod interrupt;
mod memory;

extern crate alloc;
/* 
extern crate foo indicates that you want to link against an external library 
and brings the top-level crate name into scope (equivalent to use foo). As of 
Rust 2018, in most cases you won't need to use extern crate anymore because 
Cargo informs the compiler about what crates are present. (There are one or two 
exceptions https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html#an-exception)

alloc: Items in the alloc crate are usually accessed via re-exports in the std 
crate. If you are working with a no_std crate that supports allocation, then you 
may need to explicitly import alloc.
*/

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
pub fn rust_main()->(){
//    clear_bss();
    interrupt::init();
    memory::init();

    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v,5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000{
        vec.push(i);
    }
    assert_eq!(vec.len(),10000);
    for (i, value) in vec.into_iter().enumerate(){
        assert_eq!(value,i);
    }
    println!("heal test passed");

    panic!();
    println!("Hello Quark!");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    // make this uncommented if you want to see ticks
    // loop{}
}


