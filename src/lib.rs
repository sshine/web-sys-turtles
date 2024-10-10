mod canvas;
mod hexagon;
mod position;
mod turtle;

use hexagon::Hexagon;
use position::Position;
use std::f64;
use turtle::{CanvasTurtle, Turtle as _};
use wasm_bindgen::prelude::*;

pub type WasmResult<T> = Result<T, wasm_bindgen::JsValue>;

#[wasm_bindgen(start)]
fn start() -> WasmResult<()> {
    let (canvas, context) = canvas::get_canvas_by_id("turtlemap")?;

    let dim = 800;
    canvas.set_width(dim);
    canvas.set_height(dim);

    let mut bob = CanvasTurtle::default_with_context(context);
    let big_hex = Hexagon {
        position: Position::new(53.5898, 200.0),
        angle: bob.angle(),
        side: 400.0,
    };

    bob.teleport(Position::new(400.0, 400.0), big_hex.angle);
    draw_snowflake(&mut bob, 400.0, 100.0, 16.0);
    bob.fill_color(None);
    bob.line_color("#aa0000");
    draw_more_nested_hexagons(&mut bob, big_hex);

    Ok(())
}

fn draw_more_nested_hexagons(bob: &mut CanvasTurtle, big_hex: Hexagon) {
    bob.left(30.0);

    // xpadding: (800 - 400 * cos(30) * 2) / 2 = 53.5898
    // y = 800/4

    bob.teleport(big_hex.position, big_hex.angle);
    big_hex.draw(bob);
    // big_hex.draw_children(&mut bob);

    let children = big_hex.children(&bob);
    for child_hex in children {
        bob.teleport(child_hex.position, child_hex.angle);
        child_hex.draw(bob);
        for grandchild_hex in child_hex.children(&bob) {
            bob.teleport(grandchild_hex.position, grandchild_hex.angle);
            grandchild_hex.draw(bob);
        }
    }
}

fn draw_nested_hexagons(bob: &mut CanvasTurtle) {
    bob.left(30.0);

    // xpadding: (800 - 400 * cos(30) * 2) / 2 = 53.5898
    // y = 800/4

    let big_hex = Hexagon {
        position: Position::new(53.5898, 200.0),
        angle: bob.angle(),
        side: 400.0,
    };
    bob.teleport(big_hex.position, big_hex.angle);
    big_hex.draw(bob);
    // big_hex.draw_children(&mut bob);

    let children = big_hex.children(&bob);
    for child_hex in children {
        bob.teleport(child_hex.position, child_hex.angle);
        child_hex.draw(bob);
    }
}

fn draw_nested_snowflakes(bob: &mut CanvasTurtle) {
    bob.teleport(Position::new(400.0 - 120.0, 400.0 + 200.0), bob.angle());
    draw_snowflake(bob, 600.0, 150.0, 32.0);

    bob.teleport(Position::new(400.0 - 50.0, 400.0 + 50.0), bob.angle());
    draw_snowflake(bob, 200.0, 50.0, 8.0);

    bob.teleport(Position::new(400.0 - 25.0, 400.0), bob.angle());
    draw_snowflake(bob, 60.0, 15.0, 2.0);

    bob.teleport(Position::new(400.0 - 18.0, 400.0 - 15.0), bob.angle());
    draw_snowflake(bob, 15.0, 3.75, 0.0);
}

fn draw_snowflake(bob: &mut CanvasTurtle, long: f64, short: f64, gap: f64) {
    const COLORS: [&str; 2] = ["#7EBAE4", "#5277C3"];
    for i in 0..6 {
        bob.fill_color(Some(COLORS[i % COLORS.len()].to_string()));
        bob.line_color(COLORS[i % COLORS.len()]);
        draw_lambda(bob, long, short);
        bob.forward(short * 1.5);
        bob.left(60.0);
        bob.forward(gap);
    }
}

fn draw_lambda(bob: &mut CanvasTurtle, long: f64, short: f64) {
    bob.pen_down();

    bob.forward(long);
    bob.right(120.0);
    bob.forward(short);
    bob.right(60.0);
    bob.forward(short);
    bob.left(120.0);
    bob.forward(short);
    bob.right(60.0);
    bob.forward(short / 2.0);
    bob.right(60.0);
    bob.forward(short / 2.0);
    bob.right(60.0);
    bob.forward(short * 1.5);
    bob.left(60.0);
    bob.forward(short);
    bob.right(60.0);
    bob.forward(short);
    bob.right(120.0);

    bob.pen_up();
}

fn draw_filled_again(bob: &mut CanvasTurtle) {
    bob.teleport(Position::new(400.0, 400.0), bob.angle());
    bob.pen_down();
    for _ in 0..4 {
        bob.forward(40.0);
        bob.left(90.0);
    }
    bob.fill_color(Some("black".to_string()));
    bob.pen_up();
}

#[allow(dead_code)]
fn draw_filled(context: &web_sys::CanvasRenderingContext2d) {
    context.set_stroke_style(&JsValue::from_str("blue"));
    context.set_line_width(2.0);

    context.begin_path();

    context.move_to(400.0, 400.0);
    context.line_to(400.0, 500.0);
    context.line_to(500.0, 500.0);
    // context.line_to(500.0, 400.0);
    // context.line_to(400.0, 400.0);
    context.close_path();

    context.set_fill_style(&JsValue::from_str("red"));
    context.fill();
    context.stroke();
}

#[allow(dead_code)]
fn draw_smiley(context: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .map_err(|_| "draw outer circle")?;

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context
        .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
        .map_err(|_| "draw mouth")?;

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .map_err(|_| "draw left eye")?;

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .map_err(|_| "draw right eye")?;

    context.stroke();

    Ok(())
}

#[allow(dead_code)]
fn draw_square(bob: &mut CanvasTurtle) {
    bob.forward(10.0);
    bob.right(90.0);
    bob.forward(10.0);
    bob.left(90.0);

    bob.pen_down();
    for _ in 0..4 {
        bob.forward(100.0);
        bob.right(90.0);
    }
    bob.pen_up();
}
