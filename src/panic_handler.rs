use core::panic::PanicInfo;

// use cm_backtrace::Backtrace;
use embassy_time::Duration;

use crate::watchdog::WATCHDOG;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(watchdog) = WATCHDOG.try_get() {
        unsafe {
            watchdog.lock_mut(|wd| {
                wd.stop();
                // Set scratch register to 1 to indicate panic occurred
                wd.set_scratch(0, 1);
            });
        }
    }

    error!("{info}");

    embassy_time::block_for(Duration::from_secs(1));

    cfg_select! {
        debug_assertions => {
            embassy_rp::rom_data::reset_to_usb_boot(0, 0);
        }

        _ => {
            if let Some(watchdog) = WATCHDOG.try_get() {
                unsafe {
                    watchdog.lock_mut(|wd| wd.trigger_reset());
                }

            }
        }
    }

    loop {}
}
