use std::collections::HashMap;
use std::rc::Rc;

mod key_type;

// Actions to be created will go here
struct Action;

//
struct ActionStorage {
    hm: HashMap<key_type::StringKey, Action>,
}

impl ActionStorage {
    fn init_as() -> ActionStorage {
        ActionStorage { hm: HashMap::new() }
    }

    fn key_maps(&self, key: key_type::StringKey, act: Action) {
        let mut k_as = ActionStorage::init_as();
        k_as.hm.insert(key, act);
    }
}
