use embassy_rp::uart::{self, Async, Config, UartRx};

use super::{
    data::{Data, ParseError},
    raw::{Field as _, RawBreak, RawData, RawWindDirection},
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
    pub fn uart_config() -> Config {
        let mut config = Config::default();
        config.baudrate = 9600;
        config
    }

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
            self.wait_for_first_byte().await?;

            buf[0] = RawWindDirection::TAG;
            self.rx.read(&mut buf[1..]).await?;

            let data = RawData::from_slice(&buf);

            // Verify break
            if data.r#break == RawBreak::TAG {
                break Ok(*data);
            }

            warn!("Unexpected break received");
        }
    }

    async fn wait_for_first_byte(&mut self) -> Result<(), Error> {
        let mut buf = [0];

        loop {
            self.rx.read(&mut buf).await?;

            if buf[0] == RawWindDirection::TAG {
                return Ok(());
            }
        }
    }
}
