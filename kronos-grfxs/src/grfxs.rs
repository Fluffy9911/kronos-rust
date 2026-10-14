use glfw::{Glfw, PWindow, GlfwReceiver, WindowEvent, Context};

use crate::gl_helpers::helpers;

pub struct CoreContext {
    pub glfw: Glfw,
}

impl CoreContext {
    pub fn new(glfw: Glfw) -> Self {
        Self { glfw }
    }

    pub fn new_def() -> Self {
        Self::new(helpers::create_new_default_context())
    }
pub fn glfw(&self)-> &Glfw{
    &self.glfw
}
    pub fn glmut(&mut self) ->  &Glfw{
    &self.glfw
}
    pub fn create_win(
        &mut self,
        width: usize,
        height: usize,
        name: impl ToString,
    ) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        let (mut window, events) = self.glfw
            .create_window(
                width as u32,
                height as u32,
                &name.to_string(),
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();

        // Load OpenGL functions after creating the context.
        gl::load_with(|symbol| {
            window
                .get_proc_address(symbol)
                .map_or(std::ptr::null(), |f| f as *const _)
        });

        window.set_all_polling(true);

        (window, events)
    }
}