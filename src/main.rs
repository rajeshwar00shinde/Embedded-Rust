#![no_std]
#![no_main]


//3f200008 fsel2 1<<3 turn pin21 into output
//3f20001c gpio1_set 1<<<21 turn pin 21 on
//3f200028 gpio1_clear 1<<21 turns pin 21 off

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;
    global_asm!(
        ".section .text._start"
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop {
            
            //Turn pin On
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop")
            }

            //Turn pin Off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


