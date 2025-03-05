// EVA主题颜色定义
pub struct EvaTheme {
    pub background: [f32; 4],
    pub foreground: [f32; 4],
    pub accent: [f32; 4],
    pub warning: [f32; 4],
    pub error: [f32; 4],
    pub success: [f32; 4],
    pub grid: [f32; 4],
    pub selection: [f32; 4],
}

impl Default for EvaTheme {
    fn default() -> Self {
        EvaTheme {
            // NERV深色背景
            background: [0.05, 0.05, 0.08, 1.0],
            // 主要文本颜色
            foreground: [0.9, 0.9, 0.9, 1.0],
            // EVA-01紫色
            accent: [0.4, 0.0, 0.6, 1.0],
            // 警告信息橙色
            warning: [1.0, 0.5, 0.0, 1.0],
            // 错误信息红色
            error: [0.8, 0.0, 0.0, 1.0],
            // 成功信息绿色
            success: [0.0, 0.8, 0.2, 1.0],
            // 网格线颜色
            grid: [0.2, 0.8, 0.2, 0.3],
            // 选中文本背景色
            selection: [0.3, 0.0, 0.4, 0.4],
        }
    }
}

// 语法高亮主题
pub struct SyntaxTheme {
    pub keywords: [f32; 4],
    pub functions: [f32; 4],
    pub strings: [f32; 4],
    pub numbers: [f32; 4],
    pub comments: [f32; 4],
    pub types: [f32; 4],
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        SyntaxTheme {
            // 关键字使用EVA-01紫色
            keywords: [0.6, 0.0, 0.8, 1.0],
            // 函数名使用明亮的绿色
            functions: [0.0, 0.9, 0.3, 1.0],
            // 字符串使用橙色
            strings: [1.0, 0.6, 0.0, 1.0],
            // 数字使用蓝色
            numbers: [0.0, 0.7, 1.0, 1.0],
            // 注释使用暗灰色
            comments: [0.5, 0.5, 0.5, 1.0],
            // 类型名使用青色
            types: [0.0, 0.8, 0.8, 1.0],
        }
    }
}

// 字体设置
pub struct FontConfig {
    pub family: String,
    pub size: f32,
    pub line_height: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            family: "Source Code Pro".to_string(),
            size: 14.0,
            line_height: 1.5,
        }
    }
}

// UI元素尺寸
pub struct Metrics {
    pub line_numbers_width: f32,
    pub scroll_bar_width: f32,
    pub tab_height: f32,
    pub status_bar_height: f32,
    pub padding: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics {
            line_numbers_width: 50.0,
            scroll_bar_width: 12.0,
            tab_height: 32.0,
            status_bar_height: 24.0,
            padding: 8.0,
        }
    }
}

// 完整主题配置
pub struct Theme {
    pub colors: EvaTheme,
    pub syntax: SyntaxTheme,
    pub font: FontConfig,
    pub metrics: Metrics,
    pub background: (u8, u8, u8),
    pub foreground: (u8, u8, u8),
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            colors: EvaTheme::default(),
            syntax: SyntaxTheme::default(),
            font: FontConfig::default(),
            metrics: Metrics::default(),
            background: (30, 30, 40),
            foreground: (220, 220, 220),
        }
    }
} 