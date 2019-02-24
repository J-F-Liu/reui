use crate::Element;

pub trait Widget {
    fn render(&self) -> Element;
}
