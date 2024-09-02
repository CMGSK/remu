use crate::gpu::gpu::{GPU, VRAM_BEGIN, VRAM_END, VRAM_SIZE};
pub struct MemoryBus{
    pub gpu: GPU,
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, addr: u16) -> u8{
        let address = addr as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram( address - VRAM_BEGIN )
            }
            _ => { self.memory[address] }
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let address = addr as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                 self.gpu.write_vram( address - VRAM_BEGIN, byte )
            }
            _ => {}
        }
    }

}
