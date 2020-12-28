#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(future_incompatible)]

use std::collections::HashMap;
use std::rc::Rc;

mod loaded;
mod namespace;

pub struct AssetStore<T: ?Sized> {
    inner: HashMap<namespace::AssetReference, Rc<T>>,
}

impl<T: ?Sized + Loadable> AssetStore<T> {
    pub fn new() -> AssetStore<T> {
        AssetStore {
            inner: HashMap::new(),
        }
    }
}