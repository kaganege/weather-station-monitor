use core::net::Ipv4Addr;

use cyw43::{Control, NetDriver, ScanOptions, ScanType, aligned_bytes};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use dotenvy_macro::dotenv;
use embassy_net::{Config, StackResources};
use embassy_rp::{
    clocks::RoscRng,
    dma,
    gpio::{Level, Output},
    peripherals::PIO0,
    pio::Pio,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Timer;
use futures::FutureExt;
use static_cell::StaticCell;

use crate::Irqs;

mod dhcp;
mod web;

const SSID: &str = bounded_str!(dotenv!("SSID"), 1..=32);
const PASSWORD: &str = bounded_str!(dotenv!("PASSWORD"), 8..=64);

const GATEWAY_ADDR: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 1);

const SOCKET_COUNT: usize = dhcp::SOCKET_COUNT + web::SOCKET_COUNT;

pub static BLINK_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

define_peripherals! {
    NetworkPeripherals {
        PIN_23,
        PIN_24,
        PIN_25,
        PIN_29,
        PIO0,
        DMA_CH0,
    }
}

pub async fn init(peripherals: NetworkPeripherals<'static>, spawner: &embassy_executor::Spawner) {
    let fw = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    let pwr = Output::new(peripherals.PIN_23, Level::Low);
    let cs = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        dma::Channel::new(peripherals.DMA_CH0, Irqs),
    );
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    spawner.spawn(cyw43_task(runner).unwrap());

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::Performance)
        .await;

    let ap_config = embassy_net::StaticConfigV4 {
        address: embassy_net::Ipv4Cidr::new(GATEWAY_ADDR, 24),
        gateway: Some(GATEWAY_ADDR),
        dns_servers: Default::default(),
    };

    let mut rng = RoscRng;
    let seed = rng.next_u64();

    static RESOURCES: StaticCell<StackResources<SOCKET_COUNT>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        Config::ipv4_static(ap_config),
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(net_task(runner).unwrap());

    dhcp::init(stack, spawner);
    web::init(stack, spawner);

    let channel = search_for_unused_channel(&mut control).await;
    control.start_ap_wpa2(SSID, PASSWORD, channel).await;

    control.gpio_set(0, true).await;
    Timer::after_secs(1).await;
    control.gpio_set(0, false).await;

    spawner.spawn(blink_task(control).unwrap());
}

async fn search_for_unused_channel(control: &mut Control<'_>) -> u8 {
    const PREFERRED_CHANNELS: [u8; 3] = [6, 1, 11];

    info!("Starting scan for unused channel");

    let mut options = ScanOptions::default();
    options.scan_type = ScanType::Active;

    let mut scanner = control.scan(options).await;
    let mut channel_list = [false; 13];

    while let Some(bss_info) = scanner.next().await {
        let channel = bss_info.ctl_ch;

        info!(
            "Found a bss: ssid={ssid}, rssi={rssi}, channel spec={chanspec}, channel={channel}",
            ssid = str::from_utf8(&bss_info.ssid[..bss_info.ssid_len as usize]).unwrap(),
            rssi = bss_info.rssi,
            chanspec = bss_info.chanspec,
        );

        if let Some(c) = channel_list.get_mut(channel as usize - 1) {
            *c = true;
        }
    }

    let mut unused_channels = channel_list
        .iter()
        .enumerate()
        .filter_map(|(i, is_used)| (!is_used).then_some(i as u8 + 1));
    let Some(first_unused_channel) = unused_channels.clone().next() else {
        return PREFERRED_CHANNELS[0];
    };
    let preferred_channel = unused_channels.find(|channel| PREFERRED_CHANNELS.contains(channel));

    preferred_channel.unwrap_or(first_unused_channel)
}

#[embassy_executor::task]
fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> impl Future<Output = ()> {
    runner.run().map(|_| ())
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn blink_task(mut control: Control<'static>) {
    loop {
        BLINK_SIGNAL.wait().await;

        control.gpio_set(0, true).await;
        Timer::after_millis(200).await;
        control.gpio_set(0, false).await;
    }
}
