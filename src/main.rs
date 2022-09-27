#![feature(panic_info_message)]
#![feature(sync_unsafe_cell)]
#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use embassy_executor::raw::Executor;
use fdt::Fdt;
use static_cell::StaticCell;

mod irq;
mod kernel;
mod uart;

#[cortex_a_rt::entry]
fn main() -> ! {
    let fdt = unsafe { Fdt::from_ptr(0x40000000 as *const u8) }.unwrap();
    let pl011 = fdt.find_compatible(&["arm,pl011"]).unwrap();
    let pl011_base = pl011.reg().unwrap().next().unwrap();

    unsafe { uart::init(pl011_base.starting_address as usize) };
    defmt::info!("Hello");

    defmt::debug!("Memory:");
    for region in fdt.memory().regions() {
        if let Some(size) = region.size {
            let start = region.starting_address as usize;
            let end = start + size - 1;
            defmt::debug!("  0x{:08X} - 0x{:08X}", start, end);
        }
    }

    unsafe { irq::init(&fdt) };
    let gic = irq::get();

    gic.interrupt_unmask(15);
    defmt::info!("Triggering interrupt");
    gic.send_sgi_to_self(15);

    defmt::info!("Back from IRQ handler");

    unsafe { generic_timer::embassy::init() };
    defmt::info!("CP15 Generic Timer frequency: {} Hz", unsafe {
        generic_timer::cp15_read_cntfrq()
    });

    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor =
        EXECUTOR.init_with(|| embassy_executor::raw::Executor::new(|_| {}, core::ptr::null_mut()));
    let spawner = executor.spawner();
    spawner.must_spawn(kernel::kmain());

    loop {
        unsafe { executor.poll() }
    }
}
