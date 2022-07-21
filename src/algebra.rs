use macroquad::prelude::*;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

impl Point {
    pub fn smaller_x(&self, other: &Point) -> f32 {
        if self.x < other.x {
            return self.x;
        }
        other.x
    }
    pub fn greater_x(&self, other: &Point) -> f32 {
        if self.x > other.x {
            return self.x;
        }
        other.x
    }
    pub fn from_polar(r: f32, theta: f32) -> Point {
        Point {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    pub fn scale(&self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn radius(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotate(&self, ang: f32) -> Point {
        Point::from_polar(self.radius(), self.angle() + ang)
    }

    pub fn reflect_across(&self, other: &Point) -> Point {
        Point::from_polar(self.radius(), 2.0 * other.angle() - self.angle())
    }

}

pub struct LinearEq {
    pub m: f32,
    pub b: f32,
    pub x_lower: f32,
    pub x_upper: f32,
}

impl LinearEq {
    pub fn f(&self, x: f32) -> f32 {
        self.m * x + self.b
    }
    pub fn pt_of_line_intersection(&self, other: &LinearEq) -> Point {
        // ax+b = cx+d
        // ax-cx = d-b
        // x = (d-b)/(a-c)
        Point {
            x: (other.m - self.m) / (self.b - other.b),
            y: self.f((other.m - self.m) / (self.b - other.b)),
        }
    }
    pub fn intersects(&self, other: &LinearEq) -> bool {
        let x = self.pt_of_line_intersection(other).x;
        x > self.x_lower && x < self.x_upper && x > other.x_lower && x < other.x_upper
    }
    pub fn in_range(&self, x: f32) -> bool {
        x > self.x_lower && x < self.x_upper
    }
}