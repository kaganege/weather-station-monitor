use embassy_rp::watchdog::Watchdog;
use embassy_sync::{blocking_mutex::CriticalSectionMutex, once_lock::OnceLock};
use embassy_time::{Duration, Ticker};

define_peripherals! {
    WatchdogPeripherals {
        WATCHDOG
    }
}

pub static WATCHDOG: OnceLock<CriticalSectionMutex<Watchdog>> = OnceLock::new();

pub fn init(peripherals: WatchdogPeripherals<'static>, spawner: &embassy_executor::SendSpawner) {
    let watchdog = Watchdog::new(peripherals.WATCHDOG);
    if WATCHDOG.init(CriticalSectionMutex::new(watchdog)).is_err() {
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }

    spawner.spawn(feed_task().unwrap());
}

#[embassy_executor::task]
async fn feed_task() {
    let mut timer = Ticker::every(Duration::from_secs(2));

    let feed_interval = Duration::from_secs(5);
    unsafe { WATCHDOG.get().await.lock_mut(|wd| wd.start(feed_interval)) };

    loop {
        timer.next().await;
        unsafe { WATCHDOG.get().await.lock_mut(|wd| wd.feed(feed_interval)) };
    }
}
