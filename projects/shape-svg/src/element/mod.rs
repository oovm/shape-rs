mod display;
use std::{
    collections::{btree_map, BTreeMap},
    slice,
};

/// The representation of a svg element tree.
#[derive(Debug, Clone, PartialEq)]
pub struct SVG {
    /// The element type of svg node
    element: &'static str,
    /// The attributes of svg node
    attributes: BTreeMap<&'static str, String>,
    /// The children of svg node
    children: Vec<SVG>,
}

impl SVG {
    /// Create a new svg element with given name and attributes.
    pub fn new(element: &'static str, attributes: Vec<(&'static str, String)>, children: Vec<SVG>) -> Self {
        Self { element, attributes: BTreeMap::from_iter(attributes.into_iter()), children }
    }
    /// Insert new attribute to current svg node.
    pub fn insert_attribute<S>(&mut self, keys: &'static str, value: S)
    where
        S: Into<String>,
    {
        self.attributes.insert(keys, value.into());
    }
    /// View all attributes of current svg node.
    pub fn attributes(&self) -> btree_map::Iter<'_, &'static str, String> {
        self.attributes.iter()
    }
    /// Get mutable reference of all attributes
    pub fn attributes_mut(&mut self) -> btree_map::IterMut<'_, &'static str, String> {
        self.attributes.iter_mut()
    }
    /// Insert new child to current svg node.
    pub fn insert_child(&mut self, child: SVG) {
        self.children.push(child);
    }
    /// View all children of current svg node.
    pub fn children(&self) -> slice::Iter<'_, SVG> {
        self.children.iter()
    }
    /// Get mutable reference of all children
    pub fn children_mut(&mut self) -> slice::IterMut<'_, SVG> {
        self.children.iter_mut()
    }
}
