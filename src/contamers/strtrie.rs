extern crate collections;

use collections::hashmap::HashMap;

#[deriving(Show)]
enum Value {
    Terminal(char), // node that marks the end of a word
    Internal(char), // node that is not the end of a word
    Root
}

#[deriving(Show)]
struct Node {
    value: Value,
    children: HashMap<char, ~Node>
}

#[deriving(Show)]
pub struct StrTrie {
    priv root: Node
}

impl Node {
    fn new_root() -> Node {
        Node { value: Root, children: HashMap::new() }
    }
    fn new_terminal(c: char) -> Node {
        Node { value: Terminal(c), children: HashMap::new() }
    }
    fn new_internal(c: char) -> Node {
        Node { value: Internal(c), children: HashMap::new() }
    }

    fn is_terminal(&self) -> bool {
        match self.value {
            Terminal(_) => true,
            _ => false
        }
    }

    fn insert(&mut self, val: &str) {
        match val.chars().next() {
            None => return,
            Some(c) => {
                let children = &mut self.children;
                if val.len() == 1 {
                    children.insert_or_update_with(c, ~Node::new_terminal(c), |&k, node| {
                        node.value = Terminal(k);
                    });
                } else {
                    children.find_or_insert_with(c, |&k| {
                        ~Node::new_internal(k)
                    }).insert(val.slice_from(1));
                }
            }
        }
    }

    fn contains(&self, val: &str) -> bool {
        match val.chars().next() {
            None => false,
            Some(c) => {
                match self.children.find(&c) {
                    Some(ref node) => {
                        if val.len() == 1 {
                            node.is_terminal()
                        } else {
                            node.contains(val.slice_from(1))
                        }
                    },
                    _ => false
                }
            }
        }
    }
}

impl StrTrie {
    pub fn new() -> StrTrie {
        StrTrie { root: Node::new_root() }
    }

    pub fn insert(&mut self, val: &str) {
        self.root.insert(val);
    }

    pub fn contains(&mut self, val: &str) -> bool {
        self.root.contains(val)
    }
}

#[cfg(test)]
mod test {
    use super::StrTrie;

    #[test]
    fn test_insert_contains() {
        let mut trie = StrTrie::new();
        trie.insert("a");
        assert!(trie.contains("a"));
    }

    #[test]
    fn test_does_not_contain_prefix() {
        let mut trie = StrTrie::new();
        trie.insert("boot");
        assert!(!trie.contains("boo"));
        assert!(trie.contains("boot"));
    }

    #[test]
    fn test_contains_prefix_inserted_later() {
        let mut trie = StrTrie::new();
        trie.insert("boot");
        trie.insert("boo");
        assert!(trie.contains("boo"));
        assert!(trie.contains("boot"));
    }
}
