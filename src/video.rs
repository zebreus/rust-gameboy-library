use crate::memory::Memory;

use self::tile::Tile;

/// Logic related to tiles
pub mod tile;

struct TileMap {}

/// A collection of functions for video stuff
pub trait VideoFeatures {
    /// Parse all tiles into a vec
    fn get_tile_data(&self) -> Vec<Tile>;
}

impl VideoFeatures for Memory {
    fn get_tile_data(&self) -> Vec<Tile> {
        let video_ram = &self.memory[0x8000..=0x8FFF];
        let chunks = video_ram
            .chunks_exact(16)
            .map(|chunk| Tile::from(chunk.try_into().unwrap()))
            .collect::<Vec<Tile>>();
        return chunks;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cartridge::Cartridge,
        cpu::{instruction::Instruction, Cpu, CpuState},
        memory::Memory,
        video::VideoFeatures,
    };

    #[test]
    fn test_cartridge_can_be_placed_in_memory_and_run() {
        let cartridge = Cartridge::new();
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..100000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            if cpu.read_program_counter() >= 0xc000 {
                // For breakpoint
                let _x = 0;
            }
        }
        let tiles = memory.get_tile_data();
        assert_eq!(tiles.len(), 256);
    }
}
