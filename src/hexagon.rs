use crate::turtle::{CanvasTurtle, Turtle as _};
use crate::Position;

#[derive(Debug, Copy, Clone)]
pub struct Hexagon {
    pub position: Position,
    pub angle: f64,
    pub side: f64,
}

impl Hexagon {
    pub fn draw(&self, turtle: &mut CanvasTurtle) {
        turtle.pen_down();
        turtle.forward(0.0);
        for _ in 0..6 {
            turtle.forward(self.side);
            turtle.right(60.0);
        }
        turtle.pen_up();
    }

    pub fn draw_children(&self, turtle: &mut CanvasTurtle) {
        let child_side = self.side / 3.0;
        for _ in 0..6 {
            turtle.forward(child_side);
            let hex_child = Hexagon {
                position: turtle.position(),
                angle: turtle.angle(),
                side: child_side,
            };
            hex_child.draw(turtle);
            turtle.forward(2.0 * child_side);
            turtle.right(60.0);
        }
    }

    pub fn children(&self, turtle: &CanvasTurtle) -> Vec<Hexagon> {
        let mut children = Vec::with_capacity(6);
        let mut ghost = turtle.ghost();
        let child_side = self.side / 3.0;
        for _ in 0..6 {
            ghost.forward(child_side);
            children.push(Hexagon {
                position: ghost.position(),
                angle: ghost.angle(),
                side: child_side,
            });
            ghost.forward(2.0 * child_side);
            ghost.right(60.0);
        }
        children
    }
}
