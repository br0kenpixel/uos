use core::arch::x86_64::_rdtsc;

pub fn cpu_timestamp_counter() -> u64 {
    unsafe { _rdtsc() }
}
