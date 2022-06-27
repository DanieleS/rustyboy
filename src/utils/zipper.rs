pub struct Zipper<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> Zipper<T> {
    pub fn new(items: Vec<T>) -> Option<Self> {
        if !items.is_empty() {
            Some(Zipper { items, index: 0 })
        } else {
            None
        }
    }

    pub fn new_with_index(items: Vec<T>, index: usize) -> Option<Self> {
        if index < items.len() {
            Some(Zipper { items, index })
        } else {
            None
        }
    }

    pub fn get(&self) -> &T {
        &self.items[self.index]
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.items[self.index]
    }
}
