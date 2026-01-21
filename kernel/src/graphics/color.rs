pub struct RGB { r: u8, g: u8, b: u8 }

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {Self { r, g, b }}
    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }
    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }
    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }
    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
    pub fn to_u32(&self) -> u32 {
        u32::from_le_bytes([self.r, self.g, self.b, 0xFF])
    }
}