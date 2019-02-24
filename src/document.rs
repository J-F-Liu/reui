#![allow(unused_imports)]

use crate::properties::{Background, Border, Font, Pos, Rect, Size, Text};
use crate::{Color, Element, Renderer, ShapeType, Widget};

pub struct Document {
    pub root: Element,
}

impl Document {
    pub fn render(&self, renderer: &mut Renderer) {
        fn render_rectangle(element: &Element, renderer: &mut Renderer) {
            let pos = element.property_clone::<Pos>().unwrap_or(Pos::origin());
            let size = element.property_clone::<Size>().unwrap_or(Size::zero());
            let border = element.property_clone::<Border>().unwrap_or(Border::none());
            let background = element
                .property_clone::<Background>()
                .unwrap_or(Background::transparent());
            renderer.render_rectangle(
                Rect::new(pos, size),
                border.radius,
                border.width,
                background.color,
                border.color,
            );
        }
        fn render_text(element: &Element, renderer: &mut Renderer) {
            if let Some(text) = element.property_clone::<Text>() {
                let pos = element.property_clone::<Pos>().unwrap_or(Pos::origin());
                let size = element.property_clone::<Size>().unwrap_or(Size::zero());
                let font = element.property_clone::<Font>().unwrap_or(Font::default());
                let color = element
                    .property_clone::<Color>()
                    .unwrap_or(crate::colors::transparent());
                renderer.render_text(&text.0, Rect::new(pos, size), font.name, font.size, color);
            }
        }
        fn render_element(element: &Element, renderer: &mut Renderer) {
            if let Some(shape_type) = element.property::<ShapeType>() {
                match shape_type {
                    ShapeType::Rectangle => render_rectangle(element, renderer),
                    ShapeType::Text => render_text(element, renderer),
                    _ => {}
                }
            }

            for child in element.children.iter() {
                render_element(child, renderer);
            }
        }

        render_element(&self.root, renderer);
    }
}
