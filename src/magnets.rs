// use num_traits::Zero;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

pub mod burlich;
pub mod lines;
pub mod sheets;
pub mod solenoid;
use crate::NAN;

// Traits:
pub trait DotProduct {
    type Output;
    fn dot(&self, other: &Self) -> Self::Output;
}

pub trait CrossProduct {
    type Output;
    fn cross(&self, other: &Self) -> Self::Output;
}

// pub trait Magnet2D {}

// Structs:
#[derive(Copy, Clone, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
pub struct PointArray2<T>(Vec<Point2<T>>);

// pub type

#[derive(Copy, Clone, PartialEq)]
pub struct Prism {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub center: Point3<f64>,
}

enum MagnetKind {
    Cuboid,
    Cube,
    Rectangle,
    Square,
    Cylinder,
}

// Methods
//Display
impl<T> fmt::Display for Point2<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


impl fmt::Display for Vector2
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> fmt::Display for Point3<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// impl<T> fmt::Display for PointArray2<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// Dot product
impl<T> DotProduct for Point2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = T;

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T> DotProduct for Point3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = T;

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Simple Maths
impl<T: Add<Output = T>> Add for Point2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point2<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Point2<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Div<Output = T>> Div for Point3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
