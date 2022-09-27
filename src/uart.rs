use core::{
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

use pl011::Pl011;

#[defmt::global_logger]
struct Logger;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static TAKEN: AtomicBool = AtomicBool::new(false);
static mut CS_RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();
static mut UART: MaybeUninit<Pl011> = MaybeUninit::uninit();

impl Logger {
    fn do_write(bytes: &[u8]) {
        if !INITIALIZED.load(Ordering::Relaxed) {
            return;
        }

        // SAFETY: we are holding critical section and UART is already
        // initialized
        let uart = unsafe { UART.assume_init_ref() };
        uart.write(bytes);
    }
}

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        // safety: Must be paired with corresponding call to release(), see below
        let restore = unsafe { critical_section::acquire() };

        if TAKEN.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        // no need for CAS because interrupts are disabled
        TAKEN.store(true, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe { CS_RESTORE = restore };

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe { ENCODER.start_frame(Self::do_write) }
    }

    unsafe fn flush() {}

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        ENCODER.end_frame(Self::do_write);

        TAKEN.store(false, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        let restore = CS_RESTORE;

        // safety: Must be paired with corresponding call to acquire(), see above
        critical_section::release(restore);
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        ENCODER.write(bytes, Self::do_write);
    }
}

pub unsafe fn init(addr: usize) {
    critical_section::with(|_cs| {
        if INITIALIZED.load(Ordering::Relaxed) {
            panic!("UART already initialized");
        }

        let uart = pl011::Pl011::new(addr);
        // SAFETY: we are holding critical section
        UART = MaybeUninit::new(uart);
        INITIALIZED.store(true, Ordering::Relaxed);
    });
}
