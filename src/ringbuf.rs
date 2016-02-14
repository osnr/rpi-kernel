pub const SIZE: usize = 128;

// TODO: Make a real initializer.
pub struct Buf<T> {
    pub elems: [Option<T>; SIZE],

    pub head: usize,
    pub tail: usize,
}

impl<T: Copy> Buf<T> {
    pub fn empty(&self) -> bool {
        return self.tail == self.head;
    }

    #[allow(dead_code)]
    pub fn full(&self) -> bool {
        let next_tail = (self.tail + 1) % SIZE;
        return next_tail == self.head;
    }

    pub fn pop(&mut self) -> T {
        let elem = self.elems[self.head].unwrap();
        self.elems[self.head] = None;

        self.head = (self.head + 1) % SIZE;

        return elem;
    }

    pub fn push(&mut self, elem: T) {
        let next_tail = (self.tail + 1) % SIZE;
        if next_tail == self.head {
            panic!();
        }

        self.elems[self.head] = Some(elem);
        self.tail = next_tail;
    }
}
