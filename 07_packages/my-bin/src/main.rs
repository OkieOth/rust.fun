use my_lib::binary_tree::bt::BinaryTree;
use my_lib::dummy;

fn main() {
    let mut bt = BinaryTree::new();
    dummy::say_hello();
    bt.insert(40);
    bt.insert(20);
    bt.insert(50);
    println!("Binary tree has {} elements", bt.get_node_count());
    bt.insert(70);
    println!("Binary tree has {} elements", bt.get_node_count());
}
