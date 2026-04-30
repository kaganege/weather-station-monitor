use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use edge_nal::TcpBind;
use edge_nal_embassy::Tcp;
use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_time::Timer;

use super::GATEWAY_ADDR;

use self::handler::HttpHandler;

mod handler;

pub(super) const SOCKET_COUNT: usize = HTTP_SOCKET_COUNT * 2;

const HTTP_SOCKET_COUNT: usize = 3;
const HTTP_PORT: u16 = 80;
const HTTP_BUF_SIZE: usize = 12 * 1024;
const HTTP_MAX_HEADER_COUNT: usize = edge_http::DEFAULT_MAX_HEADERS_COUNT;

type Server = edge_http::io::server::Server<
    { HTTP_SOCKET_COUNT },
    { HTTP_BUF_SIZE },
    { HTTP_MAX_HEADER_COUNT },
>;
type TcpBuffers =
    edge_nal_embassy::TcpBuffers<{ HTTP_SOCKET_COUNT }, { HTTP_BUF_SIZE }, { HTTP_BUF_SIZE }>;

pub fn init(stack: Stack<'static>, spawner: &Spawner) {
    spawner.spawn(run_server(stack).unwrap());
}

#[embassy_executor::task]
async fn run_server(stack: Stack<'static>) -> ! {
    let mut server = Server::new();

    let buffers = TcpBuffers::new();
    let unbound_socket = Tcp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            HTTP_PORT,
        )))
        .await
        .unwrap();

    loop {
        if HTTP_PORT == 80 {
            info!("Listening on http://{GATEWAY_ADDR}");
        } else {
            info!("Listening on http://{GATEWAY_ADDR}:{HTTP_PORT}");
        }

        if let Err(e) = server.run(None, &mut bound_socket, HttpHandler).await {
            warn!("HTTP server error: {e}");
        }

        Timer::after_secs(1).await;
    }
}
