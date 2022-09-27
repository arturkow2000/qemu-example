use core::{
    cell::SyncUnsafeCell,
    mem::{transmute, MaybeUninit},
    sync::atomic::{AtomicBool, Ordering},
};

use fdt::Fdt;
use gicv2::{GicType, GicV2};

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static GIC: SyncUnsafeCell<MaybeUninit<GicV2<'static>>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());

fn locate_gic<'a>(fdt: &'a Fdt) -> fdt::node::FdtNode<'a, 'a> {
    let prop = fdt
        .root()
        .property("interrupt-parent")
        .expect("/interrupt-parent property is missing");
    let handle = prop.as_usize().unwrap() as u32;
    fdt.find_phandle(handle).unwrap()
}

fn irq_handler() {
    self::get().handle_irq(|irq| {
        defmt::debug!("Hello from IRQ handler, irq={}", irq);
    })
}

pub unsafe fn init(fdt: &Fdt) {
    INITIALIZED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .expect("IRQ controller already initialized");

    let node = locate_gic(&fdt);
    if !node
        .compatible()
        .unwrap()
        .all()
        .any(|x| x == "arm,cortex-a15-gic")
    {
        defmt::panic!("No compatible interrupt controller found");
    }

    let mut reg = node.reg().unwrap();
    let distributor = reg.next().unwrap();
    let cpu = reg.next().unwrap();

    defmt::info!(
        "GIC distributor @ 0x{:08X}",
        distributor.starting_address as usize
    );
    defmt::info!(
        "GIC CPU interface @ 0x{:08X}",
        cpu.starting_address as usize
    );

    let gic = GicV2::new(
        distributor.starting_address as usize,
        cpu.starting_address as usize,
        GicType::CortexA15,
    );
    GIC.get().write(MaybeUninit::new(gic));
    cortex_a_rt::interrupt::install_irq_handler(irq_handler);
    cortex_a_rt::interrupt::enable();
}

pub fn get() -> &'static GicV2<'static> {
    if !INITIALIZED.load(Ordering::Acquire) {
        panic!("IRQ controller not initialized");
    }

    unsafe { transmute(GIC.get() as *const _) }
}
