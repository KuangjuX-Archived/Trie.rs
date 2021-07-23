use spin::Mutex;
use alloc::vec::Vec;
use alloc::vec;

pub static mut TRIE: Mutex<Trie> = Mutex::new(Trie::init());

pub struct Trie {
    root: Node
}

#[derive(Clone)]
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

    pub fn push(&mut self, path: &'static str, handler: *mut u8) {
        let s: Vec<&'static str> = path.split('/').collect();
        let mut node = &mut self.root;
        let mut deep = 0;
        loop {
            if node.child.len() == 0 {
                panic!("Path exists");
            }
            match node.get(s[deep]) {
                Some(index) => {
                   node = &mut node.child[index];
                   deep += 1; 
                },

                None => {
                    node.push(&s[deep..], handler)
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
}