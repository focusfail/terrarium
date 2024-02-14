#![allow(dead_code)]

use std::cmp::{max, min};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::TimerSubsystem;
use terrarium::particle_handler::ParticleHandler;

struct Brush {
    pub particle: u8,
    pub size: usize,
}

struct Game {
    canvas: Canvas<Window>,
    particle_size: u32,
    running: bool,
    ctx: sdl2::Sdl,
    handler: ParticleHandler,
    frametime: u32,
    timer: TimerSubsystem,
    last_pos: (usize, usize),
    brush: Brush,
    elapsed: f64,
    frames: u32,
}

impl Game {
    pub fn new(resolution: [u32; 2], particle_size: usize) -> Self {
        let ctx = sdl2::init().unwrap();
        let video_subsystem = ctx.video().unwrap();

        let window = video_subsystem
            .window("SandGame", resolution[0], resolution[1])
            .position_centered()
            .build()
            .unwrap();
        let timer = ctx.timer().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let particle_handler = ParticleHandler::new(resolution, particle_size);
        let frametime = timer.ticks();

        Self {
            canvas,
            particle_size: particle_size as u32,
            running: true,
            ctx,
            handler: particle_handler,
            frametime,
            timer,
            last_pos: (0, 0),
            brush: Brush {
                particle: 1,
                size: 4,
            },
            elapsed: 0.0,
            frames: 0,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.handle_events();
            self.update();
            self.render();
            self.timer.delay((1000.0 / 60.0) as u32);
        }
    }

    fn handle_events(&mut self) {
        let mut event_pump = self.ctx.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    match keycode {
                        Some(Keycode::Escape) => {
                            self.handler.clear();
                        }
                        _ => (),
                    };

                    match keymod {
                        Mod::LSHIFTMOD => (),
                        _ => (),
                    }
                }
                Event::MouseWheel { y, .. } => {
                    let new_width;
                    if self.brush.size == 1 {
                        new_width = self.brush.size as i32 + y;
                    } else {
                        new_width = self.brush.size as i32 + y * 2;
                    }
                    self.brush.size = min(max(1, new_width), 40) as usize;
                }
                _ => {}
            }
        }
        let mouse = sdl2::mouse::MouseState::new(&event_pump);

        let scaled_x = min(max(mouse.x(), 0), 800 - 1) as usize;
        let scaled_y = min(max(mouse.y(), 0), 600 - 1) as usize;

        self.handler.freeze = false;

        if mouse.left() {
            self.brush.particle = 1;
        } else if mouse.right() {
            self.brush.particle = 2;
        } else if mouse.middle() {
            self.brush.particle = 0;
            self.handler.freeze = true;
        } else {
            self.last_pos = (scaled_x, scaled_y);
            return;
        }

        self.handler.spawn_line(
            self.last_pos.0,
            self.last_pos.1,
            scaled_x,
            scaled_y,
            self.brush.particle,
            self.brush.size,
        );
        self.last_pos = (scaled_x, scaled_y);
    }

    fn show_mouse_indicator(&mut self) {
        let half_width = ((self.brush.size / 2) * self.particle_size as usize) as u32;

        let x = {
            let x = self.last_pos.0 - half_width as usize;
            (x / self.particle_size as usize) as usize * self.particle_size as usize
        };

        let y = {
            let y = self.last_pos.1 - half_width as usize;
            (y / self.particle_size as usize) as usize * self.particle_size as usize
        };

        let rect = Rect::new(x as i32, y as i32, half_width * 2, half_width * 2);

        self.canvas.set_draw_color((255, 255, 255));
        let _ = self.canvas.draw_rect(rect);
    }

    fn update(&mut self) {
        if !self.handler.freeze {
            self.handler.update(&mut self.canvas);
        }
    }

    fn render(&mut self) {
        let curr = self.timer.ticks();
        let dt = curr - self.frametime;
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.clear();

        self.handler.draw(&mut self.canvas);
        self.show_mouse_indicator();

        self.canvas.present();
        self.frametime = curr;

        self.elapsed += dt as f64;
        self.frames += 1;

        if self.elapsed >= 1000.0 {
            let fps = self.frames as f64 / (self.elapsed / 1000.0);

            let _ = self
                .canvas
                .window_mut()
                .set_title(&format!("SandGame | FPS: {:.2}", fps));

            self.frames = 0;
            self.elapsed = 0.0;
        }
    }
}

fn main() {
    let mut game = Game::new([800, 600], 4);
    game.run();
}
