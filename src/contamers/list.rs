#[deriving(Show, Eq)]
pub enum List<A> {
    Cons(A, ~List<A>),
    Nil
}

impl<A> List<A> {
    pub fn new() -> List<A> { Nil }
}

impl<A> List<A> {
    pub fn cons(x: A, xs: List<A>) -> List<A> {
        Cons(x, ~xs)
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iterator<T: Iterator<A>>(mut iterator: T) -> List<A> {
        let mut list: List<A> = List::new();
        for element in iterator {
            list = List::cons(element, list);
        }
        list
    }
}

impl<A> Container for List<A> {
    fn len(&self) -> uint {
        match *self {
            Nil => { 0 },
            Cons(_, ref xs) => { 1 + xs.len() }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{List, Cons, Nil};

    #[test]
    fn test_cons_empty_list() {
        let x = Nil;
        assert_eq!(List::cons(1, x), Cons(1, ~Nil));
    }

    #[test]
    fn test_from_iterator() {
        let a = ~[1, 2, 3];
        let b: List<int> = a.iter().map(|&x| x).collect();
        assert_eq!(b, Cons(3, ~Cons(2, ~Cons(1, ~Nil))));
    }
}
