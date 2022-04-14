//! pixpipe (Pixel Pipeline)

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLUE: Color = Color::new(0, 0, 170, 255);
    pub const GREEN: Color = Color::new(0, 170, 0, 255);
    pub const CYAN: Color = Color::new(0, 170, 170, 255);
    pub const RED: Color = Color::new(170, 0, 0, 255);
    pub const MAGENTA: Color = Color::new(170, 0, 170, 255);
    pub const YELLOW: Color = Color::new(170, 85, 0, 255);
    pub const WHITE: Color = Color::new(170, 170, 170, 255);
    pub const DARK_GRAY: Color = Color::new(85, 85, 85, 255);
    pub const BRIGHT_BLUE: Color = Color::new(85, 85, 255, 255);
    pub const BRIGHT_GREEN: Color = Color::new(85, 255, 85, 255);
    pub const BRIGHT_CYAN: Color = Color::new(85, 255, 255, 255);
    pub const BRIGHT_RED: Color = Color::new(255, 85, 85, 255);
    pub const BRIGHT_MAGENTA: Color = Color::new(255, 85, 255, 255);
    pub const BRIGHT_YELLOW: Color = Color::new(255, 255, 85, 255);
    pub const BRIGHT_WHITE: Color = Color::new(255, 255, 255, 255);
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub struct PixBuf {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl PixBuf {
    pub fn with_dimensions(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![Color::default(); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_slice(&self) -> &[Color] {
        self.data.as_slice()
    }

    pub fn as_raw_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *mut u8,
                (self.width * self.height) as usize * std::mem::size_of::<Color>(),
            )
        }
    }

    pub fn set(&mut self, left: u32, top: u32, color: Color) {
        if let Some(c) = self.data.get_mut((top * self.width + left) as usize) {
            *c = color
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.data.fill(color);
    }
}
