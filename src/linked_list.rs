struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn add_first(&mut self, value: T) {
        // O(1)
        let old = self.head.take();
        let new_node = Node { value, next: old };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn push(&mut self, value: T) {
        // O(n)
        let mut current = &mut self.head;
        loop {
            match current {
                Some(node) => current = &mut node.next,
                None => {
                    let new_node = Node { value, next: None };
                    *current = Some(Box::new(new_node));
                    self.size += 1;
                    return;
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // O(1)
        let old = self.head.take();
        match old {
            Some(old) => {
                let popped = old.value;
                self.head = old.next;
                self.size -= 1;
                Some(popped)
            }
            None => None,
        }
    }
}

pub struct IntoIter<T> {
    next_node: Link<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_node.take().map(|node| {
            let node = *node;
            self.next_node = node.next;
            node.value
        })
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            next_node: self.head,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(LinkedList::<()>::new().into_iter().count(), 0);
    }

    #[test]
    fn one() {
        let mut linked_list = LinkedList::new();
        linked_list.add_first(1);
        assert_eq!(linked_list.into_iter().count(), 1);
    }

    #[test]
    fn two() {
        let mut linked_list = LinkedList::new();
        linked_list.add_first(1);
        linked_list.add_first(2);
        assert_eq!(linked_list.into_iter().count(), 2);
    }
}
