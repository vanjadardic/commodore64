pub struct Memory {
    data: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 0xFFFF]
        }
    }
}