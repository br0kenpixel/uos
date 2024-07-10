use super::kernel::KernelAllocator;
use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    mem::MaybeUninit,
    ptr::NonNull,
};
use spinning_top::Spinlock;

pub struct OsAllocator(Spinlock<MaybeUninit<KernelAllocator>>);

impl OsAllocator {
    pub const fn uninit() -> Self {
        Self(Spinlock::new(MaybeUninit::uninit()))
    }

    pub fn init(&self, allocator: KernelAllocator) {
        *self.0.lock() = MaybeUninit::new(allocator);
    }
}

#[allow(clippy::significant_drop_tightening)]
unsafe impl GlobalAlloc for OsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let guard = self.0.lock();
        let allocator = guard.assume_init_ref();
        let ptr = allocator.allocate(layout).unwrap();
        let ptr = ptr.as_ptr();

        ptr.cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let guard = self.0.lock();
        let allocator = guard.assume_init_ref();

        allocator.deallocate(NonNull::new_unchecked(ptr), layout);
    }
}
