use macroquad::prelude::*;

#[macroquad::main("goobs")]
async fn main() {
    let world = World {
        palette: Palette {
            background: Color::new(0.1, 0.1, 0.15, 1.0),
            goob: LIGHTGRAY,
        },
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
            world.palette.goob,
        );
    }
}

struct Palette {
    background: Color,
    goob: Color,
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

struct Goob {
    pos: Pos,
    siz: Siz,
}

impl Goob {
    fn random() -> Goob {
        Goob {
            pos: Pos::random(),
            siz: Siz::random(),
        }
    }
}

struct World {
    goobs: Vec<Goob>,
    palette: Palette,
}
