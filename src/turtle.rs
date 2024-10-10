use crate::Position;
use wasm_bindgen::JsValue;

pub trait Turtle {
    fn forward(&mut self, distance: f64);
    fn left(&mut self, angle: f64);
    fn right(&mut self, angle: f64);
    fn teleport(&mut self, position: Position, angle: f64);
    fn position(&self) -> Position;
    fn angle(&self) -> f64;
    fn ghost(&self) -> GhostTurtle {
        GhostTurtle {
            position: self.position(),
            angle: self.angle(),
        }
    }
}
pub struct CanvasTurtle {
    position: Position,
    angle: f64,
    is_drawing: bool,
    fill_color: Option<String>,
    context: web_sys::CanvasRenderingContext2d,
}

impl Turtle for CanvasTurtle {
    fn forward(&mut self, distance: f64) {
        let Position { x: old_x, y: old_y } = self.position;
        let new_x = old_x + distance * self.angle.cos();
        let new_y = old_y + distance * self.angle.sin();
        self.context.line_to(new_x, new_y);
        if self.is_drawing {
            self.context.stroke();
        }
        self.position = Position { x: new_x, y: new_y };
    }

    fn teleport(&mut self, position: Position, angle: f64) {
        self.context.move_to(position.x, position.y);
        self.position = position;
        self.angle = angle;
    }

    fn left(&mut self, degrees: f64) {
        self.angle -= degrees * std::f64::consts::PI / 180.0;
    }

    fn right(&mut self, degrees: f64) {
        self.angle += degrees * std::f64::consts::PI / 180.0;
    }

    fn position(&self) -> Position {
        self.position
    }

    fn angle(&self) -> f64 {
        self.angle
    }
}

impl CanvasTurtle {
    pub fn default_with_context(context: web_sys::CanvasRenderingContext2d) -> Self {
        CanvasTurtle {
            position: Default::default(),
            angle: 0.0,
            is_drawing: false,
            fill_color: None,
            context,
        }
    }

    pub fn pen_down(&mut self) {
        self.is_drawing = true;
        self.context.begin_path();
    }

    pub fn pen_up(&mut self) {
        self.context.close_path();
        if let Some(color) = &self.fill_color {
            self.context.set_fill_style(&JsValue::from_str(color));
            self.context.fill();
        }
        self.is_drawing = false;
    }

    pub fn line_color(&mut self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(color));
    }

    pub fn line_width(&mut self, width: f64) {
        self.context.set_line_width(width);
    }

    pub fn fill_color(&mut self, color: Option<String>) {
        self.fill_color = color.clone();
    }
}

pub struct GhostTurtle {
    position: Position,
    angle: f64,
}

impl Turtle for GhostTurtle {
    fn forward(&mut self, distance: f64) {
        let Position { x: old_x, y: old_y } = self.position;
        let new_x = old_x + distance * self.angle.cos();
        let new_y = old_y + distance * self.angle.sin();
        self.position = Position { x: new_x, y: new_y };
    }

    fn teleport(&mut self, position: Position, angle: f64) {
        self.position = position;
        self.angle = angle;
    }

    fn left(&mut self, degrees: f64) {
        self.angle -= degrees * std::f64::consts::PI / 180.0;
    }

    fn right(&mut self, degrees: f64) {
        self.angle += degrees * std::f64::consts::PI / 180.0;
    }

    fn position(&self) -> Position {
        self.position
    }

    fn angle(&self) -> f64 {
        self.angle
    }
}
