use core::fmt::Display;

use edge_http::{
    Method,
    io::{
        Error,
        server::{Connection, Handler},
    },
};
use edge_nal::TcpSplit;
use embedded_io_async::{Read, Write};

mod routes;

pub(super) struct HttpHandler;

impl Handler for HttpHandler {
    type Error<E>
        = Error<E>
    where
        E: core::fmt::Debug;

    async fn handle<T, const N: usize>(
        &self,
        _task_id: impl Display + Copy,
        conn: &mut Connection<'_, T, N>,
    ) -> Result<(), Self::Error<T::Error>>
    where
        T: Read + Write + TcpSplit,
    {
        let headers = conn.headers()?;
        let path = headers.path;

        info!("A {} request to {} received", headers.method, path);

        match path {
            _ if headers.method != Method::Get => {
                conn.initiate_response(405, Some("Method Not Allowed"), &[])
                    .await?;
            }

            _ if let Some(subpath) = path.strip_prefix("/api")
                && routes::api::handle(subpath, conn).await? => {}

            _ if routes::template::handle(path, conn).await? => {}

            _ => {
                conn.initiate_response(404, Some("Not Found"), &[]).await?;
            }
        }

        Ok(())
    }
}
