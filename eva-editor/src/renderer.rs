use crate::theme::Theme;
use crate::ui::UI;
use crate::editor::Editor;
use crate::font::FontRenderer;
use anyhow::Result;
use std::time::Instant;
use gl::*;

pub struct Renderer {
    ui: UI,
    font_renderer: FontRenderer,
    theme: Theme,
    start_time: Instant,
    window_width: u32,
    window_height: u32,
}

impl Renderer {
    pub fn new(window_width: u32, window_height: u32) -> Result<Self> {
        Ok(Renderer {
            ui: UI::new()?,
            font_renderer: FontRenderer::new("/System/Library/Fonts/Menlo.ttc", 14)?,
            theme: Theme::default(),
            start_time: Instant::now(),
            window_width,
            window_height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        self.ui.resize(width as i32, height as i32);
    }

    pub fn render(&mut self, editor: &Editor) {
        // 清除屏幕
        unsafe {
            gl::ClearColor(
                self.theme.colors.background[0],
                self.theme.colors.background[1],
                self.theme.colors.background[2],
                self.theme.colors.background[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // 渲染 UI
        self.ui.render(editor, &self.theme);
    }
}

// 实现资源清理
impl Drop for Renderer {
    fn drop(&mut self) {
        // OpenGL资源清理会通过各个组件的Drop实现自动完成
    }
} 