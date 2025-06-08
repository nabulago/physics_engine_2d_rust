use std::{fmt, ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign, DivAssign}};
use num_traits::Float;

/// 2D vector implementation include functions for
/// Vector related operations
/// Vector Addition, Substrction, Divison, Multiplication 
/// Inplace multiplication MulAssign
/// Inplace vector addition AddAssin
/// Inplace Subsrtraction using SubAssin
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    /// 2D vector with x and y coordinates
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    /// 2D vector with x and y coordinates

    pub fn new(x: f32, y:f32) -> Self {
        // Initiaze a new vector2d object
        Self { x, y }
    }

    pub fn zero() -> Self {
        // Intiaze a new vector with zero values for x and y coordinates
        Self { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        }
        else {
            *self / mag
        }
    }

    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vector2D) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

// Implement arithmetic operations
impl Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
      Self { x: self.x + other.x, y: self.y + other.y }   
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
      Self { x: self.x - other.x, y: self.y - other.y }   
    }
}

// impl Mul<f32> for Vector2D{
//     type Output = Self;
//     fn mul(self, scaler: f32) -> Self {
//         Self { x: self.x * scaler, y: self.y * scaler}
//     }
// }

// impl Div<f32> for Vector2D{
//     type Output = Self;
//     fn div(self, scaler: f32) -> Self {
//         Self { x: self.x / scaler, y: self.y /scaler}
//     }
// }

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
    
}

impl MulAssign for Vector2D {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

//
// === Scalar Operations (generic for Float: f32, f64, etc.) ===
//
impl<T> AddAssign<T> for Vector2D
where
    T: Float + Into<f32>,
{
    fn add_assign(&mut self, scalar: T) {
        let s = scalar.into();
        self.x += s;
        self.y += s;
    }
}

impl<T> SubAssign<T> for Vector2D
where
    T: Float + Into<f32>,
{
    fn sub_assign(&mut self, scalar: T) {
        let s = scalar.into();
        self.x -= s;
        self.y -= s;
    }
}

impl<T> MulAssign<T> for Vector2D
where
    T: Float + Into<f32>,
{
    fn mul_assign(&mut self, scalar: T) {
        let s = scalar.into();
        self.x *= s;
        self.y *= s;
    }
}

impl<T> DivAssign<T> for Vector2D
where
    T: Float + Into<f32>,
{
    fn div_assign(&mut self, scalar: T) {
        let s = scalar.into();
        self.x /= s;
        self.y /= s;
    }
}

//
// === Vector-to-Scalar Binary Ops (Generic) ===
//
impl<T> Add<T> for Vector2D
where
    T: Float + Into<f32>,
{
    type Output = Self;
    fn add(self, scalar: T) -> Self {
        let s = scalar.into();
        Self { x: self.x + s, y: self.y + s }
    }
}

impl<T> Sub<T> for Vector2D
where
    T: Float + Into<f32>,
{
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        let s = scalar.into();
        Self { x: self.x - s, y: self.y - s }
    }
}

impl<T> Mul<T> for Vector2D
where
    T: Float + Into<f32>,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        let s = scalar.into();
        Self { x: self.x * s, y: self.y * s }
    }
}

impl<T> Div<T> for Vector2D
where
    T: Float + Into<f32>,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        let s = scalar.into();
        Self { x: self.x / s, y: self.y / s }
    }
}
impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "Vector (x: {}, y: {})", self.x, self.y)
    }
}
