use std::fmt::Display;

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: Display> List<T> {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn len(&self) -> usize {
        match *self {
            List::Nil => 0,
            List::Cons(_, ref node) => 1 + node.len(),
        }
    }

    pub fn display(&self) {
        match *self {
            List::Nil => {}
            List::Cons(ref x, ref node) => {
                print!("{} ", x);
                node.display();
            }
        }
    }
}
