use crate::{
    traits::{self, RefC, RefCopy},
    util::alloc,
};
use std::{collections::HashMap, ops, ptr};

pub struct Map {
    map: *mut HashMap<String, usize>,
    store: *mut crate::List,
    refc: i64,
}

impl Map {
    pub fn new() -> Self {
        let map = Map {
            map: alloc::allocate::<HashMap<String, usize>>(1),
            store: alloc::allocate::<crate::List>(1),
            refc: 1,
        };
        unsafe {
            ptr::write(map.map, HashMap::new());
            ptr::write(map.store, crate::List::new());
        }
        map
    }

    pub fn from(hashmap: HashMap<String, crate::Data>) -> Self {
        let newmap = Map::new();
        let map = unsafe {
            newmap
                .map
                .as_mut()
                .expect("shsc::Map::from: undefined index map")
        };
        let store = unsafe {
            newmap
                .store
                .as_mut()
                .expect("shsc::Map::from: undefined data store")
        };
        for (key, value) in hashmap {
            let index = store.len();
            map.insert(key, index);
            store.append(value);
        }
        newmap
    }

    pub fn insert(&mut self, key: &str, value: crate::Data) {
        let map = unsafe {
            self.map
                .as_mut()
                .expect("shsc::Map::insert: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_mut()
                .expect("shsc::Map::insert: undefined data store")
        };
        if map.contains_key(key) {
            let index: usize = map[key];
            let storeref = store
                .get_mut(index)
                .expect(&format!("shsc::Map::insert: invalid index {}", index));
            *storeref = value;
        } else {
            let index = store.len();
            map.insert(key.to_owned(), index);
            store.append(value);
        }
    }

    pub fn get(&self, key: &str) -> Option<&crate::Data> {
        let map = unsafe {
            self.map
                .as_ref()
                .expect("shsc::Map::get: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_ref()
                .expect("shsc::Map::get: undefined data store")
        };
        match map.get(key) {
            Some(&index) => Some(
                store
                    .get(index)
                    .expect(&format!("shsc::Map::get: invalid index {}", index)),
            ),
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut crate::Data> {
        let map = unsafe {
            self.map
                .as_mut()
                .expect("shsc::Map::get_mut: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_mut()
                .expect("shsc::Map::get_mut: undefined data store")
        };
        match map.get(key) {
            Some(&index) => Some(
                store
                    .get_mut(index)
                    .expect(&format!("shsc::Map::get_mut: invalid index {}", index)),
            ),
            None => None,
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<crate::Data> {
        let map = unsafe {
            self.map
                .as_mut()
                .expect("shsc::Map::remove: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_mut()
                .expect("shsc::Map::remove: undefined data store")
        };
        match map.remove(key) {
            Some(index) => {
                let tmp = store[index].refcopy();
                store[index] = crate::Data::NULL;
                Some(tmp)
            }
            None => None,
        }
    }
}

impl ops::Index<&str> for Map {
    type Output = crate::Data;

    fn index(&self, key: &str) -> &crate::Data {
        self.get(key)
            .expect(&format!("shsc::Map::index: invalid key {}", key))
    }
}

impl ops::IndexMut<&str> for Map {
    fn index_mut(&mut self, key: &str) -> &mut crate::Data {
        self.get_mut(key)
            .expect(&format!("shsc::Map::index_mut: invalid key {}", key))
    }
}

impl traits::ToStr for Map {
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
        for (key, &index) in map.iter() {
            let value = store
                .get(index)
                .expect(&format!("shsc::Map::tostr: invalid index {}", index));
            result.push_str(&format!("{}: {}", key, value.tostr()));
            if index != store.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push_str("}");
        result
    }
}

impl traits::RefCopy for Map {
    fn refcopy(&mut self) -> Self {
        self.incrc();
        Map {
            map: self.map,
            store: self.store,
            refc: self.refc,
        }
    }

    fn refdrop(&mut self) {
        let map = unsafe {
            self.map
                .as_mut()
                .expect("shsc::Map::refdrop: undefined index map")
        };
        let store = unsafe {
            self.store
                .as_mut()
                .expect("shsc::Map::refdrop: undefined data store")
        };
        self.decrc();
        if self.getrc() == 0 {
            for i in 0..store.len() {
                store[i].refdrop();
            }
            alloc::deallocate(map, 1);
            alloc::deallocate(store, 1);
        }
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        let newmap = Map::new();
        let map = unsafe {
            newmap
                .map
                .as_mut()
                .expect("shsc::Map::clone: undefined index map")
        };
        let store = unsafe {
            newmap
                .store
                .as_mut()
                .expect("shsc::Map::clone: undefined data store")
        };
        let oldmap = unsafe {
            self.map
                .as_ref()
                .expect("shsc::Map::clone: undefined index map")
        };
        let oldstore = unsafe {
            self.store
                .as_ref()
                .expect("shsc::Map::clone: undefined data store")
        };
        for (key, &index) in oldmap.iter() {
            let value = oldstore
                .get(index)
                .expect(&format!("shsc::Map::clone: invalid index {}", index));
            map.insert(key.clone(), store.len());
            store.append(value.clone());
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
    fn drop(&mut self) {
        self.refdrop();
    }
}
