#![no_std]
#![no_main]

mod requests;
mod graphics;

use x86::halt;

use requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};
use graphics::framebuffer::FramebufferHelper;

use crate::graphics::color::RGB;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            framebuffer.draw_square(25, 5, 5, RGB::new(100, 100, 100));
        }
    }

    loop {}
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { halt() };
    loop {}
}

