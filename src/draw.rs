use piston_window::{rectangle, types::Color, Context, G2d};

const BLOCK_SIZE: f64 = 20.0;

pub fn to_coord(game_coord: u32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: u32, y: u32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    ctx: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * width as f64,
            BLOCK_SIZE * height as f64,
        ],
        ctx.transform,
        g,
    );
}
