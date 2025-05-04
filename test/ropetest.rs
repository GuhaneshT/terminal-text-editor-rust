#[derive(Clone)]
enum RopeNode {
    Leaf(String),
    Internal {
        left: Rc<RopeNode>,
        right: Rc<RopeNode>,
        weight: usize, // Length of left subtree
    },
}

use std ::rc::Rc;
#[derive(Clone)]
struct Rope {
    root: Rc<RopeNode>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            root: Rc::new(RopeNode::Leaf(String::new())),
        }
    }

    fn from_string(s: &str) -> Self {
        Rope {
            root: Rc::new(RopeNode::Leaf(s.to_string())),
        }
    }

    fn len(&self) -> usize {
        self.total_len(&self.root)
    }
    
    fn total_len(&self, node: &Rc<RopeNode>) -> usize {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Internal { left, right, .. } => {
                self.total_len(left) + self.total_len(right)
            }
        }
    }
    

    fn weight(&self, node: &Rc<RopeNode>) -> usize {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.len(),
            RopeNode::Internal { weight, .. } => *weight,
        }
    }

    fn concat(left: Rope, right: Rope) -> Rope {
        let weight = left.len();
        Rope {
            root: Rc::new(RopeNode::Internal {
                left: left.root,
                right: right.root,
                weight,
            }),
        }
    }

    fn split(&self, index: usize) -> (Rope, Rope) {
        let index = index.min(self.len());
        let (left, right) = self.split_node(&self.root, index);
        (Rope { root: left }, Rope { root: right })
    }

    fn split_node(&self, node: &Rc<RopeNode>, index: usize) -> (Rc<RopeNode>, Rc<RopeNode>) {
        match node.as_ref() {
            RopeNode::Leaf(s) => {
                let index = index.min(s.len());
                let (left, right) = s.split_at(index);
                (
                    Rc::new(RopeNode::Leaf(left.to_string())),
                    Rc::new(RopeNode::Leaf(right.to_string())),
                )
            }
            RopeNode::Internal { left, right, weight } => {
                if index <= *weight {
                    let (ll, lr) = self.split_node(left, index);
                    (
                        ll,
                        Rc::new(RopeNode::Internal {
                            left: lr.clone(),
                            right: right.clone(),
                            weight: self.total_len(&lr),
                        }),
                    )
                } else {
                    let (rl, rr) = self.split_node(right, index - weight);
                    (
                        Rc::new(RopeNode::Internal {
                            left: left.clone(),
                            right: rl.clone(),
                            weight: self.total_len(&left),
                        }),
                        rr,
                    )
                }
            }
        }
    }
    

    fn insert(&self, index: usize, text: &str) -> Rope {
        let (left, right) = self.split(index);
        let middle = Rope::from_string(text);
        Rope::concat(Rope::concat(left, middle), right)
    }

    fn delete(&self, start: usize, len: usize) -> Rope {
        let (left, rest) = self.split(start);
        let rest_len = rest.len();
        let len = len.min(rest_len);
        let (_, right) = rest.split(len);
        Rope::concat(left, right)
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        self.collect(&self.root, &mut result);
        result
    }

    fn collect(&self, node: &Rc<RopeNode>, result: &mut String) {
        match node.as_ref() {
            RopeNode::Leaf(s) => result.push_str(s),
            RopeNode::Internal { left, right, .. } => {
                self.collect(left, result);
                self.collect(right, result);
            }
        }
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.get_char(&self.root, index)
    }

    fn get_char(&self, node: &Rc<RopeNode>, index: usize) -> Option<char> {
        match node.as_ref() {
            RopeNode::Leaf(s) => s.chars().nth(index),
            RopeNode::Internal { left, right, weight } => {
                if index < *weight {
                    self.get_char(left, index)
                } else {
                    self.get_char(right, index - weight)
                }
            }
        }
    }
}

fn main() {
    // Test 1: Creating a new empty Rope
    let r1 = Rope::new();
    assert_eq!(r1.len(), 0);
    assert_eq!(r1.to_string(), "");

    // Test 2: Creating a Rope from a string
    let r2 = Rope::from_string("hello");
    assert_eq!(r2.len(), 5);
    assert_eq!(r2.to_string(), "hello");

    // Test 3: Concatenating two ropes
    let r3 = Rope::from_string("world");
    let r4 = Rope::concat(r2.clone(), r3.clone());
    assert_eq!(r4.to_string(), "helloworld");
    assert_eq!(r4.len(), 10);

    // Test 4: Splitting a rope
    let (left, right) = r4.split(5);
    assert_eq!(left.to_string(), "hello");
    assert_eq!(right.to_string(), "world");

    // Test 5: Inserting into a rope
    let r5 = r4.insert(5, ", ");
    assert_eq!(r5.to_string(), "hello, world");

    // Test 6: Deleting from a rope
    let r6 = r5.delete(5, 2); // Remove ", "
    assert_eq!(r6.to_string(), "helloworld");

    // Test 7: char_at
    assert_eq!(r6.char_at(0), Some('h'));
    assert_eq!(r6.char_at(4), Some('o'));
    assert_eq!(r6.char_at(5), Some('w'));
    assert_eq!(r6.char_at(10), None); // Out of bounds

    // Test 8: Inserting at beginning and end
    let r7 = r6.insert(0, "Say ");
    let r8 = r7.insert(r7.len(), "!");
    assert_eq!(r8.to_string(), "Say helloworld!");

    // Test 9: Deleting entire rope
    let r9 = r8.delete(0, r8.len());
    assert_eq!(r9.to_string(), "");
    assert_eq!(r9.len(), 0);

    println!("All tests passed!");
}
