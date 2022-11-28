enum ClockEvent {
    ClkRise,
    ClkFall,
    PhiRise,
    PhiFall,
}

/* The gameboy seems to have 2 relevant clocks, CLK (4.194304 Mhz) and PHI (CLK/4) */
struct Clock {
    /* Elapsed time in nanoseconds */
    elapsed_time: u128,
}

impl Clock {
    /* Advance the clock and get pending events */
    fn advance_time(&mut self, delta: u128) -> Vec<ClockEvent> {
        self.elapsed_time = self.elapsed_time + delta;
        return Vec::new();
    }
}
