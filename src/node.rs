pub trait NodeTrait: Sized {
    fn size(&self) -> usize;
    fn size_mut(&mut self) -> &mut usize;
    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn take_left(&mut self) -> Option<Box<Self>>;
    fn take_right(&mut self) -> Option<Box<Self>>;
    fn set_left(&mut self, node: Option<Box<Self>>);
    fn set_right(&mut self, node: Option<Box<Self>>);
    fn rotate_right(&mut self) {
        let mut left = match self.take_left() {
            Some(left) => left,
            None => return,
        };

        self.set_left(left.take_right());
        // TODO: update() 的な関数を呼ぶ
        *self.size_mut() = self.left().map_or(0, |node| node.size())
            + self.right().map_or(0, |node| node.size())
            + 1;

        std::mem::swap(self, &mut left);
        self.set_right(Some(left));
        *self.size_mut() =
            self.left().map_or(0, |n| n.size()) + self.right().map_or(0, |n| n.size()) + 1;
    }
    fn rotate_left(&mut self) {
        let mut right = match self.take_right() {
            Some(right) => right,
            None => return,
        };

        self.set_right(right.take_left());
        // TODO: update() 的な関数を呼ぶ
        *self.size_mut() =
            self.left().map_or(0, |n| n.size()) + self.right().map_or(0, |n| n.size()) + 1;

        std::mem::swap(self, &mut right);
        self.set_left(Some(right));
        *self.size_mut() =
            self.left().map_or(0, |n| n.size()) + self.right().map_or(0, |n| n.size()) + 1;
    }
}
