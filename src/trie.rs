use spin::Mutex;
use alloc::vec::Vec;
use alloc::vec;

pub static mut TRIE: Mutex<Trie> = Mutex::new(Trie::init());

pub struct Trie {
    root: Node
}

#[derive(Debug, Clone)]
pub struct Node {
    name: &'static str,
    handler: *mut u8,
    child: Vec<Node>
}

impl Trie {
    const fn init() -> Self {
        Self {
            root: Node::new("/", 0 as *mut u8),
        }
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn call(&self, path: &'static str) -> Option<*mut u8> {
        let s: Vec<&'static str> = path.split('/').collect();
        let s = &s[1..];
        self.root.call(s)
    }

    pub fn push(&mut self, path: &'static str, handler: *mut u8) {
        let s: Vec<&'static str> = path.split('/').collect();
        let s = &s[1..];
        let mut node = &mut self.root;
        let mut deep = 0;
        while deep < s.len() {
            match node.get(s[deep]) {
                Some(index) => {
                    node = &mut node.child[index];
                    deep += 1; 
                },

                None => {
                    node.push(&s[deep..], handler);
                    break;
                }
            }
        }
    }
}

impl Node {
    const fn new(name: &'static str, handler: *mut u8) -> Self {
        Self {
            name,
            handler,
            child: vec![],
        }
    }

    fn get(&self, name: &str) -> Option<usize> {
        for index in 0..self.child.len() {
            if self.child[index].name == name {
                return Some(index)
            }
        }
        None
    }

    fn push(&mut self, path: &[&'static str], handler: *mut u8) {
        let mut deep = 0;
        self.child.push(Node{
            name: path[deep],
            handler: handler,
            child: vec![]
        });
        let mut node = &mut self.child[0];
        deep += 1;
        while deep < path.len() {
            node.child.push(Node{
                name: path[deep],
                handler,
                child: vec![]
            });
            node = &mut node.child[0];
            deep += 1;
        }
    }

    fn call(&self, path: &[&'static str]) -> Option<*mut u8> {
        let mut deep = 0;
        let mut node: &Node;
        match self.get(path[deep]) {
            Some(index) => {
                node = &self.child[index];
                deep += 1;
            },
            None => {
                return None
            }
        }
        loop {
            if deep == path.len() {
                return Some(node.handler)
            }

            match node.get(path[deep]) {
                Some(index) => {
                    node = &node.child[index];
                    deep += 1;
                },
                None => {
                    return None
                }
            }
        }
    }
}