use crate::algebra::*;
use macroquad::prelude::*;

#[derive(PartialEq)]
pub struct Vector {
    a: Point,
    b: Point,
}

impl Vector {
    pub fn angle(&self) -> f32 {
        (self.b - self.a).angle()
    }
    pub fn slope(&self) -> Option<f32> {
        if self.b.x == self.a.x {
            return None;
        }
        let d = self.b - self.a;
        Some(d.y / d.x)
    }
    pub fn intercept(&self) -> Option<f32> {
        let slope = self.slope();
        slope?;
        Some(self.a.y - slope.unwrap() * self.a.x)
    }
    pub fn to_lin_eq(&self) -> Option<LinearEq> {
        let slope = self.slope();
        slope?;
        Some(LinearEq {
            m: slope.unwrap(),
            b: self.intercept().unwrap(),
            x_lower: self.a.smaller_x(&self.b),
            x_upper: self.a.greater_x(&self.b),
        })
    }
    pub fn pt_touching(&self, other: &Vector) -> Option<Point> {
        if self.a == other.a {
            return Some(self.a);
        }
        if self.a == other.b {
            return Some(self.a);
        }
        if self.b == other.a {
            return Some(self.b);
        }
        if self.b == other.b {
            return Some(self.b);
        }
        None
    }
    pub fn is_parallel(&self, other: &Vector) -> bool {
        self.angle() == other.angle()
    }
    pub fn pt_of_intersection(&self, other: &Vector) -> Option<Point> {
        if self.pt_touching(other).is_some() {
            return self.pt_touching(other);
        }
        if self.is_parallel(other) {
            return None;
        }
        let self_eq = self.to_lin_eq();
        let other_eq = other.to_lin_eq();
        if self_eq.is_none() {
            return Some(Point {
                x: self.a.x,
                y: other.to_lin_eq().unwrap().f(self.a.x),
            });
        }
        if other_eq.is_none() {
            return Some(Point {
                x: other.a.x,
                y: self.to_lin_eq().unwrap().f(other.a.x),
            });
        }
        Some(self_eq.unwrap().pt_of_line_intersection(&other_eq.unwrap()))
    }
}

#[derive(Clone, Copy)]
pub struct Poly {
    pub center: Point,
    pub size: f32,
    pub color_position: f32,
    pub color_strength: f32,
    pub sides: u8,
}

impl Poly {
    pub fn new(x: f32, y: f32, size: f32, color_position: f32, color_strength: f32, sides: u8) -> Poly {
        Poly {
            center: Point { x, y },
            size,
            color_position,
            color_strength,
            sides,
        }
    }
    pub fn sine_map(num: f32) -> f32 {
        num.sin() / 2.0 + 0.5
    }
    pub fn d_theta(&self) -> f32 {
        2.0 * std::f32::consts::PI / self.sides as f32
    }
    pub fn ang_at_pos(&self, pos: i32) -> f32 {
        pos as f32 * self.d_theta() % (2.0 * std::f32::consts::PI)
    }
    pub fn unit_circle(&self) -> Vec<Point> {
        Vec::from_iter((0..self.sides).map(|x| Point::from_polar(1.0, self.ang_at_pos(x as i32))))
    }
    pub fn to_vecs(&self) -> Vec<Vector> {
        let circle = self.unit_circle();
        Vec::from_iter(circle.into_iter().map(|x| Vector {
            a: x.scale(self.size),
            b: x.rotate(self.d_theta()).scale(self.size),
        }))
    }
    pub fn draw(&self) {
        draw_poly(
            self.center.x,
            self.center.y,
            self.sides,
            self.size,
            0.0,
            Color::new(
                self.color_strength * Poly::sine_map(self.color_position),
                self.color_strength
                    * Poly::sine_map(self.color_position + 2.0 / 3.0 * std::f32::consts::PI),
                self.color_strength
                    * Poly::sine_map(self.color_position + 4.0 / 3.0 * std::f32::consts::PI),
                100.0,
            ),
        );
    }
}
