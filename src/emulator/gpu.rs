use crate::emulator::memory::Memory;

pub struct Gpu {
    pub current_raster_line: usize,
    pub x_pos: usize,
    pub display: [[u8; 320]; 200],
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            current_raster_line: 0,
            x_pos: 0,
            display: [[0; 320]; 200],
        }
    }

    pub fn tick(&mut self, memory: &Memory) {
        let vic_bank = memory.cia2().get_vic_bank();
        let matrix_address = memory.gpu().get_video_matrix_address() | vic_bank;
        let char_address = memory.gpu().get_character_bitmap_address() | vic_bank;
        if (char_address >= 0x1000 && char_address < 0x2000) || (char_address >= 0x9000 && char_address < 0xA000) {}

        let color = memory.color_ram().get((self.current_raster_line >> 3) * 40 + self.x_pos);
        let character = memory.get_from_gpu(matrix_address as usize + (self.current_raster_line >> 3) * 40 + self.x_pos);
        let character_slice = memory.get_from_gpu(char_address as usize + character as usize * 8 + (self.current_raster_line & 0x07));
        for bit in 0usize..8 {
            self.display[self.current_raster_line][self.x_pos * 8 + 7 - bit] = if (character_slice >> bit) & 0x01 == 0x01 {
                color
            } else {
                memory.gpu().background_color()
            };
        }
        self.x_pos += 1;
        if self.x_pos == 40 {
            self.x_pos = 0;
            self.current_raster_line += 1;
            if self.current_raster_line == 200 {
                self.current_raster_line = 0;
            }
        }
    }
}