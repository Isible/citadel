//! Very basic implementation of an environment to store variables, fucntions...
//! Under the hood this uses a hashmap to store the data.

use std::collections::HashMap;

use crate::{errors, obj::Object};

#[derive(Debug, Clone, Copy)]
pub enum EnvObjType<'o> {
    Variable {
        is_const: bool,
    },
    Function {
        ret_type: &'o str,
    },
    Label,
}

#[derive(Debug, Clone, Copy)]
pub struct EnvObj<'o> {
    pub _type: EnvObjType<'o>,
    pub val: &'o Object<'o>,
}

#[derive(Debug, Default)]
pub(crate) struct Environment<'e> {
    pub(crate) def: HashMap<&'e str, EnvObj<'e>>,
}

impl<'e> Environment<'e> {
    pub(crate) fn get(&self, key: &str) -> Result<EnvObj<'e>, errors::InvalidKeyError<&'e str>> {
        match self.def.get(key) {
            Some(val) => Ok(*val),
            None => todo!(),
        }
    }

    pub(crate) fn set(&mut self, key: &'e str, val: &'e EnvObj) {
        self.def.insert(key, *val);
    }
}
