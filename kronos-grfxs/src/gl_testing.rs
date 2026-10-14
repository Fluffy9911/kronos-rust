use glfw::Context;
use crate::{gl_clear, grfxs};
use crate::col::cols;
use crate::grfxs::CoreContext;

pub fn main(){
    let mut context = CoreContext::new_def();

    let (mut window, events) =
        context.create_win(1280, 720, "My OpenGL Window");

    while !window.should_close() {
        context.glfw.poll_events();



        unsafe {
        gl_clear!(cols::Col8::RED.as_vals());
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}