use std::path::PathBuf;
use glfw::{Action, Context, Key, WindowEvent};
use log::{info, error, debug};
use anyhow::Result;
use std::time::Instant;
use std::sync::mpsc::Receiver;

use crate::editor::Editor;
use crate::renderer::Renderer;
use crate::theme::Theme;

mod editor;
mod font;
mod renderer;
mod shader;
mod theme;
mod ui;

// OpenGL bindings
pub use gl::types::*;
pub use gl::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn init_gl() -> Result<(glfw::Glfw, glfw::Window, Receiver<(f64, WindowEvent)>)> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw.window_hint(glfw::WindowHint::ContextVersion(2, 1));

    let (mut window, events) = glfw.create_window(800, 600, "Eva Editor", glfw::WindowMode::Windowed)
        .ok_or_else(|| anyhow::anyhow!("Failed to create GLFW window."))?;

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    Ok((glfw, window, events))
}

fn main() -> Result<()> {
    env_logger::init();

    let (mut glfw, mut window, events) = init_gl()?;

    let mut editor = Editor::new();
    let mut renderer = Renderer::new(800, 600)?;

    let start_time = Instant::now();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                    renderer.resize(width as u32, height as u32);
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.render(&editor);

        window.swap_buffers();
    }

    Ok(())
}
