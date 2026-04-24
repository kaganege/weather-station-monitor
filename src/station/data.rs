use atoi::FromRadix10;
use const_default::ConstDefault;
use num_traits::Zero;
use uom::si::{
    angle::degree,
    pressure::{hectopascal, inch_of_water},
    ratio::percent,
    thermodynamic_temperature::degree_fahrenheit,
    velocity::mile_per_hour,
};

pub use uom::si::{
    f32::{
        Pressure as PressureF32, ThermodynamicTemperature as ThermodynamicTemperatureF32,
        Velocity as VelocityF32,
    },
    u8::Ratio as RatioU8,
    u16::Angle as AngleU16,
};

use crate::station::raw::RawField;

use super::raw::{
    self, RawAirPressure, RawData, RawHumidity, RawRainfall1Day, RawRainfall1Hour, RawTemperature,
    RawWindDirection, RawWindSpeed1Min, RawWindSpeed5Min,
};

#[derive(Debug, thiserror::Error)]
#[error("number parsing failed")]
pub struct NumberParseError;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    WrongFieldTag(#[from] raw::WrongFieldTagError),
    #[error(transparent)]
    NumberParsingFailed(#[from] NumberParseError),
}

#[derive(Debug, Clone, Copy, Default, ConstDefault)]
pub struct Data {
    /// Wind direction
    pub wind_direction: Option<AngleU16>,
    /// Wind speed over the last 1 minute
    pub wind_speed_1_min: Option<VelocityF32>,
    /// Max wind speed over the last 5 minutes
    pub max_wind_speed_5_min: Option<VelocityF32>,
    /// Temperature
    pub temperature: Option<ThermodynamicTemperatureF32>,
    /// Rainfall over the last hour
    pub rainfall_1_hour: Option<PressureF32>,
    /// Rainfall over the last 24 hours
    pub rainfall_1_day: Option<PressureF32>,
    /// Relative humidity
    pub humidity: Option<RatioU8>,
    /// Air pressure
    pub air_pressure: Option<PressureF32>,
}

impl TryFrom<RawData> for Data {
    type Error = ParseError;

    fn try_from(raw: RawData) -> Result<Self, Self::Error> {
        let wind_direction = RawWindDirection::try_from(raw.wind_direction)?;
        let wind_speed_1_min = RawWindSpeed1Min::try_from(raw.wind_speed_1_min)?;
        let max_wind_speed_5_min = RawWindSpeed5Min::try_from(raw.wind_speed_5_min)?;
        let temperature = RawTemperature::try_from(raw.temperature)?;
        let rainfall_1_hour = RawRainfall1Hour::try_from(raw.rainfall_1_hour)?;
        let rainfall_1_day = RawRainfall1Day::try_from(raw.rainfall_1_day)?;
        let humidity = RawHumidity::try_from(raw.humidity)?;
        let air_pressure = RawAirPressure::try_from(raw.air_pressure)?;

        Ok(Self {
            wind_direction: parse_field(wind_direction)?.map(AngleU16::new::<degree>),
            wind_speed_1_min: parse_field(wind_speed_1_min)?
                .map(|val| val as f32 * 10.0) // 0.1 mph to mph
                .map(VelocityF32::new::<mile_per_hour>),
            max_wind_speed_5_min: parse_field(max_wind_speed_5_min)?
                .map(|val| val as f32 * 10.0) // 0.1 mph to mph
                .map(VelocityF32::new::<mile_per_hour>),
            temperature: parse_field(temperature)?
                .map(|val| val as f32)
                .map(ThermodynamicTemperatureF32::new::<degree_fahrenheit>),
            rainfall_1_hour: parse_field(rainfall_1_hour)?
                .map(|val| val as f32 * 100.0) // 0.01 inch to inch
                .map(PressureF32::new::<inch_of_water>),
            rainfall_1_day: parse_field(rainfall_1_day)?
                .map(|val| val as f32 * 100.0) // 0.01 inch to inch
                .map(PressureF32::new::<inch_of_water>),
            humidity: parse_field(humidity)?
                .map(|val| if val == 0 { 100 } else { val }) // 00 means 100%
                .map(RatioU8::new::<percent>),
            air_pressure: parse_field(air_pressure)?
                .map(|val| val as f32 * 10.0) // 0.1 hPa to hPa
                .map(PressureF32::new::<hectopascal>),
        })
    }
}

#[inline]
fn parse_field<F, const N: usize>(field: F) -> Result<Option<F::ActualType>, NumberParseError>
where
    F: raw::Field + Into<RawField<N>>,
    F::ActualType: Zero + FromRadix10 + PartialEq,
{
    let raw_field: RawField<N> = field.into();
    raw_field
        .is_valid()
        .then(|| parse_byte_str(&raw_field.data))
        .transpose()
}

#[inline]
fn parse_byte_str<T>(buf: &[u8]) -> Result<T, NumberParseError>
where
    T: Zero + FromRadix10 + PartialEq,
{
    match <T as FromRadix10>::from_radix_10(buf) {
        (value, 0) if value == <T as Zero>::zero() => Err(NumberParseError), // Invalid number
        (_, len) if len != buf.len() => Err(NumberParseError),               // Corrupted number
        (value, _) => Ok(value),
    }
}
