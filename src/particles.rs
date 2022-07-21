use crate::geometry::*;
use crate::algebra::*;
use ::rand::thread_rng;
use ::rand::Rng;
use macroquad::window;

pub struct BBox {
    pub x_lower: f32,
    pub x_upper: f32,
    pub y_lower: f32,
    pub y_upper: f32,
}

impl BBox {
    pub fn check(&self, particle: &mut Particle) {
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
    pub poly: Poly,
    pub velocity: Point,
    pub life: i32,
    pub speed: f32,
}

impl Particle {
    pub fn new(
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
    pub fn draw(&self) {
        self.poly.draw();
    }
    pub fn color_tick(&mut self) {
        self.poly.color_position += 0.01;
    }
    pub fn pos_tick(&mut self) {
        self.poly.center = self.poly.center + self.velocity;
        // println!("{}", self.quad.center);
    }
    pub fn velo_tick(&mut self) {
        let mut rng = thread_rng();
        self.velocity.x += self.speed * (0.04 * rng.gen::<f32>() - 0.02);
        self.velocity.y += self.speed * (0.04 * rng.gen::<f32>() - 0.02);
    }
    pub fn collision_tick(&mut self, others: Vec<Particle>) {
        // for p in others{
        // }
    }

    pub fn tick(&mut self) {
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
    pub count: i32,
    pub lifetime: i32,
    pub center: Point,
    pub size: f32,
    pub radius: f32,
    pub particles: Vec<Particle>,
    pub sides: u8,
    pub speed: f32,
}

impl Emitter {
    pub fn new(
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
    pub fn fill(&mut self) {
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

    pub fn tick(&mut self) {
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
