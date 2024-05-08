use crate::memreg_ex::MemoryRegionEx;
use bootloader_api::info::MemoryRegion;
use core::{mem::MaybeUninit, ops::Deref, ptr::NonNull};

#[derive(Clone, Copy)]
#[repr(packed(1))]
pub struct AllocEntry {
    empty: bool,
    start: MaybeUninit<u64>,
    end: MaybeUninit<u64>,
}

impl AllocEntry {
    pub const EMPTY: Self = Self {
        empty: true,
        start: MaybeUninit::zeroed(),
        end: MaybeUninit::zeroed(),
    };

    pub const fn is_empty(&self) -> bool {
        self.empty
    }

    pub const fn is_some(&self) -> bool {
        !self.empty
    }
}

impl From<MemoryRegion> for AllocEntry {
    fn from(value: MemoryRegion) -> Self {
        Self {
            empty: false,
            start: MaybeUninit::new(value.start),
            end: MaybeUninit::new(value.end),
        }
    }
}

impl MemoryRegionEx for AllocEntry {
    fn size(&self) -> usize {
        assert!(!self.empty);

        (unsafe { self.end.assume_init() - self.start.assume_init() }) as usize
    }

    fn into_ptr(self) -> NonNull<u8> {
        unimplemented!()
    }
}
