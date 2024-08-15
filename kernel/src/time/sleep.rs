pub fn sleep_cycles(cycles: u64) {
    let current = super::cputs::cpu_timestamp_counter();
    let end = current + cycles;

    while super::cputs::cpu_timestamp_counter() < end {}
}
