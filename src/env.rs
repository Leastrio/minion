use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Default, Clone, Debug)]
pub struct Environment {
  pub store: HashMap<String, Object>,
  pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Self {
    Environment {
      store: HashMap::new(),
      outer: Some(outer),
    }
  }

  pub fn insert(&mut self, key: String, val: Object) {
    self.store.insert(key, val);
  }

  pub fn get(&self, key: &String) -> Option<Object> {
    match self.store.get(key) {
      Some(obj) => return Some(obj.clone()),
      None => {
        let outer = self.outer.clone()?;
        let env = outer.borrow();
        (*env).get(key)
      }
    }
  }
}
