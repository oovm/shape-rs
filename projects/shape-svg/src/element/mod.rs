mod display;
use std::collections::BTreeMap;

pub struct SVG {
    element: &'static str,
    attributes: BTreeMap<&'static str, String>,
    children: Vec<SVG>,
}

impl SVG {
    pub fn new(element: &'static str, attributes: Vec<(&'static str, String)>, children: Vec<SVG>) -> Self {
        Self { element, attributes: BTreeMap::from_iter(attributes.into_iter()), children }
    }
    pub fn insert_attribute<S>(&mut self, keys: &'static str, value: S)
    where
        S: Into<String>,
    {
        self.attributes.insert(keys, value.into());
    }
    pub fn insert_child(&mut self, child: SVG) {
        self.children.push(child);
    }
}
