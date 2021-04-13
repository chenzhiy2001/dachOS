//! 预约和处理时钟中断

use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};
pub static mut TICKCOUNT: usize = 0;

// 这是时钟中断的间隔
static INTERVAL: usize = 100000;

pub fn set_next_timeout(){
    set_timer(time::read() + INTERVAL);
}

/// 初始化时钟中断
/// 
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer(); 
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

pub fn tick(){
    set_next_timeout();
    unsafe{
        TICKCOUNT += 1;
        if TICKCOUNT % 100 == 0 {
            println!("{} ticks", TICKCOUNT);
        }
    }
}
