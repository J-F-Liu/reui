use crate::properties::{Pos, Size};
use crate::{Element, ShapeType, Widget};

#[derive(Default)]
pub struct Pane {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub children: Vec<Box<dyn Widget>>,
}

impl Widget for Pane {
    fn render(&self) -> Element {
        let mut elem = Element::new()
            .set_property(ShapeType::Rectangle)
            .set_property(Pos::new(self.left, self.top))
            .set_property(Size::new(self.width, self.height));
        for child in self.children.iter() {
            elem.add_child(child.render());
        }
        elem
    }
}
