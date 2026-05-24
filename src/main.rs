#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts, dma,
    peripherals::{DMA_CH0, PIO0, UART0, USB},
    pio, uart, usb,
    watchdog::ResetReason,
};
use embassy_time::Timer;

use crate::{
    logger::LoggerPeripherals, network::NetworkPeripherals, station::StationPeripherals,
    watchdog::WatchdogPeripherals,
};

#[macro_use]
mod macros;
mod logger;
mod network;
mod panic_handler;
mod random;
mod station;
mod watchdog;

bind_interrupts! {
    pub struct Irqs {
        DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
        USBCTRL_IRQ => usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
        UART0_IRQ => uart::BufferedInterruptHandler<UART0>;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    logger::init(
        borrow_peripherals!(peripherals, LoggerPeripherals { USB }),
        &spawner,
    );

    watchdog::init(
        borrow_peripherals!(peripherals, WatchdogPeripherals { WATCHDOG }),
        &spawner,
    );

    {
        let watchdog = watchdog::WATCHDOG.get().await;

        if let Some(panic) = panic_persist::get_panic_message_utf8() {
            warn!("The device previously shut down due to a panic");
            warn!("Panic message:\n{panic}");
        } else if watchdog.lock(|wd| wd.reset_reason()) == Some(ResetReason::TimedOut) {
            warn!(
                "The device previously shut down due to a deadlock (ignore if this is the first boot)"
            );
        }
    }

    cfg_select! {
        debug_assertions => {
            Timer::after_secs(1).await;
            info!("Hello from debug build!");
        }
        _ => {
            info!("Hello world");
        }
    }

    station::init(
        borrow_peripherals!(peripherals, StationPeripherals { PIN_1, UART0 }),
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
