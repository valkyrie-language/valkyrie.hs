use crate::Color;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    /// The x-coordinate of the upper left point
    pub x: f32,
    /// The y-coordinate of the upper left point
    pub y: f32,
}

/// A rectangle
#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    /// The x-coordinate of the upper left point
    pub x: f32,
    /// The y-coordinate of the upper left point
    pub y: f32,
    /// The width of the rectangle
    pub width: f32,
    /// The height of the rectangle
    pub height: f32,
    pub color: Color,
}

/// A ellipse
#[derive(Copy, Clone, Debug)]
pub struct Ellipse {
    /// The x-coordinate of the upper left point
    pub x: f32,
    /// The y-coordinate of the upper left point
    pub y: f32,
    /// The width of the rectangle
    pub width: f32,
    /// The height of the rectangle
    pub height: f32,
    pub color: Color,
}
