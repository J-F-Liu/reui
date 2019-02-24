mod document;
mod element;
mod renderer;
mod widget;

pub mod colors;
pub mod properties;
pub mod widgets;

pub use document::*;
pub use element::*;
pub use renderer::*;
pub use widget::*;

pub use properties::Rect;
pub type Color = rgb::RGBA8;
