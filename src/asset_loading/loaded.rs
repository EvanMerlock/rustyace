use std::rc::Rc;

pub enum Loaded<T: ?Sized> {
    Unloaded,
    Loaded(Rc<T>),
}