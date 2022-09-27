use embassy_time::Duration;
use futures_util::StreamExt;

#[embassy_executor::task]
pub async fn kmain() {
    defmt::info!("Kernel starting ...");
    defmt::info!("Ticking every 3 seconds");

    let mut ticker = embassy_time::Ticker::every(Duration::from_secs(3));
    while ticker.next().await.is_some() {
        defmt::info!("Timer tick");
    }

    defmt::error!("We shouldn't be here");
}
