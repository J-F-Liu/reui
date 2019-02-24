#[derive(Clone, Debug)]
pub struct Text(pub String);

#[derive(Copy, Clone)]
pub struct Font<'a> {
    pub name: &'a str,
    pub size: f32,
}

impl<'a> Font<'a> {
    pub fn new(name: &'a str, size: f32) -> Self {
        Font { name, size }
    }
    pub fn default() -> Self {
        Font::new("default", 12.0)
    }
}
