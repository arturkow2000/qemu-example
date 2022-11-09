use linked_list_allocator::LockedHeap;

extern "C" {
    fn __heap_start();
    fn __heap_end();
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    let start = __heap_start as usize;
    let end = __heap_end as usize;
    let size = end - start;
    unsafe {
        ALLOCATOR.lock().init(start as *mut u8, size);
    }
}
