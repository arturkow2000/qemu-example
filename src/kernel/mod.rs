// use embassy_time::Duration;
// use futures_util::StreamExt;

#[inline(never)]
extern "C" fn test1() {
    test2()
}

#[inline(never)]
extern "C" fn test2() {
    test3()
}

#[inline(never)]
extern "C" fn test3() {
    test4()
}

#[inline(never)]
extern "C" fn test4() {
    test5()
}

#[inline(never)]
extern "C" fn test5() {
    cortex_a_rt::panic::do_backtrace();
}

#[embassy_executor::task]
pub async fn kmain() {
    defmt::info!("Kernel starting ...");
    // ehabi_test();
    test1();
    /*defmt::info!("Ticking every 3 seconds");

    let mut ticker = embassy_time::Ticker::every(Duration::from_secs(3));
    while ticker.next().await.is_some() {
        defmt::info!("Timer tick");
    }

    defmt::error!("We shouldn't be here");*/
}
