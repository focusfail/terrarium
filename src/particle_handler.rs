#![allow(dead_code, unused_variables)]

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::particle::{self, Particle};

pub struct ParticleHandler {
    collision: Vec<(u8, (u8, u8, u8))>,
    pub width: usize,
    pub height: usize,
    particle_size: usize,
    pub freeze: bool,
}

impl ParticleHandler {
    pub fn new(res: [u32; 2], particle_size: usize) -> Self {
        let len = (res[0] * res[1]) as usize;
        let collision = vec![(0, (0, 0, 0)); len];
        let width = (res[0] as usize / particle_size) as usize;
        let height = (res[1] as usize / particle_size) as usize;

        Self {
            collision,
            width,
            height,
            particle_size,
            freeze: false,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for i in 0..self.collision.len() {
            let (part, color) = self.particle_at(i);

            if part == 0 {
                continue;
            }
            let rnd: f32 = rand::random();

            let rect = Rect::new(
                (i % self.width * self.particle_size) as i32,
                (i / self.width * self.particle_size) as i32,
                self.particle_size as u32,
                self.particle_size as u32,
            );
            canvas.set_draw_color(color);
            let _ = canvas.fill_rect(rect);
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        if self.freeze {
            return;
        }
        let mut new = vec![(0u8, (0u8, 0u8, 0u8)); self.collision.len()];

        for index in (0..self.collision.len()).rev() {
            let (part, _) = self.particle_at(index);

            if part == 0 {
                continue;
            } else if part == 1 {
                particle::Sand::update(&self, &mut new, index);
            } else if part == 2 {
                particle::Stone::update(&self, &mut new, index)
            }
        }
        self.collision = new;
    }

    pub fn clear(&mut self) {
        println!("Clear");
        self.collision = vec![(0u8, (0u8, 0u8, 0u8)); self.collision.len()];
    }

    pub fn particle_at(&self, index: usize) -> (u8, (u8, u8, u8)) {
        self.collision[index]
    }

    pub fn spawn_particle_width(&mut self, particle: u8, x: usize, y: usize, width: usize) {
        let (sx, sy) = (x / self.particle_size, y / self.particle_size);
        let half = width / 2;

        // TODO: THIS YOU CAN DO BETTER !

        for i in 0..width {
            if sx + i - half > self.width {
                continue;
            }

            for j in 0..width {
                let index = self.c2i(sx + i - half, sy + j - half);
                if index > self.collision.len() {
                    continue;
                }

                let color = Self::color_by_particle(particle);

                if self.collision[index].0 == particle {
                    continue;
                }

                self.collision[index] = (particle, color);
            }
        }
    }

    pub fn spawn_particle(&mut self, particle: u8, x: usize, y: usize) {
        let (sx, sy) = (x / self.particle_size, y / self.particle_size);
        let color = Self::color_by_particle(particle);
        let index = self.c2i(sx, sy);
        self.collision[index] = (particle, color);
    }

    pub fn color_by_particle(particle: u8) -> (u8, u8, u8) {
        match particle {
            1 => particle::Sand::var_color(),
            2 => particle::Stone::var_color(),
            3 => particle::Water::var_color(),
            _ => (0, 0, 0),
        }
    }

    pub fn spawn_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        particle: u8,
        width: usize,
    ) {
        let dist = f64::sqrt((x2 - x1).pow(2) as f64 + (y2 - y1).pow(2) as f64) as usize;

        self.spawn_particle_width(particle, x1, y1, width);

        for i in 0..dist {
            // TODO: TRY Steps in wdith
            if dist - 1 < 1 {
                return;
            }
            let t = i as f64 / (dist - 1) as f64;

            let ix = (x1 as f64 + t * (x2 as f64 - x1 as f64)).round() as usize;
            let iy = (y1 as f64 + t * (y2 as f64 - y1 as f64)).round() as usize;

            self.spawn_particle_width(particle, ix, iy, width);
        }
    }

    pub fn c2i(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn i2c(&self, index: usize) -> (usize, usize) {
        ((index % self.width) as usize, (index / self.width) as usize)
    }
}
