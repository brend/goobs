use macroquad::prelude::*;

#[macroquad::main("goobs")]
async fn main() {
    let world = World {
        palette: Palette::default(),
        goobs: (0..7).map(|_| Goob::random()).collect(),
    };

    loop {
        clear_background(world.palette.background);

        draw_world(&world);

        next_frame().await
    }
}

fn draw_world(world: &World) {
    for goob in &world.goobs {
        draw_ellipse(
            goob.pos.x,
            goob.pos.y,
            goob.siz.w,
            goob.siz.h,
            0.0,
            goob_color(goob, &world.palette),
        );
    }
}

fn goob_color(goob: &Goob, palette: &Palette) -> Color {
    match goob.health_level() {
        HealthLevel::Healthy => palette.goob_healthy,
        HealthLevel::Groggy => palette.goob_groggy,
        HealthLevel::Dying => palette.goob_dying,
    }
}

struct Palette {
    background: Color,
    goob_healthy: Color,
    goob_groggy: Color,
    goob_dying: Color,
}

impl Default for Palette {
    fn default() -> Palette {
        Palette {
            background: Color::new(0.1, 0.1, 0.15, 1.0),
            goob_healthy: WHITE,
            goob_groggy: LIGHTGRAY,
            goob_dying: DARKGRAY,
        }
    }
}

struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn random() -> Pos {
        Pos {
            x: 20.0 + (rand::rand() as f32 % screen_width() - 40.0),
            y: 20.0 + (rand::rand() as f32 % screen_height() - 40.0),
        }
    }
}

struct Siz {
    w: f32,
    h: f32,
}

impl Siz {
    fn random() -> Siz {
        let w = 2.0 + (rand::rand() as f32 % 7.0);
        let h = w;
        Siz { w, h }
    }
}

enum HealthLevel {
    Healthy,
    Groggy,
    Dying,
}

struct Health {
    value: u32,
}

impl Health {
    fn random() -> Health {
        Health { value: 100 }
    }
}

struct Goob {
    pos: Pos,
    siz: Siz,
    health: Health,
}

impl Goob {
    fn random() -> Goob {
        Goob {
            pos: Pos::random(),
            siz: Siz::random(),
            health: Health::random(),
        }
    }

    fn health_level(&self) -> HealthLevel {
        match self.health.value {
            0..20 => HealthLevel::Dying,
            20..80 => HealthLevel::Groggy,
            _ => HealthLevel::Healthy,
        }
    }
}

struct World {
    goobs: Vec<Goob>,
    palette: Palette,
}
