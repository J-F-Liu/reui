use std::{
    any::{Any, TypeId},
    clone::Clone,
    collections::HashMap,
};

#[derive(Default)]
pub struct Element {
    pub properties: HashMap<TypeId, Box<dyn Any>>,

    pub children: Vec<Element>,
}

/// Any type can be used as a property.
pub trait Property: Any {}

impl<T: Any> Property for T {}

impl Element {
    /// Creates a new element.
    pub fn new() -> Self {
        Default::default()
    }

    // Get property of element.
    pub fn property<P: Property>(&self) -> Option<&P> {
        self.properties.get(&TypeId::of::<P>()).map(|value| {
            value
                .downcast_ref()
                .expect("Property internal downcast error")
        })
    }

    // Get property of element.
    pub fn property_clone<P: Property + Clone>(&self) -> Option<P> {
        self.property::<P>().map(|value| value.clone())
    }

    // Set property of element.
    pub fn set_property<P: Property>(mut self, value: P) -> Self {
        self.properties.insert(TypeId::of::<P>(), Box::new(value));
        self
    }

    pub fn child(&self) -> Option<&Element> {
        if self.children.is_empty() {
            None
        } else {
            Some(&self.children[0])
        }
    }

    pub fn add_child(&mut self, child: Element) {
        self.children.push(child);
    }
}
