#[cfg(test)]
mod tests {
    use layer::*;

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
