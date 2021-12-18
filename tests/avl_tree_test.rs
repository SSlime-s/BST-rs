extern crate binary_search_tree;
use binary_search_tree::{avl_tree::AVLTree, tree_trait::BinarySearchTree};

#[test]
fn insert_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );
}

#[test]
fn insert_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );
}
