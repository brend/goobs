use macroquad::prelude::*;

#[macroquad::main("goobs")]
async fn main() {
    let world = World {
        palette: Palette {
            background: Color::new(0.1, 0.1, 0.15, 1.0),
            goob: LIGHTGRAY,
        },
        goobs: vec![Goob {
            pos: Pos { x: 100.0, y: 200.0 },
            siz: Siz { w: 5.0, h: 5.0 },
        }],
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

struct Siz {
    w: f32,
    h: f32,
}

struct Goob {
    pos: Pos,
    siz: Siz,
}

struct World {
    goobs: Vec<Goob>,
    palette: Palette,
}
