mod turtle;

use std::f64;
use turtle::Turtle;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("no window found")?
        .document()
        .ok_or("No document found")?;

    let canvas = document
        .get_element_by_id("turtlemap")
        .ok_or("no canvas element found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "canvas couldn't be converted")?;

    let context = canvas
        .get_context("2d")
        .map_err(|_| "canvas 2d context not found")?
        .ok_or("canvas 2d context not found...")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "context couldn't be converted")?;

    canvas.set_width(800);
    canvas.set_height(800);

    let mut bob = Turtle::default_with_context(context);

    bob.teleport(400.0 - 120.0, 400.0 + 200.0);
    draw_snowflake(&mut bob, 600.0, 150.0, 32.0);

    bob.teleport(400.0 - 50.0, 400.0 + 50.0);
    draw_snowflake(&mut bob, 200.0, 50.0, 8.0);

    bob.teleport(400.0 - 25.0, 400.0);
    draw_snowflake(&mut bob, 60.0, 15.0, 2.0);

    bob.teleport(400.0 - 18.0, 400.0 - 15.0);
    draw_snowflake(&mut bob, 15.0, 3.75, 0.0);

    Ok(())
}

fn draw_snowflake(bob: &mut Turtle, long: f64, short: f64, gap: f64) {
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

fn draw_lambda(bob: &mut Turtle, long: f64, short: f64) {
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

fn draw_filled_again(bob: &mut Turtle) {
    bob.teleport(400.0, 400.0);
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
fn draw_square(bob: &mut Turtle) {
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
