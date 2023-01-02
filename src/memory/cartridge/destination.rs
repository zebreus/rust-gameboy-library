/// Whether a version of the game is intended to be sold in Japan or elsewhere.
pub enum Destination {
    /// Japan (and possibly overseas)
    Japan,
    /// Overseas only
    OverseasOnly,
}

impl Into<Destination> for u8 {
    fn into(self) -> Destination {
        match self {
            0 => Destination::Japan,
            1 => Destination::OverseasOnly,
            _ => panic!("Invalid value for the cartridge destination"),
        }
    }
}
