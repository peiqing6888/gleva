use std::collections::HashMap;
use freetype::Library;
use anyhow::Result;
use log::error;
use freetype::face::LoadFlag;

// 字形信息
pub struct Character {
    pub texture_id: u32,
    pub size: (i32, i32),     // 宽度和高度
    pub bearing: (i32, i32),  // 基准点到字形左边和顶边的距离
    pub advance: u32,         // 到下一个字形的水平距离
}

pub struct FontRenderer {
    characters: HashMap<char, Character>,
    vbo: u32,
    vao: u32,
}

impl FontRenderer {
    pub fn new(font_path: &str, font_size: u32) -> Result<Self> {
        let ft = Library::init()?;
        let face = ft.new_face(font_path, 0)?;
        face.set_pixel_sizes(0, font_size)?;

        let mut characters = HashMap::new();
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (6 * 4 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE as u8,
                (4 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE as u8,
                (4 * std::mem::size_of::<f32>()) as i32,
                (2 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        for c in 32u32..128 {
            face.load_char(c as usize, LoadFlag::RENDER)?;
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();

            let mut texture_id = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    bitmap.width(),
                    bitmap.rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    bitmap.buffer().as_ptr() as *const _,
                );

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }

            if let Some(ch) = char::from_u32(c) {
                characters.insert(
                    ch,
                    Character {
                        texture_id,
                        size: (bitmap.width(), bitmap.rows()),
                        bearing: (glyph.bitmap_left(), glyph.bitmap_top()),
                        advance: (glyph.advance().x >> 6) as u32,
                    },
                );
            }
        }

        Ok(FontRenderer { characters, vbo, vao })
    }

    pub fn render_text(&mut self, text: &str, x: f32, y: f32, scale: f32, color: &[f32; 4]) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let mut x_pos = x;

            for c in text.chars() {
                if let Some(ch) = self.characters.get(&c) {
                    let x_pos_f = x_pos;
                    let y_pos_f = y + (ch.bearing.1 as f32 * scale);

                    let w = ch.size.0 as f32 * scale;
                    let h = ch.size.1 as f32 * scale;

                    let vertices: [f32; 24] = [
                        // pos      // tex
                        x_pos_f,     y_pos_f - h, 0.0, 1.0,
                        x_pos_f + w, y_pos_f - h, 1.0, 1.0,
                        x_pos_f + w, y_pos_f,     1.0, 0.0,
                        x_pos_f,     y_pos_f - h, 0.0, 1.0,
                        x_pos_f + w, y_pos_f,     1.0, 0.0,
                        x_pos_f,     y_pos_f,     0.0, 0.0,
                    ];

                    gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
                    gl::BindVertexArray(self.vao);

                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        (vertices.len() * std::mem::size_of::<f32>()) as isize,
                        vertices.as_ptr() as *const _,
                    );

                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::BindVertexArray(0);

                    x_pos += ch.advance as f32 * scale;
                }
            }

            gl::Disable(gl::BLEND);
        }
    }
}

impl Drop for FontRenderer {
    fn drop(&mut self) {
        unsafe {
            for character in self.characters.values() {
                gl::DeleteTextures(1, &character.texture_id);
            }
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
} 