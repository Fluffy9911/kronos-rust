use glfw;
use glfw::{init, Glfw, InitError, WindowHint};
use glfw::ffi::glfwWindowHint;

pub fn create_context() -> Result<Glfw,InitError>{
    let e = init(glfw::fail_on_errors);
    e
}

pub fn create_gl_context(vm:i8, vx:i8,  glfw:&mut Glfw){
unsafe {
    glfw.window_hint(WindowHint::ContextVersion(vx as u32, vm as u32));
    glfw.window_hint(WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
}
}

pub fn create_new_default_context() -> Glfw {
    let mut c = create_context();
    if c.is_ok(){
        let mut g = c.unwrap();
        create_gl_context(3, 3, &mut g);
       g
    }
    else{
        panic!("Failed Trying to create GLFW: {}",c.err().unwrap())
    }
}
#[macro_export]
macro_rules! gl_clear {
    ($rgba:expr) => {{
        let (r, g, b, a) = $rgba;

        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }};
}