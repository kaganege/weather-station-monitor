use edge_http::io::{Error, server::Connection};
use edge_nal::TcpSplit;
use embedded_io_async::{Read, Write};
use picoserve::response::{Json, sse::EventData};
use uom::si::{
    angle::degree,
    pressure::{hectopascal, millimeter_of_water},
    ratio::percent,
    thermodynamic_temperature::degree_celsius,
    velocity::meter_per_second,
};

use crate::station;

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SerializedData {
    /// Wind direction in degrees
    pub wind_direction: Option<f32>,
    /// Wind speed over the last 1 minute in m/s
    pub wind_speed_1_min: Option<f32>,
    /// Max wind speed over the last 5 minutes in m/s
    pub max_wind_speed_5_min: Option<f32>,
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Rainfall over the last hour in mm
    pub rainfall_1_hour: Option<f32>,
    /// Rainfall over the last 24 hours in mm
    pub rainfall_1_day: Option<f32>,
    /// Relative humidity in %
    pub humidity: Option<u8>,
    /// Air pressure in hPa
    pub air_pressure: Option<f32>,
}

impl From<station::Data> for SerializedData {
    fn from(data: station::Data) -> Self {
        Self {
            wind_direction: data.wind_direction.map(|v| v.get::<degree>()),
            wind_speed_1_min: data.wind_speed_1_min.map(|v| v.get::<meter_per_second>()),
            max_wind_speed_5_min: data
                .max_wind_speed_5_min
                .map(|v| v.get::<meter_per_second>()),
            temperature: data.temperature.map(|v| v.get::<degree_celsius>()),
            rainfall_1_hour: data.rainfall_1_hour.map(|v| v.get::<millimeter_of_water>()),
            rainfall_1_day: data.rainfall_1_day.map(|v| v.get::<millimeter_of_water>()),
            humidity: data.humidity.map(|v| v.get::<percent>() as u8),
            air_pressure: data.air_pressure.map(|v| v.get::<hectopascal>()),
        }
    }
}

pub async fn handle<T, const N: usize>(
    path: &str,
    conn: &mut Connection<'_, T, N>,
) -> Result<bool, Error<T::Error>>
where
    T: Read + Write + TcpSplit,
{
    match path {
        "/health" => {
            conn.initiate_response(200, Some("OK"), &[("Content-Type", "text/plain")])
                .await?;
            conn.write_all(b"OK").await?;
        }

        "/data" => {
            conn.initiate_response(200, Some("OK"), &[("Content-Type", "application/json")])
                .await?;

            let data = *station::read_data().await;
            let data = Json(SerializedData::from(data));

            data.write_to(conn).await?;
        }

        "/random-data" => {
            conn.initiate_response(200, Some("OK"), &[("Content-Type", "application/json")])
                .await?;

            let data = Json(SerializedData::from(station::random_data()));
            data.write_to(conn).await?;
        }

        _ => {
            return Ok(false);
        }
    }

    Ok(true)
}
