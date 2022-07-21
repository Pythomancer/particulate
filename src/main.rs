extern crate core;

use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::prelude::*;
use macroquad::window;
use std::fmt;
use std::ops::{Add, Sub};
use std::time::SystemTime;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
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
    fn smaller_x(&self, other: &Point) -> f32 {
        if self.x < other.x {
            return self.x;
        }
        other.x
    }
    fn greater_x(&self, other: &Point) -> f32 {
        if self.x > other.x {
            return self.x;
        }
        other.x
    }
    fn from_polar(r: f32, theta: f32) -> Point {
        Point {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    fn scale(&self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn radius(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    fn rotate(&self, ang: f32) -> Point {
        Point::from_polar(self.radius(), self.angle() + ang)
    }

    fn reflect_across(&self, other: &Point) -> Point {
        Point::from_polar(self.radius(), 2.0 * other.angle() - self.angle())
    }

}

pub struct LinearEq {
    m: f32,
    b: f32,
    x_lower: f32,
    x_upper: f32,
}

impl LinearEq {
    fn f(&self, x: f32) -> f32 {
        self.m * x + self.b
    }
    fn pt_of_line_intersection(&self, other: &LinearEq) -> Point {
        // ax+b = cx+d
        // ax-cx = d-b
        // x = (d-b)/(a-c)
        Point {
            x: (other.m - self.m) / (self.b - other.b),
            y: self.f((other.m - self.m) / (self.b - other.b)),
        }
    }
    fn intersects(&self, other: &LinearEq) -> bool {
        let x = self.pt_of_line_intersection(other).x;
        x > self.x_lower && x < self.x_upper && x > other.x_lower && x < other.x_upper
    }
    fn in_range(&self, x: f32) -> bool {
        x > self.x_lower && x < self.x_upper
    }
}

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

pub struct Poly {
    center: Point,
    size: f32,
    color_position: f32,
    color_strength: f32,
    sides: u8,
}

impl Poly {
    fn new(x: f32, y: f32, size: f32, color_position: f32, color_strength: f32, sides: u8) -> Poly {
        Poly {
            center: Point { x, y },
            size,
            color_position,
            color_strength,
            sides,
        }
    }
    fn sine_map(num: f32) -> f32 {
        num.sin() / 2.0 + 0.5
    }
    fn d_theta(&self) -> f32 {
        2.0 * std::f32::consts::PI / self.sides as f32
    }
    fn ang_at_pos(&self, pos: i32) -> f32 {
        pos as f32 * self.d_theta() % (2.0 * std::f32::consts::PI)
    }
    fn unit_circle(&self) -> Vec<Point> {
        Vec::from_iter((0..self.sides).map(|x| Point::from_polar(1.0, self.ang_at_pos(x as i32))))
    }
    fn to_vec(&self) -> Vec<Vector> {
        let circle = self.unit_circle();
        Vec::from_iter(circle.into_iter().map(|x| Vector {
            a: x.scale(self.size),
            b: x.rotate(self.d_theta()).scale(self.size),
        }))
    }
    fn draw(&self) {
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

pub struct BBox {
    x_lower: f32,
    x_upper: f32,
    y_lower: f32,
    y_upper: f32,
}

impl BBox {
    fn check(&self, particle: &mut Particle) {
        if particle.poly.center.x < self.x_lower {
            particle.poly.center.x = self.x_lower;
            particle.velocity.x *= -1.0;
        }
        if particle.poly.center.x > self.x_upper {
            particle.poly.center.x = self.x_upper;
            particle.velocity.x *= -1.0;
        }
        if particle.poly.center.y < self.y_lower {
            particle.poly.center.y = self.y_lower;
            particle.velocity.y *= -1.0;
        }
        if particle.poly.center.y > self.y_upper {
            particle.poly.center.y = self.y_upper;
            particle.velocity.y *= -1.0;
        }
    }
}

pub struct Particle {
    poly: Poly,
    velocity: Point,
    life: i32,
    speed: f32,
}

impl Particle {
    fn new(
        x: f32,
        y: f32,
        size: f32,
        color_position: f32,
        color_strength: f32,
        sides: u8,
        speed: f32,
    ) -> Particle {
        Particle {
            poly: Poly::new(x, y, size, color_position, color_strength, sides),
            velocity: Point { x: 0.0, y: 0.0 },
            life: 0,
            speed,
        }
    }
    fn draw(&self) {
        self.poly.draw();
    }
    fn color_tick(&mut self) {
        self.poly.color_position += 0.01;
    }
    fn pos_tick(&mut self) {
        self.poly.center = self.poly.center + self.velocity;
        // println!("{}", self.quad.center);
    }
    fn velo_tick(&mut self) {
        let mut rng = thread_rng();
        self.velocity.x += self.speed * (0.04 * rng.gen::<f32>() - 0.02);
        self.velocity.y += self.speed * (0.04 * rng.gen::<f32>() - 0.02);
    }
    fn collision_tick(&mut self, others: Vec<Particle>) {
        // for p in others{
        // }
    }

    fn tick(&mut self) {
        self.color_tick();
        self.velo_tick();
        self.pos_tick();
        BBox {
            x_lower: 0.0,
            x_upper: window::screen_width(),
            y_lower: 0.0,
            y_upper: window::screen_height(),
        }
        .check(self);
        self.life += 1;
    }
}

pub struct Emitter {
    count: i32,
    lifetime: i32,
    center: Point,
    size: f32,
    radius: f32,
    particles: Vec<Particle>,
    sides: u8,
    speed: f32,
}

impl Emitter {
    fn new(
        count: i32,
        lifetime: i32,
        center: Point,
        size: f32,
        radius: f32,
        sides: u8,
        speed: f32,
    ) -> Self {
        Emitter {
            count,
            lifetime,
            center,
            size,
            radius,
            particles: vec![],
            sides,
            speed,
        }
    }
    fn fill(&mut self) {
        let mut rng = thread_rng();
        while self.particles.len() < self.count as usize {
            let ang = 2.0 * std::f32::consts::PI * rng.gen::<f32>();
            let r = self.radius * rng.gen::<f32>();
            self.particles.push(Particle::new(
                self.center.x + ang.cos() * r,
                self.center.y + ang.sin() * r,
                self.size,
                2.0 * std::f32::consts::PI * rng.gen::<f32>(),
                rng.gen(),
                self.sides,
                self.speed,
            ));
        }
    }

    fn tick(&mut self) {
        for p in self.particles.iter_mut() {
            p.tick();
            p.draw();
        }
        self.particles
            .retain(|x| x.life < self.lifetime || self.lifetime < 0);
        if self.particles.len() < self.count as usize {
            self.fill();
        }
    }
}

#[macroquad::main("Particles")]
async fn main() {
    // let mut angle: f32 = 40.0;
    let mut time = SystemTime::now();
    // let mut p: Particle = Particle::new(300.0, 300.0);
    let mut e = Emitter::new(16, -1, Point { x: 200.0, y: 200.0 }, 32.0, 100.0, 3, 4.0);
    e.fill();
    loop {
        clear_background(WHITE);
        // p.tick();
        // p.draw();
        e.tick();
        let t: u128 = SystemTime::now()
            .duration_since(time)
            .expect("time reversed")
            .as_millis();
        draw_text(
            &(1000.0 / {
                if t > 0 {
                    t as f32
                } else {
                    0.0001
                }
            })
            .round()
            .to_string(),
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );
        time = SystemTime::now();
        next_frame().await;
    }
}
