/// The phases of an instruction with two phases
#[derive(Debug)]
pub enum TwoPhases {
    /// First phase
    First,
    /// Second phase
    Second,
}

/// The phases of an instruction with three phases
#[derive(Debug)]
pub enum ThreePhases {
    /// First phase
    First,
    /// Second phase
    Second,
    /// Third phase
    Third,
}

/// The phases of an instruction with four phases
#[derive(Debug)]
pub enum FourPhases {
    /// First phase
    First,
    /// Second phase
    Second,
    /// Third phase
    Third,
    /// Fourth phase
    Fourth,
}
