use color::Color;
use fyrox::core::color;
use lite_macro::lite_api;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
#[lite_api(class=Color)]
pub struct LiteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<LiteColor> for Color {
    fn from(value: LiteColor) -> Self {
        Color::from_rgba(value.r, value.g, value.b, value.a)
    }
}
