use crate::traits::{self, RefC, RefCopy};
use std::{collections::HashMap, ops, ptr};

pub struct Map {
    map: *mut HashMap<String, usize>,
    store: *mut crate::List,
    refc: i64,
}

impl Map {
    /// Create a new shsc::Map struct.
    /// ### Example
    /// ```
    /// let m = shsc::Map::new();
    /// ```

    pub fn new() -> Self {
        let map = Map {
            map: Box::into_raw(Box::new(HashMap::new())),
            store: Box::into_raw(Box::new(crate::List::new())),
            refc: 1,
        };
        unsafe {
            ptr::write(map.map, HashMap::new());
            ptr::write(map.store, crate::List::new());
        }
        map
    }

    /// Create a new shsc::Map struct from a native HashMap type.
    /// ### Arguments
    /// * `hashmap` - A native HashMap type
    /// ### Returns
    /// A new Map struct
    /// ### Example
    /// ```
    /// let mut hashmap = std::collections::HashMap::new();
    /// hashmap.insert("key".to_owned(), shsc::todata!(10));
    /// let m = shsc::Map::from(hashmap);
    /// ```

    pub fn from(hashmap: HashMap<String, crate::Data>) -> Self {
        let newmap = Map::new();
        unsafe {
            for (key, value) in hashmap {
                let index = newmap.store.as_ref().unwrap().len();
                newmap.map.as_mut().unwrap().insert(key, index);
                newmap.store.as_mut().unwrap().append(value);
            }
        }
        newmap
    }

    /// Insert a key-value pair into the shsc::Map struct.
    /// ### Arguments
    /// * `key` - A native str type
    /// * `value` - A shsc::Data type
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// ```

    pub fn insert(&mut self, key: &str, value: crate::Data) {
        unsafe {
            if self.map.as_ref().unwrap().contains_key(key) {
                let index: usize = self.map.as_ref().unwrap()[key];
                self.store.as_mut().unwrap()[index] = value;
            } else {
                let index = self.store.as_ref().unwrap().len();
                self.map.as_mut().unwrap().insert(key.to_owned(), index);
                self.store.as_mut().unwrap().append(value);
            }
        }
    }

    /// Get a reference to the shsc::Data type associated with a key.
    /// ### Arguments
    /// * `key` - A native str type
    /// ### Returns
    /// A reference to the shsc::Data type associated with the key
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// let value = m.get("key").unwrap();
    /// ```

    pub fn get(&self, key: &str) -> Option<&crate::Data> {
        unsafe {
            let store = self
                .store
                .as_ref()
                .expect("shsc::Map::get: undefined data store");
            match self.map.as_ref().unwrap().get(key) {
                Some(&index) => store.get(index),
                None => None,
            }
        }
    }

    /// Get a mutable reference to the shsc::Data type associated with a key.
    /// ### Arguments
    /// * `key` - A native str type
    /// ### Returns
    /// A mutable reference to the shsc::Data type associated with the key
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// let value = m.get_mut("key").unwrap();
    /// *value = shsc::todata!(20);
    /// ```

    pub fn get_mut(&mut self, key: &str) -> Option<&mut crate::Data> {
        unsafe {
            let store = self
                .store
                .as_mut()
                .expect("shsc::Map::get_mut: undefined data store");
            match self.map.as_ref().unwrap().get(key) {
                Some(&index) => store.get_mut(index),
                None => None,
            }
        }
    }

    /// Remove a key-value pair from the shsc::Map struct.
    /// ### Arguments
    /// * `key` - A native str type
    /// ### Returns
    /// The shsc::Data type associated with the key
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// let value = m.remove("key").unwrap();
    /// ```

    pub fn remove(&mut self, key: &str) -> Option<crate::Data> {
        unsafe {
            match self.map.as_mut().unwrap().remove(key) {
                Some(index) => {
                    let tmp = self.store.as_mut().unwrap()[index].refcopy();
                    self.store.as_mut().unwrap()[index] = crate::Data::NULL;
                    Some(tmp)
                }
                None => None,
            }
        }
    }
}

impl ops::Index<&str> for Map {
    type Output = crate::Data;

    /// Get a reference to the shsc::Data type associated with a key.
    /// ### Arguments
    /// * `key` - A native str type
    /// ### Returns
    /// A reference to the shsc::Data type associated with the key
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// let value = &m["key"];
    /// ```

    fn index(&self, key: &str) -> &crate::Data {
        self.get(key)
            .expect(&format!("shsc::Map::index: invalid key {}", key))
    }
}

impl ops::IndexMut<&str> for Map {
    /// Get a mutable reference to the shsc::Data type associated with a key.
    /// ### Arguments
    /// * `key` - A native str type
    /// ### Returns
    /// A mutable reference to the shsc::Data type associated with the key
    /// ### Example
    /// ```
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// m["key"] = shsc::todata!(20);
    /// ```

    fn index_mut(&mut self, key: &str) -> &mut crate::Data {
        self.get_mut(key)
            .expect(&format!("shsc::Map::index_mut: invalid key {}", key))
    }
}

impl traits::ToStr for Map {
    /// Get a string representation of the shsc::Map struct.
    /// ### Returns
    /// A string representation of the shsc::Map struct
    /// ### Example
    /// ```
    /// use shsc::traits::ToStr;
    /// let mut m = shsc::Map::new();
    /// m.insert("key", shsc::todata!(10));
    /// m.insert("key2", shsc::todata!(20));
    /// m.insert("key3", shsc::todata!(30));
    /// m.insert("key4", shsc::todata!(40));
    /// let s = m.tostr();
    /// println!("{}", s);
    /// ```

    fn tostr(&self) -> String {
        let map = unsafe {
            self.map
                .as_ref()
                .expect("shsc::Map::tostr: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_ref()
                .expect("shsc::Map::tostr: undefined data store")
        };
        let mut result = String::from("{");
        for (i, (key, &index)) in map.iter().enumerate() {
            let value = store
                .get(index)
                .expect(&format!("shsc::Map::tostr: invalid index {}", index));
            if value.is_null() {
                continue;
            }
            result.push_str(&format!("{}: {}", key, value.tostr()));
            if i < store.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push_str("}");
        result
    }
}

impl traits::RefCopy for Map {
    /// Implement the RefCopy trait for the Map struct.
    /// This allows us to create a new Map struct from an existing Map struct.
    /// ### Returns
    /// A reference counted copy of the Map struct
    /// ### Example
    /// ```
    /// use shsc::traits::RefCopy;
    /// let mut m = shsc::Map::new();
    /// let m2 = m.refcopy();
    /// ```

    fn refcopy(&mut self) -> Self {
        self.incrc();
        Map {
            map: self.map,
            store: self.store,
            refc: self.refc,
        }
    }

    /// Implement the RefCopy trait for the Map struct.
    /// This allows us to deallocate the Map struct when the reference count reaches zero.
    /// Map will be deallocated if the reference count reaches zero.
    /// ### Example
    /// ```
    /// use shsc::traits::RefCopy;
    /// let mut m = shsc::Map::new();
    /// {
    ///     let m2 = m.refcopy();
    /// }
    /// m.refdrop();
    /// ```

    fn refdrop(self) {}
}

impl Clone for Map {
    /// Implement the Clone trait for the Map struct.
    /// This allows us to create a deep copy of the Map struct.
    /// Results in a new Map struct with a reference count of 1.
    /// ### Returns
    /// A deep copy of the Map struct
    /// ### Example
    /// ```
    /// let m = shsc::Map::new();
    /// let m2 = m.clone();
    /// ```

    fn clone(&self) -> Self {
        let newmap = Map::new();
        unsafe {
            let oldstore = self
                .store
                .as_ref()
                .expect("shsc::Map::clone: undefined data store");
            for (key, &index) in self.map.as_ref().unwrap().iter() {
                let value = oldstore
                    .get(index)
                    .expect(&format!("shsc::Map::clone: invalid index {}", index));
                newmap
                    .map
                    .as_mut()
                    .unwrap()
                    .insert(key.clone(), newmap.store.as_ref().unwrap().len());
                newmap.store.as_mut().unwrap().append(value.clone());
            }
        }
        newmap
    }
}

impl traits::RefC for Map {
    fn incrc(&mut self) {
        self.refc += 1;
    }

    fn decrc(&mut self) {
        self.refc -= 1;
        if self.refc < 0 {
            self.refc = 0;
        }
    }

    fn getrc(&self) -> i64 {
        self.refc
    }
}

impl Drop for Map {
    /// Implement the Drop trait for the Map struct.
    /// This allows us to deallocate the Map struct when it goes out of scope.
    /// ### Example
    /// ```
    /// let m = shsc::Map::new();
    /// ```

    fn drop(&mut self) {
        self.decrc();
        if self.getrc() > 0 {
            return;
        }
        unsafe {
            drop(Box::from_raw(self.map));
            drop(Box::from_raw(self.store));
        }
    }
}
