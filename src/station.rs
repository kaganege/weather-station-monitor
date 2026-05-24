use const_default::ConstDefault;
use embassy_rp::uart::BufferedUartRx;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    rwlock::{RwLock, RwLockReadGuard},
};
use embassy_time::Timer;
use num_traits::float::FloatCore;
use static_cell::StaticCell;
use uom::si::{
    f32::{Pressure, Ratio, ThermodynamicTemperature, Velocity},
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
        #[expect(
            clippy::cast_sign_loss,
            reason = "`random_f32` always returns a positive number"
        )]
        wind_direction: random_option(|| match (random_f32() * 7.0).round() as u8 {
            0 => data::WindDirection::North,
            1 => data::WindDirection::NorthEast,
            2 => data::WindDirection::East,
            3 => data::WindDirection::SouthEast,
            4 => data::WindDirection::South,
            5 => data::WindDirection::SouthWest,
            6 => data::WindDirection::West,
            7 => data::WindDirection::NorthWest,
            // SAFETY: random_f32() * 7.0 is in [0, 7], so match arms cover all possible cases
            _ => unsafe { core::hint::unreachable_unchecked() },
        }),
        wind_speed_1_min: random_option(
            || Velocity::new::<meter_per_second>(random::<u8>() as f32),
        ),
        max_wind_speed_5_min: random_option(|| {
            Velocity::new::<meter_per_second>(random::<u8>() as f32)
        }),
        temperature: random_option(|| {
            ThermodynamicTemperature::new::<degree_celsius>(60.0 - random_f32() * 75.0)
        }),
        rainfall_1_hour: random_option(|| {
            Pressure::new::<millimeter_of_water>(random::<u8>() as f32)
        }),
        rainfall_1_day: random_option(|| {
            Pressure::new::<millimeter_of_water>(random::<u8>() as f32)
        }),
        humidity: random_option(|| Ratio::new::<percent>((random::<u8>() % 100) as f32)),
        air_pressure: random_option(|| Pressure::new::<hectopascal>(random_f32() * 1020.0)),
    }
}

pub fn random_all_available_data() -> Data {
    Data {
        #[expect(
            clippy::cast_sign_loss,
            reason = "`random_f32` always returns a positive number"
        )]
        wind_direction: Some(match (random_f32() * 7.0).round() as u8 {
            0 => data::WindDirection::North,
            1 => data::WindDirection::NorthEast,
            2 => data::WindDirection::East,
            3 => data::WindDirection::SouthEast,
            4 => data::WindDirection::South,
            5 => data::WindDirection::SouthWest,
            6 => data::WindDirection::West,
            7 => data::WindDirection::NorthWest,
            // SAFETY: random_f32() * 7.0 is in [0, 7], so match arms cover all possible cases
            _ => unsafe { core::hint::unreachable_unchecked() },
        }),
        wind_speed_1_min: Some(Velocity::new::<meter_per_second>(random::<u8>() as f32)),
        max_wind_speed_5_min: Some(Velocity::new::<meter_per_second>(random::<u8>() as f32)),
        temperature: Some({
            ThermodynamicTemperature::new::<degree_celsius>(60.0 - random_f32() * 75.0)
        }),
        rainfall_1_hour: Some(Pressure::new::<millimeter_of_water>(random::<u8>() as f32)),
        rainfall_1_day: Some(Pressure::new::<millimeter_of_water>(random::<u8>() as f32)),
        humidity: Some(Ratio::new::<percent>((random::<u8>() % 100) as f32)),
        air_pressure: Some(Pressure::new::<hectopascal>(random_f32() * 1020.0)),
    }
}

pub fn init(peripherals: StationPeripherals<'static>, spawner: &embassy_executor::Spawner) {
    static BUF: StaticCell<[u8; size_of::<raw::RawData>()]> = StaticCell::new();
    let rx = BufferedUartRx::new(
        peripherals.UART0,
        Irqs,
        peripherals.PIN_1,
        // SAFETY: This is safe because the UART will fill the buffer
        unsafe { BUF.uninit().assume_init_mut() },
        StationController::uart_config(),
    );
    let controller = StationController::new(rx);

    spawner.spawn(read_task(controller).unwrap());
}

#[embassy_executor::task]
async fn read_task(mut controller: StationController) -> ! {
    loop {
        let Ok(data) = controller.read().await.inspect_err(|e| error!("{e}")) else {
            Timer::after_millis(200).await;
            continue;
        };

        *CACHE.write().await = data;

        BLINK_SIGNAL.signal(());

        Timer::after_millis(500).await;
    }
}
