use rand::random;

use crate::node::NodeTrait;

struct Node<K, V> {
    key: K,
    value: V,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    size: usize,
    priority: f64,
}
type NodePtrInner<K, V> = Option<Box<Node<K, V>>>;
struct NodePtr<K, V>(NodePtrInner<K, V>);

impl<K, V> NodeTrait for Node<K, V> {
    fn size(&self) -> usize {
        self.size
    }

    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }

    fn left(&self) -> Option<&Self> {
        self.left.0.as_deref()
    }

    fn right(&self) -> Option<&Self> {
        self.right.0.as_deref()
    }

    fn take_left(&mut self) -> Option<Box<Self>> {
        self.left.0.take()
    }

    fn take_right(&mut self) -> Option<Box<Self>> {
        self.right.0.take()
    }

    fn set_left(&mut self, node: Option<Box<Self>>) {
        self.left = NodePtr(node);
    }

    fn set_right(&mut self, node: Option<Box<Self>>) {
        self.right = NodePtr(node);
    }
}
impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: NodePtr(None),
            right: NodePtr(None),
            size: 1,
            priority: random::<f64>(),
        }
    }
}
impl<K: Ord, V> NodePtr<K, V> {
    fn new(key: K, value: V) -> Self {
        NodePtr(Some(Box::new(Node::new(key, value))))
    }
}

pub struct Treap<K: Ord, V> {
    root: NodePtr<K, V>,
}
impl<K: Ord, V> Treap<K, V> {
    pub fn new() -> Self {
        Treap { root: NodePtr(None) }
    }
}
impl<K: Ord, V> Default for Treap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
