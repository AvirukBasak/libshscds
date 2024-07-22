use crate::traits::{self, Append, RefCopy};

pub struct String {
    str: crate::List,
}

impl String {
    /// Create a new shsc::String struct.
    /// ### Example
    /// ```
    /// let s = shsc::String::new();
    /// ```

    pub fn new() -> Self {
        crate::String {
            str: crate::List::new(),
        }
    }

    /// Create a new shsc::String struct from a native str type.
    /// ### Arguments
    /// * `value` - A native str type
    /// ### Returns
    /// A new String struct
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// ```

    pub fn from(value: &str) -> Self {
        let mut list = crate::List::new();
        for c in value.chars() {
            list.append(crate::todata!(c));
        }
        crate::String { str: list }
    }

    /// Get the length of the String struct.
    /// ### Returns
    /// The length of the String struct
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// let len = s.len();
    /// ```

    pub fn len(&self) -> usize {
        self.str.len()
    }

    /// Get a vector of references to the data in the shsc::String struct.
    /// ### Returns
    /// A vector of references to the data in the shsc::String struct
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// let vec = s.as_vec();
    /// ```

    pub fn as_vec(&self) -> Vec<&crate::Data> {
        self.str.as_vec()
    }

    /// Get a vector of mutable references to the data in the shsc::String struct.
    /// ### Returns
    /// A vector of mutable references to the data in the shsc::String struct
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// let vec = s.as_vec_mut();
    /// ```

    pub fn as_vec_mut(&mut self) -> Vec<&mut crate::Data> {
        self.str.as_vec_mut()
    }

    /// Concatenate two shsc::String structs.
    /// ### Arguments
    /// * `other` - A shsc::String struct
    /// ### Returns
    /// A new shsc::String struct
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// let s2 = shsc::String::from(" world");
    /// let s3 = s.concat(&s2);
    /// ```

    pub fn concat(&self, other: &crate::String) -> crate::String {
        let mut newstr = crate::String::new();
        newstr.append(self);
        newstr.append(other);
        newstr
    }
}

impl traits::Append<char> for crate::String {
    /// Implement the Append trait for the String struct.
    /// This allows us to append native char types to the String struct.
    /// ### Arguments
    /// * `c` - A native char type
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// s.append(' ');
    /// s.append('w');
    /// s.append('o');
    /// s.append('r');
    /// s.append('l');
    /// s.append('d')
    /// ```

    fn append(&mut self, c: char) {
        self.str.append(crate::todata!(c));
    }
}

impl traits::Append<&crate::String> for crate::String {
    /// Implement the Append trait for the String struct.
    /// This allows us to append shsc::String to itself.
    /// ### Arguments
    /// * `str` - A shsc::String struct
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// let s2 = shsc::String::from(" world");
    /// s.append(&s2);
    /// ```

    fn append(&mut self, str: &crate::String) {
        for item in str.str.as_vec() {
            // clone is acceptable here because data is of type CHAR
            self.str.append(item.clone());
        }
    }
}

impl traits::Append<&str> for crate::String {
    /// Implement the Append trait for the String struct.
    /// This allows us to append native str types to the String struct.
    /// ### Arguments
    /// * `value` - A native str type
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// s.append(" world");
    /// ```

    fn append(&mut self, value: &str) {
        for c in value.chars() {
            self.str.append(crate::todata!(c));
        }
    }
}

impl traits::Insert<char> for crate::String {
    /// Implement the Insert trait for the String struct.
    /// This allows us to insert native char types into the String struct.
    /// ### Arguments
    /// * `index` - The index to insert the value at
    /// * `value` - A native char type
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// s.insert(5, ' ');
    /// s.insert(6, 'w');
    /// s.insert(7, 'o');
    /// s.insert(8, 'r');
    /// s.insert(9, 'l');
    /// s.insert(10, 'd');
    /// ```

    fn insert(&mut self, index: usize, value: char) {
        self.str.insert(index, crate::todata!(value));
    }
}

impl traits::Insert<&crate::String> for crate::String {
    /// Implement the Insert trait for the String struct.
    /// This allows us to insert shsc::String into itself.
    /// ### Arguments
    /// * `index` - The index to insert the value at
    /// * `str` - A shsc::String struct
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// let s2 = shsc::String::from(" world");
    /// s.insert(5, &s2);
    /// ```

    fn insert(&mut self, index: usize, str: &crate::String) {
        for item in str.str.as_vec() {
            // clone is acceptable here because data is of type CHAR
            self.str.insert(index, item.clone());
        }
    }
}

impl traits::Insert<&str> for crate::String {
    /// Implement the Insert trait for the String struct.
    /// This allows us to insert native str types into the String struct.
    /// ### Arguments
    /// * `index` - The index to insert the value at
    /// * `value` - A native str type
    /// ### Example
    /// ```
    /// let mut s = shsc::String::from("hello");
    /// s.insert(5, " world");
    /// ```

    fn insert(&mut self, index: usize, value: &str) {
        for c in value.chars() {
            self.str.insert(index, crate::todata!(c));
        }
    }
}

impl traits::ToStr for String {
    /// Implement the ToStr trait for the String struct.
    /// This allows us to convert the String struct to a native String type.
    /// ### Returns
    /// A native String type
    /// ### Example
    /// ```
    /// use shsc::traits::ToStr;
    /// let s = shsc::String::from("hello");
    /// let str = s.tostr();
    /// assert_eq!(str, "hello");
    /// ```

    fn tostr(&self) -> std::string::String {
        let mut str = std::string::String::new();
        for (index, chr) in self.str.as_vec().iter().enumerate() {
            if let crate::DataTypes::CHAR(c) = chr.data {
                str.push(c);
            } else {
                panic!(
                    "shsc::String: expected type {} at index {}, found type {}",
                    crate::DataTypes::CHAR('\0').typename(),
                    index,
                    chr.data.typename()
                );
            }
        }
        str
    }
}

impl traits::RefCopy for String {
    /// Implement the RefCopy trait for the String struct.
    /// This allows us to create reference counted copies of the String struct.
    /// ### Returns
    /// A reference counted copy of the String struct
    /// ### Example
    /// ```
    /// use shsc::traits::RefCopy;
    /// let mut s = shsc::String::from("hello");
    /// let s2 = s.refcopy();
    /// ```

    fn refcopy(&mut self) -> Self {
        crate::String {
            str: self.str.refcopy(),
        }
    }

    /// Implement the RefCopy trait for the String struct.
    /// This allows us to drop the reference count of the String struct.
    /// String will be deallocated if the reference count reaches zero.
    /// ### Example
    /// ```
    /// use shsc::traits::RefCopy;
    /// let mut s = shsc::String::from("hello");
    /// let s2 = s.refcopy();
    /// s.refdrop();
    /// ```

    fn refdrop(&mut self) {
        // self.str.refdrop();
    }
}

impl Clone for String {
    /// Implement the Clone trait for the String struct.
    /// This allows us to create a deep copy of the String struct.
    /// Resultsin a new String struct with a reference count of 1.
    /// ### Returns
    /// A deep copy of the String struct
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// let s2 = s.clone();
    /// ```

    fn clone(&self) -> Self {
        crate::String {
            str: self.str.clone(),
        }
    }
}

impl traits::RefC for String {
    /// Implement the RefC trait for the String struct.
    /// This allows us to increment the reference count of the String struct.
    fn incrc(&mut self) {
        self.str.incrc();
    }

    /// Implement the RefC trait for the String struct.
    /// This allows us to decrement the reference count of the String struct.
    fn decrc(&mut self) {
        self.str.decrc();
    }

    /// Implement the RefC trait for the String struct.
    /// This allows us to get the reference count of the String struct.
    fn getrc(&self) -> i64 {
        self.str.getrc()
    }
}

impl Drop for String {
    /// Implement the Drop trait for the String struct.
    /// This allows us to deallocate the String struct when the reference count reaches zero.
    /// ### Example
    /// ```
    /// let s = shsc::String::from("hello");
    /// ```

    fn drop(&mut self) {
        self.refdrop();
    }
}
