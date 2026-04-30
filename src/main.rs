#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use embassy_executor::{InterruptExecutor, Spawner};
use embassy_rp::{
    bind_interrupts, dma,
    interrupt::{self, InterruptExt, Priority},
    peripherals::{DMA_CH0, DMA_CH1, DMA_CH2, PIO0, UART0, USB},
    pio, uart, usb,
};

use crate::{
    logger::LoggerPeripherals, network::NetworkPeripherals, station::StationPeripherals,
    watchdog::WatchdogPeripherals,
};

#[macro_use]
mod macros;
#[macro_use]
mod color;
mod logger;
mod network;
mod panic_handler;
mod random;
mod station;
mod watchdog;

static CORE_EXECUTOR: InterruptExecutor = InterruptExecutor::new();

bind_interrupts! {
    pub struct Irqs {
        DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>, dma::InterruptHandler<DMA_CH2>;
        USBCTRL_IRQ => usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
        UART0_IRQ => uart::InterruptHandler<UART0>;
    }
}

#[embassy_rp::interrupt]
unsafe fn SWI_IRQ_0() {
    // SAFETY: This function is an interrupt handler, so it is safe to call.
    unsafe { CORE_EXECUTOR.on_interrupt() }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    interrupt::SWI_IRQ_0.set_priority(Priority::P1);
    let core_spawner = CORE_EXECUTOR.start(interrupt::SWI_IRQ_0);

    watchdog::init(
        borrow_peripherals!(peripherals, WatchdogPeripherals { WATCHDOG }),
        &core_spawner,
    );

    logger::init(
        borrow_peripherals!(peripherals, LoggerPeripherals { USB }),
        &core_spawner,
    );

    {
        let watchdog = watchdog::WATCHDOG.get().await;

        if let Some(panic_occured) = unsafe {
            // SAFETY: Mutex::lock_mut does not called multiple times in the same lock
            watchdog.lock_mut(|wd| wd.reset_reason().map(|_| wd.get_scratch(0) == 1))
        } {
            if panic_occured {
                warn!("The device previously shut down due to a panic");
            } else {
                warn!(
                    "The device previously shut down due to a deadlock (ignore if this is the first boot)"
                );
            }

            // SAFETY: Mutex::lock_mut does not called multiple times in the same lock
            unsafe {
                watchdog.lock_mut(|wd| {
                    wd.set_scratch(0, 0); // Clear panic flag

                    // Clear the watchdog reset reason
                    embassy_rp::pac::WATCHDOG.reason().modify(|r| {
                        r.set_force(false);
                        r.set_timer(false);
                    })
                });
            }
        }
    }

    cfg_select! {
        debug_assertions => {
            info!("Hello from debug build!");
        }
        _ => {
            info!("Hello world");
        }
    }

    station::init(
        borrow_peripherals!(
            peripherals,
            StationPeripherals {
                PIN_1,
                UART0,
                DMA_CH1
            }
        ),
        &spawner,
    );

    network::init(
        borrow_peripherals!(
            peripherals,
            NetworkPeripherals {
                PIN_23,
                PIN_24,
                PIN_25,
                PIN_29,
                PIO0,
                DMA_CH0
            }
        ),
        &spawner,
    )
    .await;
}
