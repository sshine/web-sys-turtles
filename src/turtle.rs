use wasm_bindgen::JsValue;

pub struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
    is_drawing: bool,
    fill_color: Option<String>,
    context: web_sys::CanvasRenderingContext2d,
}

impl Turtle {
    pub fn default_with_context(context: web_sys::CanvasRenderingContext2d) -> Self {
        Turtle {
            x: Default::default(),
            y: Default::default(),
            angle: Default::default(),
            is_drawing: Default::default(),
            fill_color: Default::default(),
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

    pub fn teleport(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.context.move_to(x, y);
    }

    pub fn forward(&mut self, distance: f64) {
        let new_x = self.x + distance * self.angle.cos();
        let new_y = self.y + distance * self.angle.sin();
        self.context.line_to(new_x, new_y);
        if self.is_drawing {
            self.context.stroke();
        }
        self.x = new_x;
        self.y = new_y;
    }

    pub fn left(&mut self, degrees: f64) {
        self.angle -= degrees * std::f64::consts::PI / 180.0;
    }

    pub fn right(&mut self, degrees: f64) {
        self.angle += degrees * std::f64::consts::PI / 180.0;
    }
}
