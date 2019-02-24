#![allow(unused_imports)]

use reui::properties::{Background, Border, Pos, Size};
use reui::widgets::{Pane, TextBlock};
use reui::{Color, Rect};
use reui::{Document, Element, Renderer, ShapeType, Widget};

#[derive(Default)]
pub struct CommandRenderer {
    pub commands: String,
}

impl CommandRenderer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Renderer for CommandRenderer {
    fn fill(&mut self, background: Color) {
        self.commands.push_str(&format!("fill: {:?}\n", background));
    }
    fn render_rectangle(
        &mut self,
        rect: Rect,
        border_width: u32,
        border_radius: u32,
        _background: Color,
        _border_color: Color,
    ) {
        self.commands.push_str(&format!(
            "rectangle: {:?} border: {:?} {:?}\n",
            rect, border_width, border_radius
        ));
    }
    fn render_text(
        &mut self,
        text: &str,
        rect: Rect,
        font_name: &str,
        font_size: f32,
        _color: Color,
    ) {
        self.commands.push_str(&format!(
            "text: {:?} {:?} font: {:?}, {:?}\n",
            text, rect, font_name, font_size
        ));
    }
    fn render_image(&mut self, image: &[Color], rect: Rect) {
        self.commands
            .push_str(&format!("image: {:?}, {:?}\n", image.len(), rect));
    }
}

#[test]
fn test_render() {
    let elem = Pane {
        top: 0.0,
        left: 0.0,
        width: 300.0,
        height: 200.0,
        children: vec![
            Box::new(Pane {
                top: 10.0,
                left: 10.0,
                width: 10.0,
                height: 20.0,
                children: vec![],
            }),
            Box::new(Pane {
                top: 10.0,
                left: 50.0,
                width: 30.0,
                height: 40.0,
                children: vec![Box::new(TextBlock { text: "TextBlock" })],
            }),
        ],
    }
    .render();

    let document = Document { root: elem };

    let mut renderer = CommandRenderer::new();
    document.render(&mut renderer);
    print!("{}", renderer.commands);
}
