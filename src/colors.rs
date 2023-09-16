use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Color = Color {
    red: 0.,
    green: 0.,
    blue: 0.,
};

pub const GREEN: Color = Color {
    red: 0.,
    green: 1.,
    blue: 0.,
};

pub const RED: Color = Color {
    red: 1.,
    green: 0.,
    blue: 0.,
};

pub const BLUE: Color = Color {
    red: 0.,
    green: 0.,
    blue: 1.,
};

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([
            ((self.red) * 255.) as u8,
            ((self.green) * 255.) as u8,
            ((self.blue) * 255.) as u8,
            0,
        ])
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            ((self.red) * 255.) as u8,
            ((self.green) * 255.) as u8,
            ((self.blue) * 255.) as u8,
            255 as u8,
        ]
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
}
