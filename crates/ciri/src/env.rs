use std::collections::HashMap;

use crate::{errors, obj::Object};

#[derive(Debug, Clone)]
pub enum EnvObjType {
    Variable {
        is_const: bool,
        is_local: bool,
    },
    Function {
        is_local: bool,
        ret_type: String,
    },
    Label,
}

#[derive(Debug, Clone)]
pub struct EnvObj {
    pub _type: EnvObjType,
    pub val: Object,
}

#[derive(Debug)]
pub(crate) struct Environment {
    pub(crate) def: HashMap<String, EnvObj>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            def: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, key: String) -> Result<EnvObj, errors::InvalidKeyError<String>> {
        match self.def.get(&key) {
            Some(val) => Ok(val.clone()),
            None => todo!(),
        }
    }

    pub(crate) fn set(&mut self, key: String, val: EnvObj) {
        self.def.insert(key, val);
    }
}
