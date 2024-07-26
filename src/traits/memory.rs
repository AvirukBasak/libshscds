/// Traits for heap based data structures that require reference counting.
/// ### Example
/// ```
/// let mut data = shsc::Data::from(
///   shsc::List::from(vec![ shsc::todata!(1), shsc::todata!(2), shsc::todata!(3), ])
/// );
/// data.incrc();
/// data.decrc();
/// ```

pub trait RefC {
    fn incrc(&mut self);
    fn decrc(&mut self);
    fn getrc(&self) -> i64;
}

/// Traits for heap based data structures that require reference counting.
/// Also returns a shallow copy of the data structure with the reference count updated.
/// Is recommended over the RefC trait if the data structure is to be held by multiple variables.
///
/// Also drops the data structure when the reference count reaches zero.
///
/// ### Example
/// ```
/// let mut data = shsc::Data::from(
///   shsc::List::from(vec![ shsc::todata!(1), shsc::todata!(2), shsc::todata!(3), ])
/// );
/// let mut data2 = data.refcopy();
/// data.refdrop();
/// ```

pub trait RefCopy {
    fn refcopy(&mut self) -> Self;
    fn refdrop(self);
}
