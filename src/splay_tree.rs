use crate::tree_trait::BinarySearchTree;

struct Node<T> {
    key: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    size: usize,
}
impl<T> Node<T> {
    fn new(key: T) -> Self {
        Node {
            key,
            left: None,
            right: None,
            size: 1,
        }
    }
}

struct SplayTree<T>
where
    T: Ord,
{
    root: Option<Box<Node<T>>>,
}
impl<T> SplayTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        SplayTree {
            root: None,
        }
    }
}

impl<T> BinarySearchTree<T> for SplayTree<T>
where
    T: Ord,
{
    fn insert(&mut self, key: T) {
        todo!()
    }

    fn remove(&mut self, key: T) {
        todo!()
    }

    fn search(&self, key: T) -> bool {
        todo!()
    }

    fn min(&self) -> Option<&T> {
        todo!()
    }

    fn max(&self) -> Option<&T> {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn find_by_order(&self, order: usize) -> Option<&T> {
        todo!()
    }

    fn order_of_key(&self, key: T) -> Option<usize> {
        todo!()
    }
}
