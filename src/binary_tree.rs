#![allow(dead_code)]

struct Node {
    key: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
    size: usize,
}

impl BinaryTree {
    pub fn new() -> Self { Self { root: None, size: 0 }}

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn insert(&mut self, key: usize) { // O(log2(n))
        let mut current = &mut self.root;
        loop {
            match current {
                Some(node) => {
                    if key == node.key { return; } // No duplicates
                    if key < node.key {
                        current = &mut node.left;
                    } else {
                        current = &mut node.right;
                    }
                } ,
                None => {
                    let new_node = Node {
                        key,
                        left: None,
                        right: None,
                    };
                    *current = Some(Box::new(new_node));
                    self.size += 1;
                    return;
                },
            }
        }
    }
}