#![allow(unused)]

use std::collections::HashMap;

pub struct Document {
    elements: HashMap<usize, Element>
}

impl Document {
    pub fn new() -> Self {
        Self { elements: HashMap::new() }
    }

    pub fn get_element(&self, id: usize) -> Option<&Element> {
        self.elements.get(&id)
    }

    pub fn get_element_mut(&mut self, id: usize) -> Option<&mut Element> {
        self.elements.get_mut(&id)
    }

    pub fn create_element() -> usize {
        todo!()
    }

    pub fn append_element() -> usize {
        todo!()
    }

    pub fn remove_element() -> usize {
        todo!()
    }

    pub fn replace_element() -> usize {
        todo!()
    }
}

pub struct Element {
    pub id: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub attributes: Vec<Attribute>,
    pub text: String
}

impl Element {
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

pub struct Attribute {
    name: String,
    value: String
}