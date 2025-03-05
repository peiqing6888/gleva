use crate::editor::Editor;
use crate::font::FontRenderer;
use crate::theme::Theme;
use log::info;
use anyhow::Result;
use gl::*;

// EVA主题颜色
pub const EVA_GREEN: [f32; 3] = [0.0, 0.8, 0.2];
pub const EVA_ORANGE: [f32; 3] = [1.0, 0.5, 0.0];
pub const EVA_PURPLE: [f32; 3] = [0.5, 0.0, 0.5];

pub struct UI {
    font_renderer: FontRenderer,
}

impl UI {
    pub fn new() -> Result<Self> {
        info!("Initializing UiRenderer...");
        
        Ok(Self {
            font_renderer: FontRenderer::new("/System/Library/Fonts/Menlo.ttc", 14)?,
        })
    }

    pub fn render(&mut self, editor: &Editor, theme: &Theme) {
        unsafe {
            // 设置背景颜色
            gl::ClearColor(
                theme.background.0 as f32 / 255.0,
                theme.background.1 as f32 / 255.0,
                theme.background.2 as f32 / 255.0,
                1.0
            );
            
            // 渲染文本
            self.font_renderer.render_text(
                "EVA Editor",
                10.0,
                10.0,
                1.0,
                &[
                    theme.foreground.0 as f32 / 255.0,
                    theme.foreground.1 as f32 / 255.0,
                    theme.foreground.2 as f32 / 255.0,
                    1.0,
                ],
            );
            
            // 渲染编辑器内容
            if let Some(buffer) = editor.get_active_buffer() {
                let mut y = 30.0;
                for line in buffer.content.lines() {
                    self.font_renderer.render_text(
                        line,
                        10.0,
                        y,
                        1.0,
                        &[
                            theme.foreground.0 as f32 / 255.0,
                            theme.foreground.1 as f32 / 255.0,
                            theme.foreground.2 as f32 / 255.0,
                            1.0,
                        ],
                    );
                    y += 20.0;
                }
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
} 