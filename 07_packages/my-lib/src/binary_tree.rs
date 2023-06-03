pub mod bt {
    pub struct BinaryTreeNode<T: std::cmp::PartialOrd>  {
        // value of the Node
        value: T,
        // left child of the Node - the smaller child
        left_child: Option<Box<BinaryTreeNode<T>>>,
        // right child of the Node - the larger child
        right_child: Option<Box<BinaryTreeNode<T>>>
    }

    impl<T: std::cmp::PartialOrd> BinaryTreeNode<T> {
        pub fn new(i: T) -> BinaryTreeNode<T> {
            BinaryTreeNode {
                value: i,
                left_child: None,
                right_child: None
            }
        }

        pub fn insert(&mut self, i: T) -> bool {
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

    pub struct BinaryTree<T: std::cmp::PartialOrd> {
        // root node of the binary tree
        root: Option<Box<BinaryTreeNode<T>>>,
        node_count: usize,
    }

    impl<T: std::cmp::PartialOrd> BinaryTree<T> {
        pub fn new() -> BinaryTree<T> {
            BinaryTree { 
                root: None,
                node_count: 0 
            }
        }

        pub fn get_node_count(&mut self) -> usize {
            self.node_count
        }

        pub fn insert(&mut self, i: T) -> bool {
            let inserted: bool;
            match &mut self.root {
                Some(root_cont) => inserted = root_cont.as_mut().insert(i),
                None => {
                    let new_node: BinaryTreeNode<T> = BinaryTreeNode::new(i);
                    self.root = Some(Box::new(new_node));
                    inserted = true;
                },
            }
            if inserted {
                self.node_count = self.node_count + 1;
            }
            inserted
        }

        pub fn delete(&mut self, i: T) {
            // TODO
        }

        pub fn print(&mut self) {
            // TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn createBinaryTree() {
        let mut bt = BinaryTree::new();
        assert!(bt.root.is_none());
        assert_eq!(bt.node_count, 0);
        assert!(bt.insert(10));
        assert!(bt.root.is_some());
        assert_eq!(bt.node_count, 1);
        assert!(bt.insert(1));
        assert!(bt.root.is_some());
        assert_eq!(bt.node_count, 2);
        assert!(bt.insert(5));
        assert!(bt.root.is_some());
        assert_eq!(bt.node_count, 3);
        assert!(!bt.insert(5));
        assert!(bt.root.is_some());
        assert_eq!(bt.node_count, 3);
    }
}
