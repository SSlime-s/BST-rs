pub trait BinarySearchTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn search(&self, key: &K) -> Option<&V>;
    fn search_mut(&mut self, key: &K) -> Option<&mut V>;
    fn min(&self) -> Option<(&K, &V)>;
    fn max(&self) -> Option<(&K, &V)>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    fn find_by_order(&self, order: usize) -> Option<(&K, &V)>;
    /**
     * key 未満である要素の個数を返す
     */
    fn order_of_key(&self, key: &K) -> usize;
}
