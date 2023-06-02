
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

#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn createBinaryTree() {
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
}

fn main() {
    println!("Hello, world!");
}
