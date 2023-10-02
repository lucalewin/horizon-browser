#![allow(unused)]

use std::collections::HashMap;

pub struct Arena<T> {
    nodes: Vec<Node<T>>
}

pub struct HashArena<T> {
    nodes: HashMap<usize, Node<T>>
}

pub struct Node<T> {
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    data: T
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add(&mut self, data: T) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { id, parent: None, children: Vec::new(), data });
        id
    }

    pub fn get(&self, id: usize) -> Option<&Node<T>> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }
}

impl<T> HashArena<T> {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }
}

impl<T> Node<T> {
    pub fn parent<'a>(&self, arena: &'a Arena<T>) -> Option<&'a Self> {
        arena.get(self.parent?)
    }

    pub fn children<'a>(&'a self, arena: &'a Arena<T>) -> Vec<&'a Node<T>> {
        let mut children = Vec::new();
        for child_id in &self.children {
            if let Some(child) = arena.get(*child_id) {
                children.push(child)
            }
        }
        children
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}