#[deriving(Show)]
pub struct List<A> {
    priv head: Link<A>,
    priv length: uint
}

#[deriving(Show)]
struct Node<A> {
    next: Link<A>,
    value: A
}

type Link<A> = Option<~Node<A>>;

impl<A> List<A> {
    pub fn new() -> List<A> {
        List { head: None, length: 0 }
    }
}

impl<A> List<A> {
    fn push_node(&mut self, mut new_head: ~Node<A>) {
        use std::mem::swap;

        match self.head {
            None => {
                self.head = Some(new_head);
            },
            Some(ref mut head) => {
                swap(head, &mut new_head);
                head.next = Some(new_head);
            }
        };
        self.length += 1;
    }

    fn pop_node(&mut self) -> Option<~Node<A>> {
        self.head.take().map(|mut front_node| {
            self.head = front_node.next.take();
            self.length -= 1;
            front_node
        })
    }

    pub fn push(&mut self, val: A) {
        self.push_node(~Node { value: val, next: None });
    }

    pub fn pop(&mut self) -> Option<A> {
        self.pop_node().map(|n| n.value)
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iterator<T: Iterator<A>>(mut iterator: T) -> List<A> {
        let mut list: List<A> = List::new();
        for el in iterator {
            list.push(el);
        }
        list
    }
}

impl<A> Container for List<A> {
    fn len(&self) -> uint {
        self.length
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_pop_empty_list() {
        let mut list: List<()> = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push_pop_empty_list() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_list_len() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.len(), 1);
        list.pop();
        assert_eq!(list.len(), 0);
        list.pop();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_from_iterator() {
        let vec = [1, 2, 3, 4];

        let mut list: List<&int> = vec.iter().collect();
        assert_eq!(list.len(), 4);

        for i in vec.iter().rev() {
            assert_eq!(list.pop(), Some(i));
        }
    }
}
