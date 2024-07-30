use crate::traits::{self, RefC, RefCopy};
use crate::util::alloc;
use std::{ops, ptr};

const DEFAULT_COLS: usize = 32;

pub struct List {
    list: *mut *mut crate::Data,
    len: *mut usize,
    rows: *mut usize,
    cols: *mut usize,
    refc: *mut i64,
}

impl List {
    /// Create a new list
    /// ### Returns
    /// A new list
    /// ### Example
    /// ```
    /// let list = shsc::List::new();
    /// ```

    pub fn new() -> Self {
        let list = List {
            list: ptr::null_mut(),
            len: Box::into_raw(Box::new(0)),
            rows: Box::into_raw(Box::new(0)),
            cols: Box::into_raw(Box::new(DEFAULT_COLS)),
            refc: Box::into_raw(Box::new(1)),
        };
        list
    }

    /// Create a new list from a vector of data
    /// ### Arguments
    /// * `vec` - A vector of data
    /// ### Returns
    /// A new list
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///    shsc::todata!(1),
    ///    shsc::todata!(2),
    ///    shsc::todata!(3),
    /// ]);
    /// ```

    pub fn from(vec: Vec<crate::Data>) -> Self {
        let mut list = List::new();
        for item in vec.into_iter() {
            list.append(item);
        }
        list
    }

    /// Append data to the list
    /// ### Arguments
    /// * `data` - Data to append
    /// ### Example
    /// ```
    /// let mut list = shsc::List::new();
    /// list.append(shsc::todata!(1));
    /// list.append(shsc::todata!(2));
    /// list.append(shsc::todata!(3));
    /// ```

    pub fn append(&mut self, data: crate::Data) {
        unsafe {
            // if list has reached capacity, add new rows
            if (*self.len) >= (*self.rows) * (*self.cols) {
                // reallocate list
                self.list = alloc::reallocate::<*mut crate::Data>(self.list, (*self.rows) + 1);
                *self.rows += 1;
                // allocate new row
                let dest = self.list.add((*self.rows) - 1);
                let newrow = alloc::allocate::<crate::Data>(*self.cols);
                dest.write(newrow);
            }
            // else put data in list[len / rows][len % cols]
            let row = (*self.len) / (*self.cols);
            let col = (*self.len) % (*self.cols);
            // write data to list
            let dest = (*self.list.add(row)).add(col);
            dest.write(data);
            // increment length
            *self.len += 1;
        }
    }

    /// Get data at index
    /// ### Arguments
    /// * `index` - Index of data
    /// ### Returns
    /// Data at index
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///   shsc::todata!(1),
    ///   shsc::todata!(2),
    ///   shsc::todata!(3),
    /// ]);
    /// let data = list.get(1).unwrap();
    /// ```

    pub fn get(&self, index: usize) -> Option<&crate::Data> {
        unsafe {
            if index >= (*self.len) {
                return None;
            }
            let row = index / (*self.cols);
            let col = index % (*self.cols);
            let src = (*self.list.add(row)).add(col);
            Some(&*src)
        }
    }

    /// Get mutable data at index
    /// ### Arguments
    /// * `index` - Index of data
    /// ### Returns
    /// Mutable data at index
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///   shsc::todata!(1),
    ///   shsc::todata!(2),
    ///   shsc::todata!(3),
    /// ]);
    /// let data = list.get_mut(1).unwrap();
    /// ```

    pub fn get_mut(&mut self, index: usize) -> Option<&mut crate::Data> {
        unsafe {
            if index >= (*self.len) {
                return None;
            }
            let row = index / (*self.cols);
            let col = index % (*self.cols);
            let src = (*self.list.add(row)).add(col);
            Some(&mut *src)
        }
    }

    /// Move data from index and replace with shsc::Data::NULL
    /// ### Arguments
    /// * `index` - Index of data
    /// ### Returns
    /// Data at index
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let data = list.take(1).unwrap();
    /// ```

    pub fn take(&self, index: usize) -> Option<crate::Data> {
        unsafe {
            if index >= (*self.len) {
                return None;
            }
            let row = index / (*self.cols);
            let col = index % (*self.cols);
            let src = (*self.list.add(row)).add(col);
            let tmp = (*src).refcopy();
            src.read().refdrop();
            *src = crate::Data::NULL;
            Some(tmp)
        }
    }

    /// Get length of list
    /// ### Returns
    /// Length of list
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///   shsc::todata!(1),
    ///   shsc::todata!(2),
    ///   shsc::todata!(3),
    /// ]);
    /// assert_eq!(list.len(), 3);
    /// ```

    pub fn len(&self) -> usize {
        unsafe { *self.len }
    }

    /// Get list as vector
    /// ### Returns
    /// List as vector
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let vec = list.as_vec();
    /// ```

    pub fn as_vec(&self) -> Vec<&crate::Data> {
        unsafe {
            let mut vec: Vec<&crate::Data> = Vec::new();
            for i in 0..(*self.len) {
                let row = i / (*self.cols);
                let col = i % (*self.cols);
                let src = (*self.list.add(row)).add(col);
                vec.push(&*src);
            }
            vec
        }
    }

    /// Get list as vector of mutable data
    /// ### Returns
    /// List as vector of mutable data
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let vec = list.as_vec_mut();
    /// ```

    pub fn as_vec_mut(&mut self) -> Vec<&mut crate::Data> {
        unsafe {
            let mut vec = Vec::new();
            for i in 0..(*self.len) {
                let row = i / (*self.cols);
                let col = i % (*self.cols);
                let src = (*self.list.add(row)).add(col);
                vec.push(&mut *src);
            }
            vec
        }
    }

    /// Insert data at index
    /// ### Arguments
    /// * `index` - Index to insert data
    /// * `data` - Data to insert
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// list.insert(1, shsc::todata!(4));
    /// ```

    pub fn insert(&mut self, index: usize, data: crate::Data) {
        unsafe {
            if index > (*self.len) {
                panic!("shsc::List: insert: index out of bounds for {}", index);
            }
            // if list has reached capacity, add new rows
            if (*self.len) >= (*self.rows) * (*self.cols) {
                // reallocate list
                self.list = alloc::reallocate::<*mut crate::Data>(self.list, (*self.rows) + 1);
                *self.rows += 1;
                // allocate new row
                let dest = self.list.add((*self.rows) - 1);
                let newrow = alloc::allocate::<crate::Data>(*self.cols);
                dest.write(newrow);
            }
            // shift elements to the right
            for i in (index..(*self.len)).rev() {
                let row = i / (*self.cols);
                let col = i % (*self.cols);
                let row1 = (i + 1) / (*self.cols);
                let col1 = (i + 1) % (*self.cols);
                let src = (*self.list.add(row)).add(col);
                let dest = (*self.list.add(row1)).add(col1);
                *dest = (*src).refcopy();
            }
            // insert data at index
            let row = index / (*self.cols);
            let col = index % (*self.cols);
            let dest = (*self.list.add(row)).add(col);
            dest.write(data);
            // increment length
            *self.len += 1;
        }
    }

    /// Remove data at index
    /// ### Arguments
    /// * `index` - Index to remove data
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// list.remove(1);
    /// ```

    pub fn remove(&mut self, index: usize) -> crate::Data {
        unsafe {
            if index >= (*self.len) {
                panic!("shsc::List: remove: index out of bounds for {}", index);
            }
            let removed =
                (*(*self.list.add(index / (*self.cols))).add(index % (*self.cols))).refcopy();
            // shift elements to the left
            for i in index..(*self.len) - 1 {
                let row = i / (*self.cols);
                let col = i % (*self.cols);
                let row1 = (i + 1) / (*self.cols);
                let col1 = (i + 1) % (*self.cols);
                let src = (*self.list.add(row1)).add(col1);
                let dest = (*self.list.add(row)).add(col);
                dest.read().refdrop();
                *dest = (*src).refcopy();
            }
            // decrement length
            *self.len -= 1;
            removed
        }
    }
}

impl ops::Index<usize> for List {
    type Output = crate::Data;

    /// Implement the Index trait for the List struct.
    /// This allows us to index the List struct.
    /// ### Arguments
    /// * `index` - Index of data
    /// ### Returns
    /// Data at index
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let data = &list[1];
    /// ```

    fn index(&self, index: usize) -> &crate::Data {
        self.get(index)
            .expect(&format!("shsc::List: index: undefined data at {}", index))
    }
}

impl ops::IndexMut<usize> for List {
    /// Implement the IndexMut trait for the List struct.
    /// This allows us to index the List struct and modify the data at the index.
    /// ### Arguments
    /// * `index` - Index of data
    /// ### Returns
    /// Mutable data at index
    /// ### Example
    /// ```
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// list[1] = shsc::todata!(4);
    /// ```

    fn index_mut(&mut self, index: usize) -> &mut crate::Data {
        self.get_mut(index).expect(&format!(
            "shsc::List: index_mut: undefined data at {}",
            index
        ))
    }
}

impl traits::ToStr for List {
    /// Get list as string
    /// ### Returns
    /// List as string
    /// ### Example
    /// ```
    /// use crate::shsc::traits::ToStr;
    /// let list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let s = list.tostr();
    /// ```

    fn tostr(&self) -> std::string::String {
        let mut s = String::new();
        s.push('[');
        for (i, item) in self.as_vec().iter().enumerate() {
            s.push_str(&item.tostr());
            if i < self.len() - 1 {
                s.push(',');
            }
        }
        s.push(']');
        s
    }
}

impl traits::RefCopy for List {
    /// Implement the RefCopy trait for the List struct.
    /// This allows us to create a reference copy of the List struct.
    /// ### Returns
    /// A reference copy of the list
    /// ### Example
    /// ```
    /// use crate::shsc::traits::RefCopy;
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let list1 = list.refcopy();
    /// ```

    fn refcopy(&mut self) -> Self {
        self.incrc();
        List {
            list: self.list,
            len: self.len,
            rows: self.rows,
            cols: self.cols,
            refc: self.refc,
        }
    }

    /// Implement the RefDrop trait for the List struct.
    /// This allows us to drop the List struct.
    /// List will be deallocated if the reference count reaches zero.
    /// Can cause undefined behavior if used without shsc::traits::RefCopy::refcopy.
    /// ### Example
    /// ```
    /// use shsc::traits::RefCopy;
    /// let mut list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// {
    ///     let list2 = list.refcopy();     // new reference copy
    /// }                                   // drop reference copy
    /// list.refdrop();                     // drop original reference copy
    /// ```

    fn refdrop(self) {
        // drop trait is called here
    }
}

impl Clone for List {
    /// Implement the Clone trait for the List struct.
    /// This allows us to create a deep copy of the List struct.
    /// Results in a new List struct with a reference count of 1.
    /// ### Returns
    /// A deep copy of the List struct
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// let list1 = list.clone();
    /// ```

    fn clone(&self) -> Self {
        let mut newlist = List::new();
        for &item in self.as_vec().iter() {
            newlist.append(item.clone());
        }
        newlist
    }
}

impl traits::RefC for List {
    /// Implement the RefC trait for the List struct.
    /// This allows us to increment the reference count of the List struct.
    fn incrc(&mut self) {
        unsafe { *self.refc += 1 };
    }

    /// Implement the RefC trait for the List struct.
    /// This allows us to decrement the reference count of the List struct.
    fn decrc(&mut self) {
        unsafe {
            *self.refc -= 1;
            if (*self.refc) < 0 {
                *self.refc = 0;
            }
        }
    }

    /// Implement the RefC trait for the List struct.
    /// This allows us to get the reference count of the List struct.
    fn getrc(&self) -> i64 {
        unsafe { *self.refc }
    }
}

impl Drop for List {
    /// Implement the Drop trait for the List struct.
    /// This allows us to deallocate the List struct when the reference count reaches zero.
    /// ### Example
    /// ```
    /// let list = shsc::List::from(vec![
    ///     shsc::todata!(1),
    ///     shsc::todata!(2),
    ///     shsc::todata!(3),
    /// ]);
    /// ```

    fn drop(&mut self) {
        unsafe {
            self.decrc();
            if self.getrc() > 0 {
                return;
            }
            for i in 0..(*self.rows) {
                let row = *self.list.add(i);
                for j in 0..(*self.cols) {
                    // drop data in this scope
                    row.add(j).read().refdrop();
                }
                alloc::deallocate::<crate::Data>(row, *self.cols);
            }
            alloc::deallocate::<*mut crate::Data>(self.list, *self.rows);
            drop(Box::from_raw(self.len));
            drop(Box::from_raw(self.rows));
            drop(Box::from_raw(self.cols));
            drop(Box::from_raw(self.refc));
        }
    }
}
