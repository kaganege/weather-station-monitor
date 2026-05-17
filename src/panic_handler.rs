use core::panic::PanicInfo;

use panic_persist::report_panic_info;

use crate::watchdog::WATCHDOG;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(watchdog) = WATCHDOG.try_get() {
        // SAFETY: Mutex::lock_mut does not called multiple times in the same lock
        unsafe {
            watchdog.lock_mut(|wd| wd.stop());
        }
    }

    cortex_m::interrupt::disable();

    report_panic_info(info);

    cfg_select! {
        debug_assertions => {
            embassy_rp::rom_data::reset_to_usb_boot(0, 0);

            loop {
                cortex_m::asm::nop();
            }
        }

        _ => {
            cortex_m::peripheral::SCB::sys_reset();
        }
    }
}
