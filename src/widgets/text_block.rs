use crate::properties::Text;
use crate::{Element, ShapeType, Widget};

#[derive(Default)]
pub struct TextBlock<'a> {
    pub text: &'a str,
}

impl<'a> Widget for TextBlock<'a> {
    fn render(&self) -> Element {
        Element::new()
            .set_property(ShapeType::Text)
            .set_property(Text(self.text.to_owned()))
    }
}
