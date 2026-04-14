struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {

    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    pub fn push(&mut self, value: T) { // O(1)
        let old = self.head.take();
        let new_node = Node {
            value,
            next: old,
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> { // O(1)
        let old = self.head.take();
        match old {
            Some(old) => {
                let popped = old.value;
                self.head = old.next;
                self.size -= 1;
                Some(popped)
            },
            None => None,
        }
    }
}