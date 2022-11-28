enum ClockEvent {
    CLK_RISE,
    CLK_FALL,
    PHI_RISE,
    PHI_FALL,
}

/* The gameboy seems to have 2 relevant clocks, CLK (4.194304 Mhz) and PHI (CLK/4) */
struct Clock {
    /* Elapsed time in nanoseconds */
    elapsed_time: i128,
}

impl Clock {
    /* Advance the clock and get pending events */
    fn advance_time(&self, delta: i128) -> Vec<ClockEvent> {
        elapsed_time = elapsed_time + delta;
        return Vec::new();
    }
}
