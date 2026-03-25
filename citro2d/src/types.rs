//! Types

/// A 2D point in space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    /// Depth, between 0..=1.0.
    pub z: f32,
}

impl Point {
    /// Create a new point from (x, y, z) coordinates.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a new point from (x, y) coordinates with a depth of 0.
    pub const fn new_no_z(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl From<Point> for (f32, f32, f32) {
    fn from(val: Point) -> Self {
        (val.x, val.y, val.z)
    }
}

impl From<Point> for (f32, f32) {
    fn from(val: Point) -> Self {
        (val.x, val.y)
    }
}

/// Size of a 2D object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

impl From<Size> for (f32, f32) {
    fn from(val: Size) -> Self {
        (val.width, val.height)
    }
}

/// A color for citro2d functions.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Color encoded as ABGR.
    pub(crate) inner: u32,
}

impl Color {
    /// Create a new color with the given RGB values. Alpha is set to 255 (fully opaque).
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_with_alpha(r, g, b, 255)
    }

    /// Create a new color with the given RGBA values.
    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        let inner = u32::from_be_bytes([a, b, g, r]);
        Self { inner }
    }

    /// Get the inner ABGR color, as needed for citro2d functions.
    pub fn inner(&self) -> u32 {
        self.inner
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::new_with_alpha(r, g, b, a)
    }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        Color { inner: val }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.inner
    }
}

/// A gradient color for rendering shapes.
#[repr(C)]
pub struct GradientColor {
    pub top_l: Color,
    pub top_r: Color,
    pub bot_l: Color,
    pub bot_r: Color,
}

// todo: rotation?
/// A bounding box of an object, storing the top left coordinates and its dimensions.
pub struct Bounding {
    top_left: Point,
    size: Size,
}

impl Bounding {
    /* Getters */

    pub const fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn top_right(&self) -> Point {
        self.top_left + (self.size.width, 0.).into()
    }

    pub fn bottom_left(&self) -> Point {
        self.top_left + (0., self.size.height).into()
    }

    pub fn bottom_right(&self) -> Point {
        self.top_left + (self.size.width, self.size.height).into()
    }

    pub fn center(&self) -> Point {
        self.top_left + (self.size.width / 2., self.size.height / 2.).into()
    }

    pub const fn size(&self) -> Size {
        self.size
    }

    pub const fn width(&self) -> f32 {
        self.size.width
    }

    pub const fn height(&self) -> f32 {
        self.size.height
    }

    /* Builder */

    pub const fn with_top_left(point: Point, size: Size) -> Self {
        let top_left = point;
        Self { top_left, size }
    }

    pub fn with_top_right(point: Point, size: Size) -> Self {
        let top_left = point - (size.width, 0.).into();
        Self { top_left, size }
    }

    pub fn with_bottom_left(point: Point, size: Size) -> Self {
        let top_left = point - (0., size.height).into();
        Self { top_left, size }
    }

    pub fn with_bottom_right(point: Point, size: Size) -> Self {
        let top_left = point - (size.width, size.height).into();
        Self { top_left, size }
    }

    pub fn with_center(point: Point, size: Size) -> Self {
        let top_left = point - (size.width / 2., size.height / 2.).into();
        Self { top_left, size }
    }

    /* Setters */

    pub fn set_top_left(&mut self, point: Point) {
        *self = Self::with_top_left(point, self.size)
    }

    pub fn set_top_right(&mut self, point: Point) {
        *self = Self::with_top_right(point, self.size)
    }

    pub fn set_bottom_left(&mut self, point: Point) {
        *self = Self::with_bottom_left(point, self.size)
    }

    pub fn set_bottom_right(&mut self, point: Point) {
        *self = Self::with_bottom_right(point, self.size)
    }

    pub fn set_center(&mut self, point: Point) {
        *self = Self::with_center(point, self.size)
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_width(&mut self, width: f32) {
        self.size.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.size.height = height;
    }
}
