use embassy_rp::uart::{self, Async, Config, UartRx};
use embassy_time::Timer;

use crate::station::raw::RAW_DATA_END_BYTE;

use super::{
    data::{Data, ParseError},
    raw::{
        Field as _, RawAirPressure, RawData, RawHumidity, RawRainfall1Day, RawRainfall1Hour,
        RawTemperature, RawWindDirection, RawWindSpeed1Min, RawWindSpeed5Min,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serial error: {0}")]
    Serial(#[from] uart::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
}

pub struct StationController<'a> {
    rx: UartRx<'a, Async>,
}

impl<'a> StationController<'a> {
    #[inline]
    pub fn uart_config() -> Config {
        let mut config = Config::default();
        config.baudrate = 9600;
        config
    }

    #[inline]
    pub const fn new(rx: UartRx<'a, Async>) -> Self {
        Self { rx }
    }

    pub async fn read(&mut self) -> Result<Data, Error> {
        let data = self.read_raw_data().await?;
        let data: Data = data.try_into()?;

        Ok(data)
    }

    async fn read_raw_data(&mut self) -> Result<RawData, Error> {
        let mut buf = [0; size_of::<RawData>()];

        loop {
            self.wait_for_first_byte(&mut buf[0]).await?;

            loop {
                match self.rx.read(&mut buf[1..]).await {
                    Err(uart::Error::Overrun) => {
                        debug!("Uart overrun while reading station data");
                    }
                    Err(uart::Error::Break) => {
                        Timer::after_millis(200).await;
                        continue;
                    }
                    result => result?,
                }

                match self.read_byte().await? {
                    // End of data
                    RAW_DATA_END_BYTE => {
                        // Skip CRC and carriage return
                        #[expect(
                            clippy::let_underscore_must_use,
                            reason = "We don't care about the CRC and carriage return bytes"
                        )]
                        let _ = self.rx.blocking_read(&mut [0; 3]);

                        break;
                    }

                    // New data, overwrite existing data
                    RawWindDirection::TAG => {
                        continue;
                    }

                    _ => {
                        warn!("Unexpected byte received while verifying raw data");

                        self.wait_for_first_byte(&mut buf[0]).await?;
                        continue;
                    }
                }
            }

            let data = RawData::from_array(buf);

            debug!("Station buffer: {buf:?}");
            debug!("{data:?}");

            if data.wind_speed_1_min.tag != RawWindSpeed1Min::TAG {
                warn!("Unexpected wind speed 1 min tag received");
                continue;
            }

            if data.wind_speed_5_min.tag != RawWindSpeed5Min::TAG {
                warn!("Unexpected wind speed 5 min tag received");
                continue;
            }

            if data.temperature.tag != RawTemperature::TAG {
                warn!("Unexpected temperature tag received");
                continue;
            }

            if data.rainfall_1_hour.tag != RawRainfall1Hour::TAG {
                warn!("Unexpected rainfall 1 hour tag received");
                continue;
            }

            if data.rainfall_1_day.tag != RawRainfall1Day::TAG {
                warn!("Unexpected rainfall 1 day tag received");
                continue;
            }

            if data.humidity.tag != RawHumidity::TAG {
                warn!("Unexpected humidity tag received");
                continue;
            }

            if data.air_pressure.tag != RawAirPressure::TAG {
                warn!("Unexpected air pressure tag received");
                continue;
            }

            return Ok(data);
        }
    }

    async fn wait_for_first_byte(&mut self, buf: &mut u8) -> Result<(), Error> {
        loop {
            *buf = self.read_byte().await?;

            if *buf == RawWindDirection::TAG {
                return Ok(());
            }
        }
    }

    async fn read_byte(&mut self) -> Result<u8, uart::Error> {
        let mut buf = [0];

        loop {
            match self.rx.blocking_read(&mut buf) {
                Err(uart::Error::Overrun) => debug!("Uart overrun while waiting first byte"),
                Err(uart::Error::Break) => {
                    Timer::after_millis(100).await;
                    continue;
                }
                result => result?,
            }

            return Ok(buf[0]);
        }
    }
}
