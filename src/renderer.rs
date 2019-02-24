use crate::{Color, Rect};

#[derive(Clone, Debug)]
pub enum ShapeType {
    Line,
    Rectangle,
    Ellipse,
    Text,
    Image,
}

pub trait Renderer {
    fn fill(&mut self, background: Color);
    fn render_rectangle(
        &mut self,
        rect: Rect,
        border_width: u32,
        border_radius: u32,
        background: Color,
        border_color: Color,
    );
    fn render_text(
        &mut self,
        text: &str,
        rect: Rect,
        font_name: &str,
        font_size: f32,
        color: Color,
    );
    fn render_image(&mut self, image: &[Color], rect: Rect);
}
