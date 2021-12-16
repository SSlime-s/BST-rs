pub trait BinarySearchTree<T> {
    fn insert(&mut self, key: T);
    fn remove(&mut self, key: T);
    fn search(&self, key: T) -> bool;
    fn min(&self) -> Option<&T>;
    fn max(&self) -> Option<&T>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    fn find_by_order(&self, order: usize) -> Option<&T>;
    fn order_of_key(&self, key: T) -> Option<usize>;
}
