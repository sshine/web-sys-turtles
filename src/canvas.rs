use crate::WasmResult;

use wasm_bindgen::JsCast as _;

pub fn get_canvas_by_id(
    id: &str,
) -> WasmResult<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let document = web_sys::window()
        .ok_or("no window found")?
        .document()
        .ok_or("No document found")?;

    let canvas = document
        .get_element_by_id(id)
        .ok_or("no canvas element found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "canvas couldn't be converted")?;

    let context = canvas
        .get_context("2d")
        .map_err(|_| "canvas 2d context not found")?
        .ok_or("canvas 2d context not found...")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "context couldn't be converted")?;

    Ok((canvas, context))
}
