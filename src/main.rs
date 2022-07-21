extern crate core;
use std::time::SystemTime;
use macroquad::prelude::*;
use crate::algebra::*;
use crate::particles::*;
pub mod algebra;
pub mod geometry;
pub mod particles;



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
