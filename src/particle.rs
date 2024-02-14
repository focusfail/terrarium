use rand::Rng;

use crate::particle_handler::ParticleHandler;

#[allow(unused_variables)]
pub trait Particle {
    fn update(handler: &ParticleHandler, new: &mut Vec<(u8, (u8, u8, u8))>, index: usize);
    fn var_color() -> (u8, u8, u8);
}

pub struct Sand {
    pub id: u8,
}

impl Particle for Sand {
    fn update(handler: &ParticleHandler, new: &mut Vec<(u8, (u8, u8, u8))>, index: usize) {
        let (x, y) = handler.i2c(index);
        let particle = handler.particle_at(index);

        if !(y + 1 < handler.height) {
            new[index] = particle;
            return;
        } else if handler.particle_at(handler.c2i(x, y + 1)).0 == 0 {
            new[handler.c2i(x, y + 1)] = particle;
            return;
        }

        let mut dir: isize = 1;
        let change_dir: bool = rand::thread_rng().gen();

        if change_dir {
            dir *= -1;
        }

        let spot_a = x as isize - dir;
        let spot_b = x as isize + dir;

        if spot_a >= 0
            && spot_a < handler.width as isize
            && handler.particle_at(handler.c2i(spot_a as usize, y)).0 == 0
            && handler.particle_at(handler.c2i(spot_a as usize, y + 1)).0 == 0
        {
            let new_spot = handler.c2i(spot_a as usize, y + 1);
            new[new_spot] = particle;
        } else if spot_b >= 0
            && spot_b < handler.width as isize
            && handler.particle_at(handler.c2i(spot_b as usize, y)).0 == 0
            && handler.particle_at(handler.c2i(spot_b as usize, y + 1)).0 == 0
        {
            let new_spot = handler.c2i(spot_b as usize, y + 1);
            new[new_spot] = particle;
        } else {
            new[index] = particle;
        }
    }

    fn var_color() -> (u8, u8, u8) {
        let colors = vec![(182, 160, 113), (220, 193, 136), (255, 230, 157)];
        let rng = rand::thread_rng().gen_range(0..colors.len());

        colors[rng]
    }
}

pub struct Stone {
    pub id: u8,
}

impl Particle for Stone {
    fn update(handler: &ParticleHandler, new: &mut Vec<(u8, (u8, u8, u8))>, index: usize) {
        new[index] = handler.particle_at(index);
    }

    fn var_color() -> (u8, u8, u8) {
        let colors = vec![(65, 65, 65), (60, 60, 60), (75, 75, 75)];
        let rng = rand::thread_rng().gen_range(0..colors.len());

        colors[rng]
    }
}

pub struct Water {
    pub id: u8,
}

impl Particle for Water {
    fn update(handler: &ParticleHandler, new: &mut Vec<(u8, (u8, u8, u8))>, index: usize) {
        // TODO: CUSTOM WATER UPDATES
        let (x, y) = handler.i2c(index);
        let particle = handler.particle_at(index);

        if !(y + 1 < handler.height) {
            new[index] = particle;
            return;
        } else if handler.particle_at(handler.c2i(x, y + 1)).0 == 0 {
            new[handler.c2i(x, y + 1)] = particle;
            return;
        }

        let mut dir: isize = 1;
        let change_dir: bool = rand::thread_rng().gen();

        if change_dir {
            dir *= -1;
        }

        let spot_a = x as isize - dir;
        let spot_b = x as isize + dir;

        if spot_a >= 0
            && spot_a < handler.width as isize
            && handler.particle_at(handler.c2i(spot_a as usize, y)).0 == 0
            && handler.particle_at(handler.c2i(spot_a as usize, y + 1)).0 == 0
        {
            let new_spot = handler.c2i(spot_a as usize, y + 1);
            new[new_spot] = particle;
        } else if spot_b >= 0
            && spot_b < handler.width as isize
            && handler.particle_at(handler.c2i(spot_b as usize, y)).0 == 0
            && handler.particle_at(handler.c2i(spot_b as usize, y + 1)).0 == 0
        {
            let new_spot = handler.c2i(spot_b as usize, y + 1);
            new[new_spot] = particle;
        } else {
            new[index] = particle;
        }
    }

    fn var_color() -> (u8, u8, u8) {
        let colors = vec![(28, 163, 236), (21, 127, 185), (28, 175, 255)];
        let rng = rand::thread_rng().gen_range(0..colors.len());

        colors[rng]
    }
}
