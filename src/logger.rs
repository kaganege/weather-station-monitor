use core::fmt::Write;

use embassy_rp::{peripherals::USB, usb::Driver};
use embassy_usb::{
    Builder, Config,
    class::cdc_acm::{CdcAcmClass, State},
};
use embassy_usb_logger::{DummyHandler, MAX_PACKET_SIZE, UsbLogger};
use futures::future::join;
use log::{Level, LevelFilter};

use crate::{Irqs, color::Color};

const LOG_LEVEL: LevelFilter = LevelFilter::Info;
const LOG_BUFFER_SIZE: usize = 1024;

define_peripherals! {
    LoggerPeripherals {
        USB,
    }
}

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    static LOGGER: UsbLogger<LOG_BUFFER_SIZE, DummyHandler> =
        UsbLogger::with_custom_style(|record, writer| {
            let level = record.level();
            let color = match level {
                Level::Error => Color::BrightRed,
                Level::Warn => Color::BrightYellow,
                Level::Info => Color::BrightGreen,
                Level::Debug => Color::BrightBlue,
                Level::Trace => Color::BrightMagenta,
            };
            let body = format_args!("[{level}] {}", record.args());
            let body = colored!(body).with_fg(color);

            #[expect(
                clippy::let_underscore_must_use,
                reason = "We can't do much if writeln! fails, so we ignore the result"
            )]
            let _ = writeln!(writer, "{body}");
        });

    #[expect(
        clippy::multiple_unsafe_ops_per_block,
        reason = "The safety comment applies to the entire block"
    )]
    // SAFETY: this function called once at the start of the program
    unsafe {
        if log::set_logger_racy(&LOGGER).is_ok() {
            log::set_max_level_racy(LOG_LEVEL);
        }
    }

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Kağan Ege");
    config.product = Some("Weather station monitor");
    config.serial_number = None;
    config.max_power = 100;
    config.max_packet_size_0 = MAX_PACKET_SIZE;

    let mut config_descriptor = [0; 128];
    let mut bos_descriptor = [0; 16];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();
    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );
    let class = CdcAcmClass::new(&mut builder, &mut state, MAX_PACKET_SIZE as u16);
    let mut device = builder.build();

    join(device.run(), LOGGER.create_future_from_class(class)).await;
}

pub fn init(peripherals: LoggerPeripherals<'static>, spawner: &embassy_executor::SendSpawner) {
    let usb_driver = Driver::new(peripherals.USB, Irqs);

    spawner.spawn(logger_task(usb_driver).unwrap());
}
