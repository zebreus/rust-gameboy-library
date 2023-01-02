/// The trait for things that are serial connections
pub trait SerialConnection {
    /// Send and receive a bit.
    ///
    /// Should return true, if there is no data source available
    fn exchange_bit(&mut self, send: bool) -> bool;
}

impl SerialConnection for LoggerSerialConnection {
    fn exchange_bit(&mut self, send: bool) -> bool {
        self.received_byte = (self.received_byte << 1) | (if send { 1 } else { 0 });
        self.received_bits += 1;
        if self.received_bits < 8 {
            return true;
        }
        let character = self.received_byte as char;
        match character {
            '\n' => {
                println!("Serial: {}", self.current_line);
                self.current_line = String::new();
            }
            _ => {
                self.current_line.push(character);
            }
        };

        self.received_bits = 0;
        self.received_byte = 0;
        return true;
    }
}

/// A serial connection that logs everything to console.
pub struct LoggerSerialConnection {
    received_byte: u8,
    received_bits: usize,
    current_line: String,
}

impl LoggerSerialConnection {
    /// Create a new logger
    pub fn new() -> LoggerSerialConnection {
        LoggerSerialConnection {
            received_byte: 0,
            received_bits: 0,
            current_line: String::new(),
        }
    }
}

/// A serial connection that executes a closure on every line
pub struct LineBasedConnection<'a> {
    handler: &'a mut dyn FnMut(&String) -> (),
    received_byte: u8,
    received_bits: usize,
    current_line: String,
}

impl<'a> LineBasedConnection<'a> {
    /// Create a new line based connection
    pub fn new(handler: &'a mut dyn FnMut(&String) -> ()) -> LineBasedConnection<'a> {
        return LineBasedConnection {
            handler: handler,
            received_byte: 0,
            received_bits: 0,
            current_line: String::new(),
        };
    }
}

impl<'a> SerialConnection for LineBasedConnection<'a> {
    fn exchange_bit(&mut self, send: bool) -> bool {
        self.received_byte = (self.received_byte << 1) | (if send { 1 } else { 0 });
        self.received_bits += 1;
        if self.received_bits < 8 {
            return true;
        }
        let character = self.received_byte as char;
        match character {
            '\n' => {
                (self.handler)(&self.current_line);
                self.current_line = String::new();
            }
            _ => {
                self.current_line.push(character);
            }
        };

        self.received_bits = 0;
        self.received_byte = 0;
        return true;
    }
}
