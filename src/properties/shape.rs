use crate::colors::TRANSPARENT;
use crate::Color;

#[derive(Copy, Clone, Default)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }
    pub fn origin() -> Self {
        Pos::new(0.0, 0.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }
    pub fn zero() -> Self {
        Size::new(0.0, 0.0)
    }
}

/// Describes a new visual rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rect {
    //// X position of the rectangle.
    pub x: f32,

    /// Y position of the rectangle.
    pub y: f32,

    /// Width of the rectangle.
    pub width: f32,

    /// Height of the rectangle.
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle with the given parameters.
    pub fn new(origin: Pos, size: Size) -> Self {
        Rect {
            x: origin.x,
            y: origin.y,
            width: size.width,
            height: size.height,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Background {
    pub color: Color,
}

impl Background {
    pub fn transparent() -> Self {
        Background { color: TRANSPARENT }
    }
}

#[derive(Copy, Clone)]
pub struct Border {
    pub width: u32,
    pub radius: u32,
    pub color: Color,
}

impl Border {
    pub fn new(width: u32, radius: u32, color: Color) -> Self {
        Border {
            width,
            radius,
            color,
        }
    }
    pub fn none() -> Self {
        Border::new(0, 0, TRANSPARENT)
    }
}
