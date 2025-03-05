use std::path::PathBuf;
use anyhow::Result;
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::ThemeSet;

pub struct Buffer {
    pub content: String,
    pub file_path: Option<PathBuf>,
    pub syntax: Option<&'static SyntaxReference>,
    pub modified: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            content: String::new(),
            file_path: None,
            syntax: None,
            modified: false,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Buffer {
            content,
            file_path: Some(path),
            syntax: None,
            modified: false,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.file_path {
            std::fs::write(path, &self.content)?;
            self.modified = false;
        }
        Ok(())
    }

    pub fn insert(&mut self, position: usize, text: &str) {
        if position <= self.content.len() {
            self.content.insert_str(position, text);
            self.modified = true;
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        if start < end && end <= self.content.len() {
            self.content.replace_range(start..end, "");
            self.modified = true;
        }
    }
}

pub struct Cursor {
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: 0,
            line: 0,
            column: 0,
        }
    }
}

pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub active_buffer: usize,
    pub cursor: Cursor,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buffers: vec![Buffer::new()],
            active_buffer: 0,
            cursor: Cursor::default(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let buffer = Buffer::from_file(path)?;
        self.buffers.push(buffer);
        self.active_buffer = self.buffers.len() - 1;
        Ok(())
    }

    pub fn get_active_buffer(&self) -> Option<&Buffer> {
        self.buffers.get(self.active_buffer)
    }

    pub fn get_active_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.buffers.get_mut(self.active_buffer)
    }

    pub fn move_cursor(&mut self, position: usize) {
        // 首先获取必要的信息
        let (content_len, content) = if let Some(buffer) = self.get_active_buffer() {
            (buffer.content.len(), buffer.content.clone())
        } else {
            return;
        };

        if position > content_len {
            return;
        }

        // 计算行和列
        let content_before_cursor = &content[..position];
        let line_count = content_before_cursor.matches('\n').count();
        let column = if let Some(last_newline) = content_before_cursor.rfind('\n') {
            content_before_cursor.len() - last_newline - 1
        } else {
            content_before_cursor.len()
        };

        // 更新光标位置
        self.cursor.position = position;
        self.cursor.line = line_count;
        self.cursor.column = column;
    }

    pub fn insert_text(&mut self, text: &str) {
        let position = self.cursor.position;
        if let Some(buffer) = self.get_active_buffer_mut() {
            let old_len = buffer.content.len();
            buffer.insert(position, text);
            // 只有在实际插入了文本时才移动光标
            if buffer.content.len() > old_len {
                self.move_cursor(position + text.len());
            }
        }
    }

    pub fn delete_text(&mut self, count: usize) {
        let position = self.cursor.position;
        let start = position.saturating_sub(count);
        if let Some(buffer) = self.get_active_buffer_mut() {
            buffer.delete(start, position);
            self.move_cursor(start);
        }
    }

    pub fn save_active_buffer(&mut self) -> Result<()> {
        if let Some(buffer) = self.get_active_buffer_mut() {
            buffer.save()?;
        }
        Ok(())
    }
} 