use core::net::{Ipv4Addr, SocketAddrV4};

use edge_dhcp::{
    io::{self, DEFAULT_SERVER_PORT},
    server::ServerOptions,
};
use edge_nal::UdpBind;
use edge_nal_embassy::Udp;
use embassy_net::Stack;
use embassy_time::Timer;

use super::GATEWAY_ADDR;

pub(super) const SOCKET_COUNT: usize = 4;

const MAX_CLIENT_COUNT: usize = 16; // Should I consider all the students?
const BUFFER_SIZE: usize = 1024;
const METADATA_BUFFER_SIZE: usize = 10;

type UdpBuffers = edge_nal_embassy::UdpBuffers<
    { SOCKET_COUNT },
    { BUFFER_SIZE },
    { BUFFER_SIZE },
    { METADATA_BUFFER_SIZE },
>;
type Server<F> = edge_dhcp::server::Server<F, { MAX_CLIENT_COUNT }>;

pub fn init(stack: Stack<'static>, spawner: &embassy_executor::Spawner) {
    spawner.spawn(run_dhcp(stack).unwrap());
}

#[embassy_executor::task]
async fn run_dhcp(stack: Stack<'static>) {
    info!("DHCP server starting");

    let mut buf = [0u8; 1500];
    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let buffers = UdpBuffers::new();
    let unbound_socket = Udp::new(stack, &buffers);
    let mut bound_socket = unbound_socket
        .bind(core::net::SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();

    loop {
        if let Err(e) = io::server::run(
            &mut Server::new_with_et(GATEWAY_ADDR),
            &ServerOptions::new(GATEWAY_ADDR, Some(&mut gw_buf)),
            &mut bound_socket,
            &mut buf,
        )
        .await
        {
            warn!("DHCP server error: {e}")
        }

        Timer::after_millis(500).await;
    }
}
