/// Trait for converting an shsc data structure to a string.
/// String representation should preferably be in JSON format.
/// ToStr implementations are hence recursive.
/// ### Example
/// ```
/// use shsc::traits::ToStr;
/// let list = shsc::List::from(vec![ shsc::todata!(1), shsc::todata!(2), shsc::todata!(3), ]);
/// assert_eq!(list.tostr(), "[1,2,3]");
/// ```

pub trait ToStr {
    fn tostr(&self) -> std::string::String;
}
