use gtk4::pango::FontDescription;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size_px: f64,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color_rgba: [f64; 4],
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "Cantarell".into(),
            font_size_px: 14.0,
            bold: false,
            italic: false,
            underline: false,
            color_rgba: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl TextStyle {
    pub fn build_font_description(&self) -> FontDescription {
        let mut d = FontDescription::new();
        d.set_family(&self.font_family);
        d.set_absolute_size(self.font_size_px * gtk4::pango::SCALE as f64);
        if self.bold {
            d.set_weight(gtk4::pango::Weight::Bold);
        }
        if self.italic {
            d.set_style(gtk4::pango::Style::Italic);
        }
        d
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WorldPos {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextItem {
    pub id: u64,
    pub pos: WorldPos,
    pub width: f64,
    pub text: String,
    pub style: TextStyle,
    pub align: TextAlign,
    pub underlined: bool, // gespiegelt für Render-Helfer
}

#[derive(Clone, Debug)]
pub enum CanvasItem {
    Text(TextItem),
}

#[derive(Clone, Debug)]
pub struct ViewTransform {
    pub pan: WorldPos,
    pub scale: f64,
}

impl Default for ViewTransform {
    fn default() -> Self {
        Self {
            pan: WorldPos { x: 0.0, y: 0.0 },
            scale: 1.0,
        }
    }
}

impl ViewTransform {
    pub fn screen_to_world(&self, sx: f64, sy: f64) -> WorldPos {
        WorldPos {
            x: self.pan.x + sx / self.scale,
            y: self.pan.y + sy / self.scale,
        }
    }

    pub fn world_to_screen(&self, w: WorldPos) -> (f64, f64) {
        (
            (w.x - self.pan.x) * self.scale,
            (w.y - self.pan.y) * self.scale,
        )
    }
}
