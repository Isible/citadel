use std::collections::HashMap;

use crate::{errors, obj::Object};

pub(crate) struct Environment {
    pub(crate) def: HashMap<String, Object>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            def: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, key: String) -> Result<&Object, errors::InvalidKeyError<String>> {
        match self.def.get(&key) {
            Some(val) => Ok(val),
            None => todo!(),
        }
    }

    pub(crate) fn set(&mut self, key: String, val: Object) {
        self.def.insert(key, val);
    }
}
