/// Represents a Tile.
///
/// A Tile is a 8 pixel by 8 pixel image. Each pixel can be one of four colors. The four different colors are represented by the bytes `0b00`, `0b01`, `0b10` and `0b11`
pub struct Tile {
    /// All pixels of a tile.
    pub pixels: [u8; 64],
}

// macro_rules! color_at {
//     ($lsb:ident, $msb:ident, $position:literal) => {
//         (byteLSB & 0b00000001 ) | ((byteMSB & 0b00000001 ) << 1)
//     };
// }

fn bytes_to_color(bytes: [u8; 2]) -> [u8; 8] {
    let byte_lsb = bytes[0];
    let byte_msb = bytes[1];
    let colors: [u8; 8] = [
        ((byte_lsb >> 7) & 0b00000001) | (((byte_msb >> 7) & 0b00000001) << 1),
        ((byte_lsb >> 6) & 0b00000001) | (((byte_msb >> 6) & 0b00000001) << 1),
        ((byte_lsb >> 5) & 0b00000001) | (((byte_msb >> 5) & 0b00000001) << 1),
        ((byte_lsb >> 4) & 0b00000001) | (((byte_msb >> 4) & 0b00000001) << 1),
        ((byte_lsb >> 3) & 0b00000001) | (((byte_msb >> 3) & 0b00000001) << 1),
        ((byte_lsb >> 2) & 0b00000001) | (((byte_msb >> 2) & 0b00000001) << 1),
        ((byte_lsb >> 1) & 0b00000001) | (((byte_msb >> 1) & 0b00000001) << 1),
        ((byte_lsb >> 0) & 0b00000001) | (((byte_msb >> 0) & 0b00000001) << 1),
    ];
    return colors;
}

impl Tile {
    /// Create a tile from 16 bytes.
    // TODO: Document tile representation
    ///
    /// See pandoc section for tile data <https://gbdev.io/pandocs/Tile_Data.html>
    pub fn from(data: [u8; 16]) -> Tile {
        let pixels_vec: Vec<u8> = data
            .chunks_exact(2)
            .map(|two| bytes_to_color(two.try_into().unwrap()))
            .flatten()
            .collect();
        let pixels: [u8; 64] = pixels_vec.try_into().unwrap();
        return Tile { pixels };
    }
}

#[cfg(test)]
mod tests {
    use super::Tile;

    #[test]
    fn decoding_tile_works() {
        let original: [u8; 16] = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];
        let expected_result: [u8; 64] = [
            0b00, 0b10, 0b11, 0b11, 0b11, 0b11, 0b10, 0b00, 0b00, 0b11, 0b00, 0b00, 0b00, 0b00,
            0b11, 0b00, 0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00, 0b00, 0b11, 0b00, 0b00,
            0b00, 0b00, 0b11, 0b00, 0b00, 0b11, 0b01, 0b11, 0b11, 0b11, 0b11, 0b00, 0b00, 0b01,
            0b01, 0b01, 0b11, 0b01, 0b11, 0b00, 0b00, 0b11, 0b01, 0b11, 0b01, 0b11, 0b10, 0b00,
            0b00, 0b10, 0b11, 0b11, 0b11, 0b10, 0b00, 0b00,
        ];
        let decoded_tile = Tile::from(original);
        assert_eq!(expected_result, decoded_tile.pixels);
    }
}
