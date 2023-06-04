
struct BinaryTreeNode<T: std::cmp::PartialOrd>  {
    // value of the Node
    value: T,
    // left child of the Node - the smaller child
    left_child: Option<Box<BinaryTreeNode<T>>>,
    // right child of the Node - the larger child
    right_child: Option<Box<BinaryTreeNode<T>>>
}

impl<T: std::cmp::PartialOrd> BinaryTreeNode<T> {
    fn new(i: T) -> BinaryTreeNode<T> {
        BinaryTreeNode {
            value: i,
            left_child: None,
            right_child: None
        }
    }

    fn insert(&mut self, i: T) -> bool {
        if i < self.value {
            return insert_child_or_travers(i, &mut self.left_child);
        } else if i > self.value {
            return insert_child_or_travers(i, &mut self.right_child);
        } else {
            return false;
        }
    }
}


fn insert_child_or_travers<T: std::cmp::PartialOrd> (i: T, child: &mut Option<Box<BinaryTreeNode<T>>>) -> bool {
    match child {
        Some(c) => return c.as_mut().insert(i),
        None => {
            let new_node: BinaryTreeNode<T> = BinaryTreeNode::new(i);
            *child = Some(Box::new(new_node));
            return true;
        }
    }
}


struct BinaryTree<T: std::cmp::PartialOrd> {
    // root node of the binary tree
    root: Option<Box<BinaryTreeNode<T>>>,
    nodeCount: usize,
}

impl<T: std::cmp::PartialOrd> BinaryTree<T> {
    fn new() -> BinaryTree<T> {
        BinaryTree { 
            root: None,
            nodeCount: 0 
        }
    }

    fn insert(&mut self, i: T) -> bool {
        let inserted: bool;
        match &mut self.root {
            Some(rootCont) => inserted = rootCont.as_mut().insert(i),
            None => {
                let newNode: BinaryTreeNode<T> = BinaryTreeNode::new(i);
                self.root = Some(Box::new(newNode));
                inserted = true;
            },
        }
        if inserted {
            self.nodeCount = self.nodeCount + 1;
        }
        inserted
    }

    fn delete(&mut self, i: T) {
        // TODO
    }

    fn print(&mut self) {
        // TODO
    }
}

struct SplitReturn<'a> {
    key: &'a str,
    left: Option<&'a str>,
    right: Option<&'a str>
}

enum SplitState {
    SearchForKeyEnd,
    SearchForChildSplit,
    SearchForKeyChildEnd
}

fn split_binary_tree_string<'a>(s: &'a str) -> SplitReturn {
    // got something like that ... 3(2(1,2),5(4,))
    
    let mut state= SplitState::SearchForKeyEnd;
    let mut ret = SplitReturn {
        key: "",
        left: None,
        right: None,
    };

    let mut open_bracket: i32 = -1;
    let mut child_split: i32 = -1;
    let mut bracket_count: usize = 0;

    for (i, c) in s.char_indices() {
        match state {
            SplitState::SearchForKeyEnd => {
                match c {
                    '(' => {
                        // found end of key
                        ret.key = &s[0 .. i];
                        open_bracket = i as i32;
                        bracket_count = 1;
                        state = SplitState::SearchForChildSplit;
                    },
                    ',' => {
                        // no key available ... e. g. `,4`
                    },
                    _ => {
                        // look at next char
                    }
                }
            },
            SplitState::SearchForChildSplit => {
                match c {
                    '(' => {
                        // found end of key
                        bracket_count += 1;
                    },
                    ')' => {
                        // found end of key
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            // TODO ... wrong format???
                        }
                    },
                    ',' => {
                        if bracket_count == 1 {
                            child_split = i as i32;
                            let start_slice: usize = (open_bracket as usize) + 1;
                            ret.left = Some(&s[start_slice .. i]);
                            state = SplitState::SearchForKeyChildEnd;
                        }
                        // no key available ... e. g. `,4`
                    },
                    _ => {
                        // look at next char
                    }
                };
            },
            SplitState::SearchForKeyChildEnd => {
                match c {
                    '(' => {
                        // found end of key
                        bracket_count += 1;
                    },
                    ')' => {
                        // found end of key
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            let start_slice: usize = (child_split as usize) + 1;
                            ret.right = Some(&s[start_slice .. i]);
                        }
                    },
                    _ => {
                        // look at next char
                    }
                }
            },
        }
        println!("  c={}", c)
    }
    SplitReturn { key: s, left: Some(s), right: None }
}

#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn create_binary_tree() {
        let mut bt = BinaryTree::new();
        assert!(bt.root.is_none());
        assert_eq!(bt.nodeCount, 0);
        assert!(bt.insert(10));
        assert!(bt.root.is_some());
        assert_eq!(bt.nodeCount, 1);
        assert!(bt.insert(1));
        assert!(bt.root.is_some());
        assert_eq!(bt.nodeCount, 2);
        assert!(bt.insert(5));
        assert!(bt.root.is_some());
        assert_eq!(bt.nodeCount, 3);
        assert!(!bt.insert(5));
        assert!(bt.root.is_some());
        assert_eq!(bt.nodeCount, 3);
    }

    fn test_split_binary_tree_string() {

    }

}

fn main() {
    println!("Hello, world!");
}
