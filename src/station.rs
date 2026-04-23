use const_default::ConstDefault;
use embassy_rp::uart::UartRx;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    rwlock::{RwLock, RwLockReadGuard},
};
use embassy_time::Timer;
use uom::si::{
    angle::degree,
    pressure::{hectopascal, millimeter_of_water},
    ratio::percent,
    thermodynamic_temperature::degree_celsius,
    velocity::meter_per_second,
};

use crate::{
    Irqs,
    network::BLINK_SIGNAL,
    random::{random, random_f32, random_option},
};

use controller::StationController;

pub use data::*;

mod controller;
mod data;
mod raw;

define_peripherals! {
    StationPeripherals {
        UART0,
        DMA_CH1,
        PIN_1, // RX
    }
}

static CACHE: RwLock<CriticalSectionRawMutex, Data> = RwLock::new(Data::DEFAULT);

pub fn read_data<'a>()
-> impl Future<Output = RwLockReadGuard<'a, CriticalSectionRawMutex, Data>> + Send {
    CACHE.read()
}

pub fn random_data() -> Data {
    Data {
        wind_direction: random_option(|| AngleU16::new::<degree>(random())),
        wind_speed_1_min: random_option(|| {
            VelocityF32::new::<meter_per_second>(random::<u8>() as f32)
        }),
        max_wind_speed_5_min: random_option(|| {
            VelocityF32::new::<meter_per_second>(random::<u8>() as f32)
        }),
        temperature: random_option(|| {
            ThermodynamicTemperatureF32::new::<degree_celsius>(60.0 - random_f32() * 75.0)
        }),
        rainfall_1_hour: random_option(|| {
            PressureF32::new::<millimeter_of_water>(random::<u8>() as f32)
        }),
        rainfall_1_day: random_option(|| {
            PressureF32::new::<millimeter_of_water>(random::<u8>() as f32)
        }),
        humidity: random_option(|| RatioU8::new::<percent>(random::<u8>() % 100)),
        air_pressure: random_option(|| PressureF32::new::<hectopascal>(random_f32() * 1020.0)),
    }
}

pub fn init(peripherals: StationPeripherals<'static>, spawner: &embassy_executor::Spawner) {
    let rx = UartRx::new(
        peripherals.UART0,
        peripherals.PIN_1,
        Irqs,
        peripherals.DMA_CH1,
        StationController::uart_config(),
    );
    let controller = StationController::new(rx);

    spawner.spawn(read_task(controller).unwrap());
}

#[embassy_executor::task]
async fn read_task(mut controller: StationController<'static>) -> ! {
    loop {
        let Ok(data) = controller.read().await.inspect_err(|e| error!("{e}")) else {
            Timer::after_millis(200).await;
            continue;
        };

        *CACHE.write().await = data;

        BLINK_SIGNAL.signal(());
    }
}
