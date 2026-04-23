use askama::Template;
use edge_http::io::{Error, server::Connection};
use edge_nal::TcpSplit;
use embedded_io_async::{Read, Write};
use num_traits::float::FloatCore;
use picoserve::io::WriteExt;
use uom::si::{
    pressure::{hectopascal, millimeter_of_water},
    ratio::percent,
    thermodynamic_temperature::degree_celsius,
    velocity::meter_per_second,
};

use crate::station;

fn round(value: f32, precision: u32) -> f32 {
    let multiplier = 10_f32.powi(precision as i32);
    (value * multiplier).round() / multiplier
}

#[derive(Template)]
#[template(path = "index.askama")]
pub struct IndexTemplate {
    data: station::Data,
}

#[derive(Template)]
#[template(path = "demo.askama")]
pub struct DemoTemplate {
    data: station::Data,
}

pub async fn handle<T, const N: usize>(
    path: &str,
    conn: &mut Connection<'_, T, N>,
) -> Result<bool, Error<T::Error>>
where
    T: Read + Write + TcpSplit,
{
    match path {
        "/" => {
            conn.initiate_response(
                200,
                Some("OK"),
                &[("Content-Type", "text/html; charset=utf-8")],
            )
            .await?;

            let data = *station::read_data().await;

            write!(conn, "{}", IndexTemplate { data }).await?;
        }

        "/demo" => {
            conn.initiate_response(
                200,
                Some("OK"),
                &[("Content-Type", "text/html; charset=utf-8")],
            )
            .await?;

            let data = station::random_data();

            write!(conn, "{}", DemoTemplate { data }).await?;
        }

        _ => {
            return Ok(false);
        }
    }

    Ok(true)
}
