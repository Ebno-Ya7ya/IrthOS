use crate::graphics::color::RGB;

pub trait FramebufferHelper {
    fn draw_pixel(&self, x: usize, y: usize);
    fn draw_pixel_colored(&self, x: usize, y: usize, color: u32);
    fn draw_square(&self, x: usize, y: usize, size: usize, color: RGB);
}

impl FramebufferHelper for limine::framebuffer::Framebuffer<'_> {
    fn draw_pixel(&self, x: usize, y: usize) {
        self.draw_pixel_colored(x, y, RGB::white().to_u32());
    }
    fn draw_pixel_colored(&self, x: usize, y: usize, color: u32) {
        let ptr = self.addr();
        let pitch = self.pitch() as usize;
        let bytepp = self.bpp() as usize / 8;

        let offset = x * bytepp + y * pitch;

        unsafe { *(ptr.add(offset) as *mut u32) = color };        
    }
    fn draw_square(&self, x: usize, y: usize, size: usize, color: RGB) {
        for x_offset in 0..size {
            for y_offset in 0..size {
                self.draw_pixel_colored(x + x_offset, y + y_offset, color.to_u32());
            }
        }
    }
}