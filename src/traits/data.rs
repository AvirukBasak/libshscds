/// Trait for data structures that can be inserted into.
/// ### Example
/// ```
/// let mut list = shsc::List::from(vec![ shsc::todata!(1), shsc::todata!(2), shsc::todata!(3), ]);
/// list.insert(1, shsc::todata!(4));
/// ```

pub trait Insert<T> {
    fn insert(&mut self, index: usize, value: T);
}

/// Trait for data structures that can be appended to.
/// ### Example
/// ```
/// let mut list = shsc::List::from(vec![ shsc::todata!(1), shsc::todata!(2), shsc::todata!(3), ]);
/// list.append(shsc::todata!(4));
/// ```

pub trait Append<T> {
    fn append(&mut self, value: T);
}
